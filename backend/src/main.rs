use actix_web::{
    middleware::{Compress, Logger, NormalizePath, TrailingSlash},
    web::{self, Data, JsonConfig, PathConfig},
    App, HttpServer,
};
use clap::Parser;

mod brackets;
mod common;
mod config_error_handlers;
mod games;
mod macros;
mod teams;
mod tournaments;

#[derive(Parser, Debug)]
struct Opts {
    #[clap(env)]
    database_url: String,
    #[clap(env)]
    server_address: String,
}

async fn def(req: actix_web::HttpRequest) -> impl actix_web::Responder {
    actix_web::HttpResponse::NotFound().body(format!("{:#?}", req))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().expect(".env file not found");
    let Opts {
        database_url,
        server_address,
    } = Opts::parse();

    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&database_url)
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Compress::default())
            .wrap(NormalizePath::new(TrailingSlash::Trim))
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
            .default_service(web::to(def))
    })
    .bind(server_address)
    .expect("Failed to bind server to address")
    .run()
    .await
}
