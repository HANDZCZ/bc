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
struct ReqData {
    tournament_id: Uuid,
    team_id: Uuid,
}

#[post("/apply_for")]
pub async fn apply_for(
    pool: Data<PgPool>,
    data: Json<ReqData>,
    user: LoggedInUser,
) -> impl Responder {
    match query!(
        "call apply_for_tournament($1, $2, $3)",
        user.id,
        data.team_id,
        data.tournament_id
    )
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
mod tests {
    use actix_web::test;

    use super::*;
    use crate::{common::TournamentType, tests::*};
    const URI: &str = "/tournaments/apply_for";

    #[actix_web::test]
    async fn test_ok() {
        let (app, rollbacker, pool) = get_test_app().await;
        let (auth_header, user_id) = new_user_insert_random(&app).await;
        let game_id = new_game_insert(&pool).await;
        ok_or_rollback_game!(game_id, rollbacker);
        let team_id = new_team_insert_random(user_id, &pool).await;
        ok_or_rollback_team!(team_id, rollbacker);
        let tournament_id =
            new_tournament_insert_random(game_id, false, false, TournamentType::FFA, &pool).await;
        ok_or_rollback_tournament!(tournament_id, rollbacker);

        let data = ReqData {
            team_id,
            tournament_id,
        };

        let req = test::TestRequest::post()
            .uri(URI)
            .insert_header(auth_header)
            .set_json(data)
            .to_request();
        let resp = test::call_service(&app, req).await;
        let num_teams = sqlx::query!(
            "select count(*) from teams_to_tournaments where team_id = $1 and tournament_id = $2",
            team_id,
            tournament_id
        )
        .fetch_one(&pool)
        .await
        .unwrap()
        .count
        .unwrap();

        rollbacker.rollback().await;
        assert_eq!(resp.status().as_u16(), 200);
        assert_eq!(num_teams, 1);
    }

    #[actix_web::test]
    async fn test_not_team_manager() {
        let (app, rollbacker, pool) = get_test_app().await;
        let (_auth_header, user_id) = new_user_insert_random(&app).await;
        let (other_auth_header, _other_user_id) = new_user_insert_random(&app).await;
        let game_id = new_game_insert(&pool).await;
        ok_or_rollback_game!(game_id, rollbacker);
        let team_id = new_team_insert_random(user_id, &pool).await;
        ok_or_rollback_team!(team_id, rollbacker);
        let tournament_id =
            new_tournament_insert_random(game_id, false, false, TournamentType::FFA, &pool).await;
        ok_or_rollback_tournament!(tournament_id, rollbacker);

        let data = ReqData {
            team_id,
            tournament_id,
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

    #[actix_web::test]
    async fn test_applications_closed() {
        let (app, rollbacker, pool) = get_test_app().await;
        let (auth_header, user_id) = new_user_insert_random(&app).await;
        let game_id = new_game_insert(&pool).await;
        ok_or_rollback_game!(game_id, rollbacker);
        let team_id = new_team_insert_random(user_id, &pool).await;
        ok_or_rollback_team!(team_id, rollbacker);
        let tournament_id =
            new_tournament_insert_random(game_id, false, true, TournamentType::FFA, &pool).await;
        ok_or_rollback_tournament!(tournament_id, rollbacker);

        let data = ReqData {
            team_id,
            tournament_id,
        };

        let req = test::TestRequest::post()
            .uri(URI)
            .insert_header(auth_header)
            .set_json(data)
            .to_request();
        let resp = test::call_service(&app, req).await;

        rollbacker.rollback().await;
        assert_eq!(resp.status().as_u16(), 403);
    }
}
