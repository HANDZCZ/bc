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
    macros::{resp_200_Ok_json, resp_400_BadReq_json, resp_500_IntSerErr_json},
};

#[derive(Serialize, Deserialize)]
struct Invite {
    team_id: Uuid,
    accepted: bool,
}

#[post("/handle_invite")]
pub async fn handle_invite(
    pool: Data<PgPool>,
    data: Json<Invite>,
    user: LoggedInUser,
) -> impl Responder {
    match query!(
        "call handle_player_invite($1, $2, $3)",
        user.id,
        data.team_id,
        data.accepted
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            resp_200_Ok_json!()
        }
        Err(sqlx::Error::Database(error)) => {
            if let Some(true) = error.code().map(|c| c == "44444") {
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
    const URI: &str = "/teams/players/handle_invite";

    #[actix_web::test]
    pub async fn test_ok_accept() {
        let (app, rollbacker, pool) = get_test_app().await;
        let (auth_header, user_id) = new_user_insert_random(&app).await;
        let team_id = new_team_insert_random(user_id, &pool).await;
        ok_or_rollback_team!(team_id, rollbacker);
        let ins_res = new_player_to_team_invite_insert(user_id, team_id, &pool).await;
        ok_or_rollback_player_to_team_invite!(ins_res, _ins_res, rollbacker);

        let data = Invite {
            team_id,
            accepted: true
        };
        let req = test::TestRequest::post()
            .uri(URI)
            .insert_header(auth_header)
            .set_json(data)
            .to_request();
        let resp = test::call_service(&app, req).await;
        let num_players = sqlx::query!(
            "select count(*) from players_to_teams where team_id = $1",
            team_id
        )
        .fetch_one(&pool)
        .await
        .unwrap()
        .count
        .unwrap();
        let num_player_invites = sqlx::query!(
            "select count(*) from players_to_teams_invites where team_id = $1",
            team_id
        )
        .fetch_one(&pool)
        .await
        .unwrap()
        .count
        .unwrap();

        rollbacker.rollback().await;
        assert_eq!(resp.status().as_u16(), 200);
        assert_eq!(num_players, 1);
        assert_eq!(num_player_invites, 0);
    }

    #[actix_web::test]
    pub async fn test_ok_reject() {
        let (app, rollbacker, pool) = get_test_app().await;
        let (auth_header, user_id) = new_user_insert_random(&app).await;
        let team_id = new_team_insert_random(user_id, &pool).await;
        ok_or_rollback_team!(team_id, rollbacker);
        let ins_res = new_player_to_team_invite_insert(user_id, team_id, &pool).await;
        ok_or_rollback_player_to_team_invite!(ins_res, _ins_res, rollbacker);

        let data = Invite {
            team_id,
            accepted: false
        };
        let req = test::TestRequest::post()
            .uri(URI)
            .insert_header(auth_header)
            .set_json(data)
            .to_request();
        let resp = test::call_service(&app, req).await;
        let num_players = sqlx::query!(
            "select count(*) from players_to_teams where team_id = $1",
            team_id
        )
        .fetch_one(&pool)
        .await
        .unwrap()
        .count
        .unwrap();
        let num_player_invites = sqlx::query!(
            "select count(*) from players_to_teams_invites where team_id = $1",
            team_id
        )
        .fetch_one(&pool)
        .await
        .unwrap()
        .count
        .unwrap();

        rollbacker.rollback().await;
        assert_eq!(resp.status().as_u16(), 200);
        assert_eq!(num_players, 0);
        assert_eq!(num_player_invites, 0);
    }

    #[actix_web::test]
    pub async fn test_no_invite() {
        let (app, rollbacker, pool) = get_test_app().await;
        let (auth_header, user_id) = new_user_insert_random(&app).await;
        let team_id = new_team_insert_random(user_id, &pool).await;
        ok_or_rollback_team!(team_id, rollbacker);

        let data = Invite {
            team_id,
            accepted: true
        };
        let req = test::TestRequest::post()
            .uri(URI)
            .insert_header(auth_header)
            .set_json(data)
            .to_request();
        let resp = test::call_service(&app, req).await;

        rollbacker.rollback().await;
        assert_eq!(resp.status().as_u16(), 400);
    }
}
