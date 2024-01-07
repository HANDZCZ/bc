use actix_http::{
    body::{BoxBody, EitherBody},
    header::TryIntoHeaderPair,
    Request,
};
use actix_service::Service;
use actix_web::{
    dev::ServiceResponse,
    http::header,
    middleware::{NormalizePath, TrailingSlash},
    test::{self, read_body_json},
    web::{Data, JsonConfig, PathConfig},
    App,
};
use actix_web_grants::GrantsMiddleware;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    common::TournamentType,
    config_error_handlers,
    jwt_stuff::{self, JwtMiddleware},
};

const JWT_SECRET: &str = "secret";
const TOKEN_TTL: u64 = 25 * 365 * 24 * 60 * 60;

pub struct Rollbacker(PgPool);

impl Rollbacker {
    pub async fn rollback(self) {
        sqlx::query("ROLLBACK")
            .execute(&self.0)
            .await
            .expect("ROLLBACK failed");
    }
}

pub async fn get_test_app() -> (
    impl Service<Request, Response = ServiceResponse<EitherBody<BoxBody>>, Error = actix_web::Error>,
    Rollbacker,
    PgPool,
) {
    dotenvy::dotenv().unwrap();

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    let data_pool = Data::new(pool.clone());

    sqlx::query("BEGIN")
        .execute(&pool)
        .await
        .expect("BEGIN failed");

    (
        test::init_service(
            App::new()
                .wrap(GrantsMiddleware::with_extractor(jwt_stuff::extract))
                .wrap(JwtMiddleware::new(JWT_SECRET.into(), TOKEN_TTL))
                .wrap(NormalizePath::new(TrailingSlash::Trim))
                .app_data(data_pool)
                .app_data(
                    JsonConfig::default()
                        .error_handler(config_error_handlers::json_config_error_handler),
                )
                .app_data(
                    PathConfig::default()
                        .error_handler(config_error_handlers::json_config_error_handler),
                )
                .configure(crate::games::configure)
                .configure(crate::tournaments::configure)
                .configure(crate::brackets::configure)
                .configure(crate::teams::configure)
                .configure(crate::users::configure),
        )
        .await,
        Rollbacker(pool.clone()),
        pool,
    )
}

pub async fn new_user_insert_testing(
    app: &impl Service<
        Request,
        Response = ServiceResponse<EitherBody<BoxBody>>,
        Error = actix_web::Error,
    >,
) -> (impl TryIntoHeaderPair, Uuid) {
    new_user_insert(
        app,
        "testing-regular-user".into(),
        "testing-regular-user@test.test".into(),
        "pass".into(),
    )
    .await
}
pub async fn new_user_insert(
    app: &impl Service<
        Request,
        Response = ServiceResponse<EitherBody<BoxBody>>,
        Error = actix_web::Error,
    >,
    nick: String,
    email: String,
    password: String,
) -> (impl TryIntoHeaderPair, Uuid) {
    #[derive(Deserialize, Serialize)]
    pub struct ReqData {
        nick: String,
        password: String,
        email: String,
    }
    let req = test::TestRequest::post()
        .uri("/users/register")
        .insert_header(actix_web::http::header::ContentType::json())
        .set_json(ReqData {
            nick,
            email,
            password,
        })
        .to_request();

    let resp = test::call_service(app, req).await;
    assert!(resp.status().is_success());
    #[derive(Deserialize, Serialize)]
    struct ResBody {
        id: Uuid,
    }
    (
        (
            header::AUTHORIZATION,
            resp.headers()
                .get(header::AUTHORIZATION)
                .unwrap()
                .to_owned(),
        ),
        read_body_json::<ResBody, _>(resp).await.id,
    )
}

pub async fn get_regular_users_auth_header(
    app: &impl Service<
        Request,
        Response = ServiceResponse<EitherBody<BoxBody>>,
        Error = actix_web::Error,
    >,
) -> impl TryIntoHeaderPair {
    new_user_insert_testing(app).await.0
}

