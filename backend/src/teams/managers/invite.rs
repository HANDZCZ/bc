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
    macros::{resp_200_Ok_json, resp_400_BadReq_json, resp_500_IntSerErr_json, resp_403_Forbidden_json},
};

#[derive(Serialize, Deserialize)]
struct ManagerToTeam {
    manager_ids: Vec<Uuid>,
    team_id: Uuid,
}

#[post("/invite")]
pub async fn invite(
    pool: Data<PgPool>,
    data: Json<ManagerToTeam>,
    user: LoggedInUser,
) -> impl Responder {
    match query!(
        "call invite_managers_to_team($1, $2, $3)",
        user.id,
        data.team_id,
        &data.manager_ids
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            resp_200_Ok_json!()
        }
        Err(sqlx::Error::Database(error)) => {
            if error.is_foreign_key_violation() {
                let err = crate::common::Error::new("inviting users to team failed - foreign key constraints violation (team_id, manager_id)");
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
    const URI: &str = "/teams/managers/invite";

    #[actix_web::test]
    pub async fn test_ok() {
        let (app, rollbacker, pool) = get_test_app().await;
        let (auth_header, user_id) = new_user_insert_testing(&app).await;
        let team_id = new_team_insert_testing(user_id, &pool).await;
        ok_or_rollback_team!(team_id, rollbacker);
        let (_other_auth_header, other_user_id) = new_user_insert(&app, "testuser-2".into(), "testuser2@test.test".into(), "pass".into()).await;

        let data = ManagerToTeam {
            team_id,
            manager_ids: vec![other_user_id],
        };
        let req = test::TestRequest::post()
            .uri(URI)
            .insert_header(auth_header)
            .set_json(data)
            .to_request();
        let resp = test::call_service(&app, req).await;
        let num_managers = sqlx::query!(
            "select count(*) from managers_to_teams_invites where team_id = $1",
            team_id
        )
        .fetch_one(&pool)
        .await
        .unwrap()
        .count
        .unwrap();

        rollbacker.rollback().await;
        assert_eq!(resp.status().as_u16(), 200);
        assert_eq!(num_managers, 1);
    }

    #[actix_web::test]
    pub async fn test_forbidden() {
        let (app, rollbacker, pool) = get_test_app().await;
        let (_auth_header, user_id) = new_user_insert_testing(&app).await;
        let (other_auth_header, _other_user_id) = new_user_insert(&app, "test2-user".into(), "test2-user@test.test".into(), "pass".into()).await;
        let team_id = new_team_insert_testing(user_id, &pool).await;
        ok_or_rollback_team!(team_id, rollbacker);

        let data = ManagerToTeam {
            team_id,
            manager_ids: vec![user_id],
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
