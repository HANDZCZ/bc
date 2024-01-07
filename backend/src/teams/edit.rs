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
    macros::{
        resp_200_Ok_json, resp_400_BadReq_json, resp_403_Forbidden_json, resp_500_IntSerErr_json,
    },
};

#[derive(Serialize, Deserialize)]
struct Team {
    id: Uuid,
    name: Option<String>,
    description: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct RowsAffected {
    rows_affected: u64,
}

#[post("/edit")]
pub async fn edit(pool: Data<PgPool>, data: Json<Team>, user: LoggedInUser) -> impl Responder {
    match query!(
        "call edit_team($1, $2, $3, $4)",
        user.id,
        data.name,
        data.description,
        data.id
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => resp_200_Ok_json!(),
        Err(sqlx::Error::Database(error)) => {
            if error.is_unique_violation() {
                let err =
                    crate::common::Error::new("request for team edit violates unique constraints");
                resp_400_BadReq_json!(err)
            } else if let Some(true) = error.code().map(|c| c == "66666") {
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
    const URI: &str = "/teams/edit";

    #[actix_web::test]
    pub async fn test_ok() {
        let (app, rollbacker, pool) = get_test_app().await;
        let (auth_header, user_id) = new_user_insert_testing(&app).await;
        let team_id = new_team_insert_testing(user_id, &pool).await;
        ok_or_rollback_team!(team_id, rollbacker);

        let data = Team {
            id: team_id,
            name: None,
            description: None,
        };
        let req = test::TestRequest::post()
            .uri(URI)
            .insert_header(auth_header)
            .set_json(data)
            .to_request();
        let resp = test::call_service(&app, req).await;

        rollbacker.rollback().await;
        assert_eq!(resp.status().as_u16(), 200);
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
            name: None,
            description: None,
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
