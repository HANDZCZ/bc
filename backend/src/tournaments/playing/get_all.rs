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
    macros::{resp_200_Ok_json, resp_500_IntSerErr_json},
};

#[derive(Serialize, Deserialize)]
struct Info {
    team_id: Uuid,
    team_name: String,
    players: JsonString,
}

#[get("/{id}")]
pub async fn get_all(pool: Data<PgPool>, id: web::Path<Uuid>) -> impl Responder {
    match query_as!(Info, r#"select team_name as "team_name!", team_id as "team_id!", players as "players!: String" from teams_tournaments_playing_players where tournament_id = $1"#, id.into_inner())
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(infos) => {
            resp_200_Ok_json!(infos)
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
    use crate::{common::TournamentType, tests::*};
    const URI: &str = "/tournaments/playing";

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

        let req = test::TestRequest::get()
            .uri(&format!("{}/{}", URI, tournament_id))
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_resp_status_eq_or_rollback!(resp, 200, rollbacker);
        let res: Vec<Info> = read_body_json(resp).await;
        let res_num = res.len();

        let ins_res = new_team_to_tournament_insert(team_id, tournament_id, &pool).await;
        ok_or_rollback_teams_to_tournament!(ins_res, _ins_res, rollbacker);

        let req = test::TestRequest::get()
            .uri(&format!("{}/{}", URI, tournament_id))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_resp_status_eq_or_rollback!(resp, 200, rollbacker);
        let res: Vec<Info> = read_body_json(resp).await;
        rollbacker.rollback().await;
        assert_ne!(res.len(), res_num);
    }
}
