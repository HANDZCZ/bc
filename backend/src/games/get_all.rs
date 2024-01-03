use actix_web::{get, web::Data, Responder};
use sqlx::{query_as, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::macros::{resp_200_Ok_json, resp_500_IntSerErr_json};

#[derive(Serialize, Deserialize)]
struct Game {
    id: Uuid,
    name: String,
    description: String,
    version: String,
}

#[get("")]
pub async fn get_all(pool: Data<PgPool>) -> impl Responder {
    match query_as!(Game, "select * from games")
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(games) => {
            resp_200_Ok_json!(games)
        }
        Err(_) => {
            resp_500_IntSerErr_json!()
        }
    }
}
