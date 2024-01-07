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

#[cfg(test)]
mod tests {
    use actix_web::test;

    use super::*;
    use crate::tests::*;
    const URI: &str = "/users";

    #[actix_web::test]
    async fn test_ok() {
        let (app, rollbacker, _pool) = get_test_app().await;
        let (_auth_header, id) = new_user_insert_testing(&app).await;

        let req = test::TestRequest::get()
            .uri(&format!("{}/{}", URI, id))
            .to_request();
        let resp = test::call_service(&app, req).await;
        rollbacker.rollback().await;
        assert_eq!(resp.status().as_u16(), 200);
    }

    #[actix_web::test]
    async fn test_bad_req() {
        let (app, rollbacker, _pool) = get_test_app().await;

        let req = test::TestRequest::get()
            .uri(&format!("{}/{}", URI, Uuid::new_v4()))
            .to_request();
        let resp = test::call_service(&app, req).await;
        rollbacker.rollback().await;
        assert_eq!(resp.status().as_u16(), 400);
    }
}