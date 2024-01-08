use actix_web::{get, web::Data, Responder};
use sqlx::{query_as, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{macros::{resp_200_Ok_json, resp_500_IntSerErr_json}, common::TournamentType};

#[derive(Serialize, Deserialize)]
struct Tournament {
    id: Uuid,
    name: String,
    description: String,
    min_team_size: i32,
    max_team_size: i32,
    requires_application: bool,
    applications_closed: bool,
    tournament_type: TournamentType,
    game_id: Uuid,
    game_name: String,
    game_description: String,
    game_version: String,
}

#[get("")]
pub async fn get_all(pool: Data<PgPool>) -> impl Responder {
    match query_as!(
        Tournament,
        r#"select id as "id!",
        name as "name!",
        description as "description!",
        min_team_size as "min_team_size!",
        max_team_size as "max_team_size!",
        requires_application as "requires_application!",
        applications_closed as "applications_closed!",
        tournament_type as "tournament_type!: TournamentType",
        game_id as "game_id!", game_name as "game_name!",
        game_description as "game_description!",
        game_version as "game_version!"
        from tournaments_and_game_info"#
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
    use crate::{tests::*, common::TournamentType};
    const URI: &str = "/tournaments";

    #[actix_web::test]
    async fn test_ok() {
        let (app, rollbacker, pool) = get_test_app().await;

        let game_id = new_game_insert(&pool).await;
        ok_or_rollback_game!(game_id, rollbacker);
        let tournament_id = new_tournament_insert_random(game_id, false, false, TournamentType::OneBracketOneFinalPositions, &pool).await;
        ok_or_rollback_tournament!(tournament_id, _tournament_id, rollbacker);

        let req = test::TestRequest::get()
            .uri(URI)
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_resp_status_eq_or_rollback!(resp, 200, rollbacker);
        let res: Vec<Tournament> = read_body_json(resp).await;
        let res_num = res.len();

        let tournament_id = new_tournament_insert_random(game_id, false, false, TournamentType::OneBracketOneFinalPositions, &pool).await;
        ok_or_rollback_tournament!(tournament_id, _tournament_id, rollbacker);

        let req = test::TestRequest::get()
            .uri(URI)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_resp_status_eq_or_rollback!(resp, 200, rollbacker);
        let res: Vec<Tournament> = read_body_json(resp).await;
        rollbacker.rollback().await;
        assert_ne!(res.len(), res_num);
    }
}