pub async fn get_tournament_managers_auth_header(
    app: &impl Service<
        Request,
        Response = ServiceResponse<EitherBody<BoxBody>>,
        Error = actix_web::Error,
    >,
) -> impl TryIntoHeaderPair {
    #[derive(Deserialize, Serialize)]
    pub struct ReqData {
        email: String,
        password: String,
    }
    let req = test::TestRequest::post()
        .uri("/users/login")
        .insert_header(actix_web::http::header::ContentType::json())
        .set_json(ReqData {
            email: "testing-tournament-manager@test.test".into(),
            password: "pass".into(),
        })
        .to_request();

    let resp = test::call_service(app, req).await;
    assert!(resp.status().is_success());
    (
        header::AUTHORIZATION,
        resp.headers()
            .get(header::AUTHORIZATION)
            .unwrap()
            .to_owned(),
    )
}

pub async fn new_game_insert(pool: &PgPool) -> Result<Uuid, sqlx::Error> {
    Ok(sqlx::query!(
        "insert into games (name, description) values ($1, $2) returning games.id",
        "test-game",
        "test-game"
    )
    .fetch_one(pool)
    .await?
    .id)
}

pub async fn new_tournament_insert(
    game_id: Uuid,
    requires_application: bool,
    applications_closed: bool,
    tournament_type: TournamentType,
    pool: &PgPool,
) -> Result<Uuid, sqlx::Error> {
    Ok(sqlx::query!(
        r#"insert into tournaments (name, description, game_id, max_team_size, requires_application, applications_closed, tournament_type)
        values ($1, $2, $3, $4, $5, $6, $7) returning tournaments.id"#,
        "test-tournament",
        "test-tournament",
        game_id,
        5,
        requires_application,
        applications_closed,
        tournament_type as TournamentType
    )
    .fetch_one(pool)
    .await?
    .id)
}

pub async fn new_bracket_tree_insert(
    tournament_id: Uuid,
    position: i32,
    pool: &PgPool,
) -> Result<Uuid, sqlx::Error> {
    Ok(sqlx::query!(
        r#"insert into bracket_trees (tournament_id, position)
        values ($1, $2) returning bracket_trees.id"#,
        tournament_id,
        position
    )
    .fetch_one(pool)
    .await?
    .id)
}

pub async fn new_bracket_insert(
    bracket_tree_id: Uuid,
    layer: i32,
    position: i32,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"insert into brackets (team1, team2, winner, bracket_tree_id, layer, position) values ($1, $2, $3, $4, $5, $6)"#,
        Option::<Uuid>::None,
        Option::<Uuid>::None,
        Option::<bool>::None,
        bracket_tree_id,
        layer,
        position
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[macro_export]
macro_rules! ok_or_rollback_macro {
    ($field:ident, $new_field:ident, $rollbacker:ident, $error:expr) => {
        let Ok($new_field) = $field else {
            $rollbacker.rollback().await;
            panic!($error);
        };
    };
    ($field:ident, $rollbacker:ident, $error:expr) => {
        ok_or_rollback_macro!($field, $field, $rollbacker, $error);
    };
}
#[allow(unused_imports)]
pub use ok_or_rollback_macro as ok_or_rollback;

use paste::paste;
macro_rules! build_ok_or_rollback {
    ($suffix:ident, $error:expr) => {
        paste! {
            #[macro_export]
            macro_rules! [<ok_or_rollback_ $suffix _macro>] {
                ($field:ident, $new_field:ident, $rollbacker:ident) => {
                    crate::tests::ok_or_rollback!($field, $new_field, $rollbacker, $error);
                };
                ($field:ident, $rollbacker:ident) => {
                    crate::tests::ok_or_rollback!($field, $field, $rollbacker, $error);
                };
            }
            #[allow(unused_imports)]
            pub use [<ok_or_rollback_ $suffix _macro>] as [<ok_or_rollback_ $suffix>];
        }
    };
}

build_ok_or_rollback!(game, "game insert failed");
build_ok_or_rollback!(tournament, "tournament insert failed");
build_ok_or_rollback!(bracket_tree, "bracket_tree insert failed");
build_ok_or_rollback!(bracket, "bracket insert failed");

#[macro_export]
macro_rules! assert_resp_status_eq_or_rollback_macro {
    ($response:ident, $status:expr, $rollbacker:ident) => {
        if $response.status().as_u16() != $status {
            $rollbacker.rollback().await;
            assert_eq!($response.status().as_u16(), $status);
            return;
        }
    };
}
#[allow(unused_imports)]
pub use assert_resp_status_eq_or_rollback_macro as assert_resp_status_eq_or_rollback;
