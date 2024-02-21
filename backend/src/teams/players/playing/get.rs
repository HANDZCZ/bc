use actix_web::{
    get,
    web::{self, Data},
    Responder,
};
use sqlx::{query_as, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    common::JsonString,
    macros::{resp_200_Ok_json, resp_400_BadReq_json, resp_500_IntSerErr_json},
};

#[derive(Serialize, Deserialize)]
struct Info {
    players: JsonString,
}

#[derive(Serialize, Deserialize)]
struct ReqData {
    team_id: Uuid,
    tournament_id: Uuid,
}

#[get("/{team_id}/{tournament_id}")]
pub async fn get(pool: Data<PgPool>, data: web::Path<ReqData>) -> impl Responder {
    match query_as!(Info, r#"select players as "players!: String" from teams_tournaments_playing_players where team_id = $1 and tournament_id = $2"#, data.team_id, data.tournament_id)
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(info) => {
            resp_200_Ok_json!(info)
        }
        Err(sqlx::Error::RowNotFound) => {
            let err = crate::common::Error::new("not signed up for this tournament or tournament doesn't exists");
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

    use crate::{common::TournamentType, tests::*};
    const URI: &str = "/teams/players/playing";

    #[actix_web::test]
    async fn test_ok() {
        let (app, rollbacker, pool) = get_test_app().await;
        let game_id = new_game_insert(&pool).await;
        ok_or_rollback_game!(game_id, rollbacker);
        let (_auth_header, user_id) = new_user_insert_random(&app).await;
        let team_id = new_team_insert_random(user_id, &pool).await;
        ok_or_rollback_team!(team_id, rollbacker);
        let tournament_id =
            new_tournament_insert_random(game_id, false, false, TournamentType::FFA, &pool).await;
        ok_or_rollback_tournament!(tournament_id, rollbacker);
        let ins_res = new_team_to_tournament_insert(team_id, tournament_id, &pool).await;
        ok_or_rollback_teams_to_tournament!(ins_res, _ins_res, rollbacker);

        let req = test::TestRequest::get()
            .uri(&format!("{}/{}/{}", URI, team_id, tournament_id))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_resp_status_eq_or_rollback!(resp, 200, rollbacker);
        rollbacker.rollback().await;
    }

    #[actix_web::test]
    async fn test_bar_request() {
        let (app, rollbacker, pool) = get_test_app().await;
        let game_id = new_game_insert(&pool).await;
        ok_or_rollback_game!(game_id, rollbacker);
        let (_auth_header, user_id) = new_user_insert_random(&app).await;
        let team_id = new_team_insert_random(user_id, &pool).await;
        ok_or_rollback_team!(team_id, rollbacker);
        let tournament_id =
            new_tournament_insert_random(game_id, false, false, TournamentType::FFA, &pool).await;
        ok_or_rollback_tournament!(tournament_id, rollbacker);

        let req = test::TestRequest::get()
            .uri(&format!("{}/{}/{}", URI, team_id, tournament_id))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_resp_status_eq_or_rollback!(resp, 400, rollbacker);
        rollbacker.rollback().await;
    }
}
