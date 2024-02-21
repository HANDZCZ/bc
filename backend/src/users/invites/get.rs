use actix_web::{
    get,
    web::{self, Data},
    Responder,
};
use sqlx::{query_as, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    common::JsonString,
    macros::{resp_200_Ok_json, resp_400_BadReq_json, resp_500_IntSerErr_json},
};

#[derive(Serialize, Deserialize)]
struct User {
    player_invites: JsonString,
    manager_invites: JsonString,
}

#[get("/{id}")]
pub async fn get(pool: Data<PgPool>, id: web::Path<Uuid>) -> impl Responder {
    match query_as!(
        User,
        r#"select player_invites as "player_invites!: String", manager_invites as "manager_invites!: String"
        from user_invites
        where id = $1"#,
        id.into_inner()
    )
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
