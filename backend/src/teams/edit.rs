use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use sqlx::{query, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    jwt_stuff::LoggedInUser,
    macros::{
        resp_200_Ok_json, resp_400_BadReq_json, resp_403_Forbidden_json, resp_500_IntSerErr_json,
    },
};

#[derive(Serialize, Deserialize)]
struct Team {
    id: Uuid,
    name: Option<String>,
    description: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct RowsAffected {
    rows_affected: u64,
}

#[post("/edit")]
pub async fn edit(pool: Data<PgPool>, data: Json<Team>, user: LoggedInUser) -> impl Responder {
    match query!(
        "call edit_team($1, $2, $3, $4)",
        user.id,
        data.name,
        data.description,
        data.id
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => resp_200_Ok_json!(),
        Err(sqlx::Error::Database(error)) => {
            if error.is_unique_violation() {
                let err = crate::common::Error::new("request for team edit violates unique constraints");
                resp_400_BadReq_json!(err)
            } else if let Some(true) = error.code().map(|c| c == "66666") {
                let err = crate::common::Error::new(error.message());
                resp_403_Forbidden_json!(err)
            } else if let Some(true) = error.code().map(|c| c == "44444") {
                let err = crate::common::Error::new(error.message());
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
