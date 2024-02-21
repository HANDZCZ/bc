use actix_web::{get, web::Data, Responder};
use sqlx::{query_as, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    common::JsonString,
    macros::{resp_200_Ok_json, resp_500_IntSerErr_json},
};

#[derive(Serialize, Deserialize)]
struct RowData {
    id: Uuid,
    teams: JsonString,
}

#[get("")]
pub async fn get_all(pool: Data<PgPool>) -> impl Responder {
    match query_as!(
        RowData,
        r#"select id as "id!",
        teams as "teams!: String"
        from tournaments_team_applications"#
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(tournaments) => {
            resp_200_Ok_json!(tournaments)
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
    const URI: &str = "/tournaments/team_applications";

    #[actix_web::test]
    async fn test_ok() {
        let (app, rollbacker, pool) = get_test_app().await;

        let req = test::TestRequest::get().uri(URI).to_request();
        let resp = test::call_service(&app, req).await;

        assert_resp_status_eq_or_rollback!(resp, 200, rollbacker);
        let res: Vec<RowData> = read_body_json(resp).await;
        let res_num = res.len();

        let game_id = new_game_insert(&pool).await;
        ok_or_rollback_game!(game_id, rollbacker);
        let tournament_id = new_tournament_insert_random(
            game_id,
            false,
            false,
            TournamentType::OneBracketOneFinalPositions,
            &pool,
        )
        .await;
        ok_or_rollback_tournament!(tournament_id, _tournament_id, rollbacker);

        let req = test::TestRequest::get().uri(URI).to_request();
        let resp = test::call_service(&app, req).await;
        assert_resp_status_eq_or_rollback!(resp, 200, rollbacker);
        let res: Vec<RowData> = read_body_json(resp).await;
        rollbacker.rollback().await;
        assert_ne!(res.len(), res_num);
    }
}
