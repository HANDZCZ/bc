use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use fancy_regex::Regex;
use once_cell::sync::Lazy;
use sqlx::{query_as, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    hash_utils::{make_hash, make_salt},
    jwt_stuff::{AuthData, UserData},
    macros::{
        resp_200_Ok_json, resp_400_BadReq_json, resp_403_Forbidden_json, resp_500_IntSerErr_json,
    },
};

#[derive(Deserialize, Serialize)]
pub struct RegisterData {
    nick: String,
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct ReturningRow {
    id: Uuid,
}

static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?=^[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?)(.{5,320}$)"#).unwrap()
});

#[post("/register")]
pub async fn register(
    pool: Data<PgPool>,
    data: Json<RegisterData>,
    auth_data: AuthData,
) -> impl Responder {
    let auth_data = auth_data.into_inner();
    {
        if auth_data.borrow().get_data().is_some() {
            let err = crate::common::Error::new("already logged in");
            return resp_403_Forbidden_json!(err);
        }
    }

    if !EMAIL_REGEX.is_match(&data.email).unwrap() {
        let err = crate::common::Error::new("email is not valid");
        return resp_400_BadReq_json!(err);
    }

    let salt = make_salt();
    let hash = make_hash(&data.password, &salt);

    match query_as!(
        ReturningRow,
        "insert into users (nick, hash, salt, email) values ($1, $2, $3, $4) returning users.id",
        data.nick,
        hash.to_vec(),
        salt,
        data.email.to_lowercase()
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(row) => {
            *auth_data.borrow_mut().get_data_mut() = Some(UserData::new(row.id));

            resp_200_Ok_json!(row)
        }
        Err(sqlx::Error::Database(error)) => {
            if error.is_unique_violation() {
                let err = crate::common::Error::new("register request violates unique constraints");
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

#[cfg(test)]
pub mod tests {
    use crate::tests::*;

    #[actix_web::test]
    pub async fn test_ok() {
        let (app, rollbacker, _pool) = get_test_app().await;
        let _reg_user_header = get_regular_users_auth_header(&app).await;

        rollbacker.rollback().await;
    }
}
