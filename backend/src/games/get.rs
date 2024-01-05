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
struct Game {
    name: String,
    description: String,
    version: String,
}

#[get("/{id}")]
pub async fn get(pool: Data<PgPool>, id: web::Path<Uuid>) -> impl Responder {
    match query_as!(Game, "select name, description, version from games where id = $1", id.into_inner())
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(game) => {
            resp_200_Ok_json!(game)
        }
        Err(sqlx::Error::RowNotFound) => {
            let err = crate::common::Error::new("game not found");
            resp_400_BadReq_json!(err)
        }
        Err(_) => {
            resp_500_IntSerErr_json!()
        }
    }
}
