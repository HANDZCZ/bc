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
    macros::{resp_200_Ok_json, resp_403_Forbidden_json, resp_400_BadReq_json, resp_500_IntSerErr_json},
};

#[derive(Serialize, Deserialize)]
struct Team {
    id: Uuid,
}

#[derive(Serialize, Deserialize)]
struct RowsAffected {
    rows_affected: u64,
}

#[post("/delete")]
pub async fn delete(pool: Data<PgPool>, data: Json<Team>, user: LoggedInUser) -> impl Responder {
    match query!("call delete_team($1, $2)", user.id, data.id)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => {
            resp_200_Ok_json!()
        }
        Err(sqlx::Error::Database(error)) => {
            if let Some(true) = error.code().map(|c| c == "66666") {
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

#[cfg(test)]
pub mod tests {
    use actix_web::test;

    use super::*;
    use crate::tests::*;
    const URI: &str = "/teams/delete";

    #[actix_web::test]
    pub async fn test_ok() {
        let (app, rollbacker, pool) = get_test_app().await;
        let (auth_header, user_id) = new_user_insert_testing(&app).await;
        let team_id = new_team_insert_testing(user_id, &pool).await;
        ok_or_rollback_team!(team_id, rollbacker);

        let data = Team {
            id: team_id,
        };
        let req = test::TestRequest::post()
            .uri(URI)
            .insert_header(auth_header)
            .set_json(data)
            .to_request();
        let resp = test::call_service(&app, req).await;

        rollbacker.rollback().await;
        let code = resp.status().as_u16();
        let body = test::read_body(resp).await;
        assert_eq!(code, 200, "{}", String::from_utf8(body.to_vec()).unwrap());
    }

    #[actix_web::test]
    pub async fn test_forbidden() {
        let (app, rollbacker, pool) = get_test_app().await;
        let (_auth_header, user_id) = new_user_insert_testing(&app).await;
        let (other_auth_header, _other_user_id) = new_user_insert(
            &app,
            "testing-user2".into(),
            "test2@test2.test".into(),
            "pass".into(),
        )
        .await;
        let team_id = new_team_insert_testing(user_id, &pool).await;
        ok_or_rollback_team!(team_id, rollbacker);

        let data = Team {
            id: team_id,
        };
        let req = test::TestRequest::post()
            .uri(URI)
            .insert_header(other_auth_header)
            .set_json(data)
            .to_request();
        let resp = test::call_service(&app, req).await;

        rollbacker.rollback().await;
        assert_eq!(resp.status().as_u16(), 403);
    }
}
