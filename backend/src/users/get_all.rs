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

#[cfg(test)]
mod tests {
    use actix_web::test::{self, read_body_json};

    use super::*;
    use crate::tests::*;
    const URI: &str = "/users";

    #[actix_web::test]
    async fn test_ok() {
        let (app, rollbacker, _pool) = get_test_app().await;

        let req = test::TestRequest::get()
            .uri(URI)
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_resp_status_eq_or_rollback!(resp, 200, rollbacker);
        let res: Vec<User> = read_body_json(resp).await;
        let res_num = res.len();

        let (_auth_header, _id) = new_user_insert_testing(&app).await;

        let req = test::TestRequest::get()
            .uri(URI)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_resp_status_eq_or_rollback!(resp, 200, rollbacker);
        let res: Vec<User> = read_body_json(resp).await;
        rollbacker.rollback().await;
        assert_ne!(res.len(), res_num);
    }
}