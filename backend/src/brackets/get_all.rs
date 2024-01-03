use actix_web::{get, web::{Data, self}, Responder};
use sqlx::{query_as, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::macros::{resp_200_Ok_json, resp_500_IntSerErr_json};

#[derive(Serialize, Deserialize)]
struct Bracket {
    team1: Option<Uuid>,
    team2: Option<Uuid>,
    winner: Option<bool>,
    layer: i32,
    // TODO: set min to 0
    position: i32,
}

#[get("/{id}")]
pub async fn get_all(pool: Data<PgPool>, id: web::Path<Uuid>) -> impl Responder {
    match query_as!(
        Bracket,
        "select team1, team2, winner, layer, position from brackets where bracket_tree_id = $1",
        id.into_inner()
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(brackets) => {
            resp_200_Ok_json!(brackets)
        }
        Err(_) => {
            resp_500_IntSerErr_json!()
        }
    }
}
