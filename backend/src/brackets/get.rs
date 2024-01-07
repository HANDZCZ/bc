use actix_web::{
    get,
    web::{self, Data},
    Responder,
};
use sqlx::{query_as, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::macros::{resp_200_Ok_json, resp_500_IntSerErr_json, resp_400_BadReq_json};

#[derive(Serialize, Deserialize)]
struct Bracket {
    bracket_tree_id: Uuid,
    layer: u8,
    // TODO: set min to 0
    position: i32,
}

#[derive(Serialize, Deserialize)]
struct BracketInfo {
    team1: Option<Uuid>,
    team2: Option<Uuid>,
    winner: Option<bool>,
}

#[get("/{bracket_tree_id}/{layer}/{position}")]
pub async fn get(pool: Data<PgPool>, path: web::Path<Bracket>) -> impl Responder {
    match query_as!(
        BracketInfo,
        "select team1, team2, winner from brackets where bracket_tree_id = $1 and layer = $2 and position = $3",
        path.bracket_tree_id,
        path.layer as i32,
        path.position
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(data) => {
            resp_200_Ok_json!(data)
        }
        Err(sqlx::Error::RowNotFound) => {
            let err = crate::common::Error::new("bracket not found");
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

    use crate::{tests::*, common::TournamentType};
    const URI: &str = "/brackets";

    #[actix_web::test]
    async fn test_ok() {
        let (app, rollbacker, pool) = get_test_app().await;

        let game_id = new_game_insert(&pool).await;
        ok_or_rollback_game!(game_id, rollbacker);
        let tournament_id = new_tournament_insert_testing(game_id, false, false, TournamentType::OneBracketOneFinalPositions, &pool).await;
        ok_or_rollback_tournament!(tournament_id, rollbacker);
        let bracket_tree_id = new_bracket_tree_insert(tournament_id, 0, &pool).await;
        ok_or_rollback_bracket_tree!(bracket_tree_id, rollbacker);
        let _bracket_res = new_bracket_insert(bracket_tree_id, 255, 0, &pool).await;
        ok_or_rollback_bracket!(_bracket_res, rollbacker);

        let req = test::TestRequest::get()
            .uri(&format!("{}/{}/{}/{}", URI, bracket_tree_id, 255, 0))
            .to_request();
        let resp = test::call_service(&app, req).await;

        rollbacker.rollback().await;
        assert_eq!(resp.status().as_u16(), 200);
    }
}