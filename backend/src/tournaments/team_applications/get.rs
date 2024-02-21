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
struct Tournament {
    teams: JsonString,
}

#[get("/{id}")]
pub async fn get(pool: Data<PgPool>, id: web::Path<Uuid>) -> impl Responder {
    match query_as!(
        Tournament,
        r#"select teams as "teams!: String"
        from tournaments_team_applications
        where id = $1"#,
        id.into_inner()
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(tournament) => {
            resp_200_Ok_json!(tournament)
        }
        Err(sqlx::Error::RowNotFound) => {
            let err = crate::common::Error::new("tournament not found");
            resp_400_BadReq_json!(err)
        }
        Err(_) => {
            resp_500_IntSerErr_json!()
        }
    }
}

#[cfg(test)]
mod tests {
    use actix_web::test::{self, read_body};

    use crate::{common::TournamentType, tests::*};
    const URI: &str = "/tournaments/team_applications";

    #[actix_web::test]
    async fn test_ok() {
        let (app, rollbacker, pool) = get_test_app().await;

        let game_id = new_game_insert(&pool).await;
        ok_or_rollback_game!(game_id, rollbacker);
        let tournament_id = new_tournament_insert_random(
            game_id,
            true,
            false,
            TournamentType::OneBracketOneFinalPositions,
            &pool,
        )
        .await;
        ok_or_rollback_tournament!(tournament_id, tournament_id, rollbacker);

        let req = test::TestRequest::get()
            .uri(&format!("{}/{}", URI, tournament_id))
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_resp_status_eq_or_rollback!(resp, 200, rollbacker);
        let res = String::from_utf8(read_body(resp).await.to_vec()).unwrap();
        assert!(res.contains("[]"));

        let (_auth_header, user_id) = new_user_insert_random(&app).await;
        let team_id = new_team_insert_random(user_id, &pool).await;
        ok_or_rollback_team!(team_id, rollbacker);
        let ins_res =
            new_team_to_tournament_application_insert(team_id, tournament_id, &pool).await;
        ok_or_rollback_teams_to_tournament_application!(ins_res, _ins_res, rollbacker);

        let req = test::TestRequest::get()
            .uri(&format!("{}/{}", URI, tournament_id))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_resp_status_eq_or_rollback!(resp, 200, rollbacker);
        let res = String::from_utf8(read_body(resp).await.to_vec()).unwrap();
        rollbacker.rollback().await;
        assert!(res.contains(&team_id.to_string()));
    }
}
