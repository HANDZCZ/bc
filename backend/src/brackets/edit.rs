use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use sqlx::{query, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::macros::{resp_200_Ok_json, resp_500_IntSerErr_json, resp_400_BadReq_json};

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

#[derive(Serialize, Deserialize)]
struct RowsAffected {
    rows_affected: u64,
}

#[post("/edit")]
pub async fn edit(pool: Data<PgPool>, data: Json<Bracket>) -> impl Responder {
    match query!(
        "update brackets set team1 = $1, team2 = $2, winner = $3 where bracket_tree_id = $4 and layer = $5 and position = $6",
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
        Ok(query_result) => {
            let rows_affected = RowsAffected {
                rows_affected: query_result.rows_affected(),
            };
            resp_200_Ok_json!(rows_affected)
        }
        Err(sqlx::Error::Database(error)) => {
            if error.is_unique_violation() {
                let err = crate::common::Error {
                    error: "request for bracket edit violates unique constraints".to_owned(),
                };
                resp_400_BadReq_json!(err)
            } else if error.is_foreign_key_violation() {
                let err = crate::common::Error {
                    error: "request for bracket edit violates foreign key constraints (bracket_tree_id, team1, team2)".to_owned(),
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
