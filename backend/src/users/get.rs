use actix_web::{
    get,
    web::{self, Data},
    Responder,
};
use sqlx::{query_as, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{macros::{resp_200_Ok_json, resp_500_IntSerErr_json, resp_400_BadReq_json}, common::JsonString};

#[derive(Serialize, Deserialize)]
struct User {
    nick: String,
    email: String,
    roles: JsonString
}

#[get("/{id}")]
pub async fn get(pool: Data<PgPool>, id: web::Path<Uuid>) -> impl Responder {
    match query_as!(User, r#"select nick as "nick!", email as "email!", roles as "roles!: String" from users_and_roles where id = $1"#, id.into_inner())
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(user) => {
            resp_200_Ok_json!(user)
        }
        Err(sqlx::Error::RowNotFound) => {
            let err = crate::common::Error::new("user not found");
            resp_400_BadReq_json!(err)
        }
        Err(_) => {
            resp_500_IntSerErr_json!()
        }
    }
}

