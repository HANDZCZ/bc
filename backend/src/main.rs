use actix_web::{
    middleware::{Compress, Logger, NormalizePath, TrailingSlash},
    web::{self, Data, JsonConfig, PathConfig},
    App, HttpServer,
};
use actix_web_grants::GrantsMiddleware;
use clap::Parser;
use jwt_stuff::JwtMiddleware;

mod brackets;
mod common;
mod config_error_handlers;
mod games;
mod hash_utils;
mod jwt_stuff;
mod macros;
mod teams;
#[cfg(test)]
mod tests;
mod tournaments;
mod users;

#[derive(Parser, Debug)]
struct Opts {
    #[clap(env)]
    database_url: String,
    #[clap(env)]
    server_address: String,
    #[clap(env)]
    jwt_secret: String,
    #[clap(env)]
    token_ttl: u64,
}

#[cfg(debug_assertions)]
async fn default_handler(req: actix_web::HttpRequest) -> impl actix_web::Responder {
    actix_web::HttpResponse::NotFound().body(format!("{:#?}", req))
}
#[cfg(not(debug_assertions))]
async fn default_handler(_: actix_web::HttpRequest) -> impl actix_web::Responder {
    actix_web::HttpResponse::NotFound()
        .insert_header(actix_web::http::header::ContentType::json())
        .body("{}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().expect(".env file not found");
    let Opts {
        database_url,
        server_address,
        jwt_secret,
        token_ttl,
    } = Opts::parse();

    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&database_url)
        .await
        .unwrap();

    HttpServer::new(move || {
        let app = App::new()
            .wrap(GrantsMiddleware::with_extractor(jwt_stuff::extract))
            .wrap(JwtMiddleware::new(jwt_secret.clone(), token_ttl))
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .wrap(Logger::default())
            .wrap(Compress::default())
            .app_data(Data::new(pool.clone()))
            .app_data(
                JsonConfig::default()
                    .error_handler(config_error_handlers::json_config_error_handler),
            )
            .app_data(
                PathConfig::default()
                    .error_handler(config_error_handlers::json_config_error_handler),
            )
            .configure(games::configure)
            .configure(tournaments::configure)
            .configure(brackets::configure)
            .configure(teams::configure)
            .configure(users::configure)
            .default_service(web::to(default_handler));

        #[cfg(debug_assertions)]
        {
            app.service(web::resource("/jwt_info").to(jwt_info))
        }

        #[cfg(not(debug_assertions))]
        {
            app
        }
    })
    .bind(server_address)
    .expect("Failed to bind server to address")
    .run()
    .await
}

#[cfg(debug_assertions)]
async fn jwt_info(jwt: jwt_stuff::AuthData) -> impl actix_web::Responder {
    macros::resp_200_Ok_json!(jwt.into_inner().borrow().get_data())
}
