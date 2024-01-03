use actix_web::{
    get,
    web::{self, Data},
    Responder,
};
use sqlx::{query_as, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::macros::{resp_200_Ok_json, resp_500_IntSerErr_json, resp_400_BadReq_json};

#[derive(Serialize, Deserialize)]
struct Bracket {
    bracket_tree_id: Uuid,
    layer: u8,
    // TODO: set min to 0
    position: i32,
}

#[derive(Serialize, Deserialize)]
struct BracketInfo {
    team1: Option<Uuid>,
    team2: Option<Uuid>,
    winner: Option<bool>,
}

#[get("/{bracket_tree_id}/{layer}/{position}")]
pub async fn get(pool: Data<PgPool>, path: web::Path<Bracket>) -> impl Responder {
    match query_as!(
        BracketInfo,
        "select team1, team2, winner from brackets where bracket_tree_id = $1 and layer = $2 and position = $3",
        path.bracket_tree_id,
        path.layer as i32,
        path.position
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(data) => {
            resp_200_Ok_json!(data)
        }
        Err(sqlx::Error::RowNotFound) => {
            let err = crate::common::Error {
                error: "bracket not found".to_owned(),
            };
            resp_400_BadReq_json!(err)
        }
        Err(_) => {
            resp_500_IntSerErr_json!()
        }
    }
}
