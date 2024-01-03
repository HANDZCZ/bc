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
struct Bracket {
    team1: Option<Uuid>,
    team2: Option<Uuid>,
    winner: Option<bool>,
    bracket_tree_id: Uuid,
    layer: u8,
    // TODO: set min to 0
    position: i32,
}

#[derive(Serialize, Deserialize)]
struct ReturningRow {
    id: Uuid,
}

#[post("/new")]
pub async fn new(pool: Data<PgPool>, data: Json<Bracket>) -> impl Responder {
    match query_as!(
        ReturningRow,
        "insert into brackets (team1, team2, winner, bracket_tree_id, layer, position) values ($1, $2, $3, $4, $5, $6)",
        data.team1,
        data.team2,
        data.winner,
        data.bracket_tree_id,
        data.layer as i32,
        data.position
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            resp_200_Ok_json!()
        }
        Err(sqlx::Error::Database(error)) => {
            if error.is_unique_violation() {
                let err = crate::common::Error {
                    error: "request for new bracket violates unique constraints".to_owned(),
                };
                resp_400_BadReq_json!(err)
            } else if error.is_foreign_key_violation() {
                let err = crate::common::Error {
                    error: "request for new bracket violates foreign key constraints (bracket_tree_id, team1, team2)".to_owned(),
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
