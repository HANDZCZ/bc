use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use sqlx::{query, PgPool};

use serde::{Deserialize, Serialize};

use crate::{
    hash_utils::{make_salt, make_hash},
    jwt_stuff::LoggedInUser,
    macros::{resp_200_Ok_json, resp_400_BadReq_json, resp_500_IntSerErr_json},
};

#[derive(Serialize, Deserialize)]
struct User {
    nick: Option<String>,
    password: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct RowsAffected {
    rows_affected: u64,
}

#[post("/edit")]
pub async fn edit(pool: Data<PgPool>, data: Json<User>, user: LoggedInUser) -> impl Responder {
    let (salt, hash) = if let Some(password) = data.password.as_ref() {
        let salt = make_salt();
        let hash = make_hash(password, &salt);
        (Some(salt), Some(hash.to_vec()))
    } else {
        (None, None)
    };

    match query!(
        "update users set nick = coalesce($1, nick), hash = coalesce($2, hash), salt = coalesce($3, salt) where id = $4",
        data.nick,
        hash,
        salt,
        user.id
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
                let err = crate::common::Error::new("request for user edit violates unique constraints");
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
