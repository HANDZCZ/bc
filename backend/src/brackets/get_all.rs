use actix_web::{get, web::{Data, self}, Responder};
use sqlx::{query_as, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::macros::{resp_200_Ok_json, resp_500_IntSerErr_json};

#[derive(Serialize, Deserialize)]
struct Bracket {
    team1: Option<Uuid>,
    team2: Option<Uuid>,
    winner: Option<bool>,
    layer: i32,
    // TODO: set min to 0
    position: i32,
}

#[get("/{id}")]
pub async fn get_all(pool: Data<PgPool>, id: web::Path<Uuid>) -> impl Responder {
    match query_as!(
        Bracket,
        "select team1, team2, winner, layer, position from brackets where bracket_tree_id = $1",
        id.into_inner()
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(brackets) => {
            resp_200_Ok_json!(brackets)
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
    use crate::{tests::*, common::TournamentType};
    const URI: &str = "/brackets";

    #[actix_web::test]
    async fn test_ok() {
        let (app, rollbacker, pool) = get_test_app().await;

        let game_id = new_game_insert(&pool).await;
        ok_or_rollback_game!(game_id, rollbacker);
        let tournament_id = new_tournament_insert_random(game_id, false, false, TournamentType::OneBracketOneFinalPositions, &pool).await;
        ok_or_rollback_tournament!(tournament_id, rollbacker);
        let bracket_tree_id = new_bracket_tree_insert(tournament_id, 0, &pool).await;
        ok_or_rollback_bracket_tree!(bracket_tree_id, rollbacker);

        let req = test::TestRequest::get()
            .uri(&format!("{}/{}", URI, bracket_tree_id))
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_resp_status_eq_or_rollback!(resp, 200, rollbacker);
        let res: Vec<Bracket> = read_body_json(resp).await;
        let res_num = res.len();

        let _bracket_res = new_bracket_insert(bracket_tree_id, 255, 0, &pool).await;
        ok_or_rollback_bracket!(_bracket_res, rollbacker);

        let req = test::TestRequest::get()
            .uri(&format!("{}/{}", URI, bracket_tree_id))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_resp_status_eq_or_rollback!(resp, 200, rollbacker);
        let res: Vec<Bracket> = read_body_json(resp).await;
        rollbacker.rollback().await;
        assert_ne!(res.len(), res_num);
    }
}