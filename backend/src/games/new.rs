use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use sqlx::{query_as, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::macros::{resp_200_Ok_json, resp_400_BadReq_json, resp_500_IntSerErr_json};

#[derive(Serialize, Deserialize)]
struct Game {
    name: String,
    description: String,
    version: String,
}

#[derive(Serialize, Deserialize)]
struct ReturningRow {
    id: Uuid,
}

#[post("/new")]
pub async fn new(pool: Data<PgPool>, data: Json<Game>) -> impl Responder {
    match query_as!(
        ReturningRow,
        "insert into games (name, description, version) values ($1, $2, $3) returning games.id",
        data.name,
        data.description,
        data.version
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(game_row) => {
            resp_200_Ok_json!(game_row)
        }
        Err(sqlx::Error::Database(error)) => {
            if error.is_unique_violation() {
                let err = crate::common::Error {
                    error: "request for new game violates unique constraints".to_owned(),
                };
                resp_400_BadReq_json!(err)
            } else {
                let err = crate::common::Error {
                    error: format!("unhandled error - {}", error)
                };
                resp_400_BadReq_json!(err)
            }
        }
        Err(_) => {
            resp_500_IntSerErr_json!()
        }
    }
}
