use actix_web::{get, web::Data, Responder};
use sqlx::{query_as, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{macros::{resp_200_Ok_json, resp_500_IntSerErr_json}, common::JsonString};

#[derive(Serialize, Deserialize)]
struct User {
    id: Uuid,
    nick: String,
    email: String,
    roles: JsonString
}

#[get("")]
pub async fn get_all(pool: Data<PgPool>) -> impl Responder {
    match query_as!(User, r#"select id as "id!", nick as "nick!", email as "email!", roles as "roles!: String" from users_and_roles"#)
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(users) => {
            resp_200_Ok_json!(users)
        }
        Err(_) => {
            resp_500_IntSerErr_json!()
        }
    }
}

