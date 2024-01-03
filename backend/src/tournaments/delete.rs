use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use sqlx::{query, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::macros::{resp_200_Ok_json, resp_500_IntSerErr_json};

#[derive(Serialize, Deserialize)]
struct Tournament {
    id: Uuid,
}

#[derive(Serialize, Deserialize)]
struct RowsAffected {
    rows_affected: u64,
}

#[post("/delete")]
pub async fn delete(pool: Data<PgPool>, data: Json<Tournament>) -> impl Responder {
    match query!("delete from tournaments where id = $1", data.id)
        .execute(pool.get_ref())
        .await
    {
        Ok(query_result) => {
            let rows_affected = RowsAffected {
                rows_affected: query_result.rows_affected(),
            };
            resp_200_Ok_json!(rows_affected)
        }
        Err(_) => {
            resp_500_IntSerErr_json!()
        }
    }
}
