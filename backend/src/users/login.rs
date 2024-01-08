use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use sqlx::{query_as, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    hash_utils::verify_password,
    jwt_stuff::{AuthData, UserData},
    macros::{
        resp_200_Ok_json, resp_400_BadReq_json, resp_403_Forbidden_json, resp_500_IntSerErr_json,
    },
};

#[derive(Deserialize, Serialize)]
pub struct LoginData {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct User {
    id: Uuid,
    salt: String,
    hash: Vec<u8>,
}

#[post("/login")]
pub async fn login(
    pool: Data<PgPool>,
    data: Json<LoginData>,
    auth_data: AuthData,
) -> impl Responder {
    let auth_data = auth_data.into_inner();
    {
        if auth_data.borrow().get_data().is_some() {
            let err = crate::common::Error::new("already logged in");
            return resp_403_Forbidden_json!(err);
        }
    }

    match query_as!(
        User,
        "select salt, hash, id from users where email = $1",
        data.email.to_lowercase()
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(row) => {
            if verify_password(
                row.hash.as_slice().try_into().unwrap(),
                &row.salt,
                &data.password,
            ) {
                *auth_data.borrow_mut().get_data_mut() = Some(UserData::new(row.id));
                resp_200_Ok_json!()
            } else {
                let err = crate::common::Error::new("wrong credentials");
                resp_400_BadReq_json!(err)
            }
        }
        Err(sqlx::Error::RowNotFound) => {
            let err = crate::common::Error::new("user doesn't exist");
            resp_400_BadReq_json!(err)
        }
        Err(sqlx::Error::Database(error)) => {
            let err = crate::common::Error::new(format!("unhandled error - {}", error));
            resp_400_BadReq_json!(err)
        }
        Err(_) => {
            resp_500_IntSerErr_json!()
        }
    }
}

#[cfg(test)]
pub mod tests {
    use actix_web::test;

    use super::*;
    use crate::tests::*;
    const URI: &str = "/users/login";

    #[actix_web::test]
    pub async fn test_ok() {
        let (app, rollbacker, _pool) = get_test_app().await;
        let (_reg_user_header, _user_id) = new_user_insert(
            &app,
            "test-user-login".into(),
            "testing-regular-user@test.test".into(),
            "pass".into(),
        )
        .await;
        let data = LoginData {
            email: "testing-regular-user@test.test".into(),
            password: "pass".into(),
        };

        let req = test::TestRequest::post()
            .uri(URI)
            .set_json(data)
            .to_request();
        let resp = test::call_service(&app, req).await;

        rollbacker.rollback().await;
        assert_eq!(resp.status().as_u16(), 200);
    }
}
