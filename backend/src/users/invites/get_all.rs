use actix_web::{get, web::Data, Responder};
use sqlx::{query_as, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    common::JsonString,
    macros::{resp_200_Ok_json, resp_400_BadReq_json, resp_500_IntSerErr_json},
};

#[derive(Serialize, Deserialize)]
struct User {
    id: Uuid,
    player_invites: JsonString,
    manager_invites: JsonString,
}

#[get("")]
pub async fn get_all(pool: Data<PgPool>) -> impl Responder {
    match query_as!(
        User,
        r#"select id as "id!", player_invites as "player_invites!: String", manager_invites as "manager_invites!: String"
        from user_invites"#,
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(users) => {
            resp_200_Ok_json!(users)
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
