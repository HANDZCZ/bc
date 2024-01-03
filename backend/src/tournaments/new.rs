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
struct Tournament {
    name: String,
    description: String,
    game_id: Uuid,
}

#[derive(Serialize, Deserialize)]
struct ReturningRow {
    id: Uuid,
}

#[post("/new")]
pub async fn new(pool: Data<PgPool>, data: Json<Tournament>) -> impl Responder {
    match query_as!(
        ReturningRow,
        "insert into tournaments (name, description, game_id) values ($1, $2, $3) returning tournaments.id",
        data.name,
        data.description,
        data.game_id
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
                    error: "request for new tournament violates unique constraints".to_owned(),
                };
                resp_400_BadReq_json!(err)
            } else if error.is_foreign_key_violation() {
                let err = crate::common::Error {
                    error: "request for new tournament violates foreign key constraints (game id doesn't exists)".to_owned(),
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
