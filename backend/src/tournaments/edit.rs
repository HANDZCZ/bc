use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use sqlx::{query, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{macros::{resp_200_Ok_json, resp_500_IntSerErr_json, resp_400_BadReq_json, check_user_authority}, jwt_stuff::LoggedInUserWithAuthorities};

#[derive(Serialize, Deserialize)]
struct Tournament {
    id: Uuid,
    name: Option<String>,
    description: Option<String>,
    game_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize)]
struct RowsAffected {
    rows_affected: u64,
}

#[post("/edit")]
pub async fn edit(pool: Data<PgPool>, data: Json<Tournament>, user: LoggedInUserWithAuthorities) -> impl Responder {
    check_user_authority!(user, "role::Tournament Manager");

    match query!(
        "update tournaments set name = coalesce($1, name), description = coalesce($2, description), game_id = coalesce($3, game_id) where id = $4",
        data.name,
        data.description,
        data.game_id,
        data.id
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
                let err = crate::common::Error::new("request for tournament edit violates unique constraints");
                resp_400_BadReq_json!(err)
            } else if error.is_foreign_key_violation() {
                let err = crate::common::Error::new("request for tournament edit violates foreign key constraints (game id doesn't exists)");
                resp_400_BadReq_json!(err)
            } else {
                let err = crate::common::Error::new(format!("unhandled error - {}", error));
                resp_400_BadReq_json!(err)
            }
        }
        Err(_) => {
            resp_500_IntSerErr_json!()
        }
    }
}
