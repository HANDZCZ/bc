use std::usize;

use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use sqlx::{query, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    jwt_stuff::LoggedInUserWithAuthorities,
    macros::{
        check_user_authority, resp_200_Ok_json, resp_400_BadReq_json, resp_500_IntSerErr_json,
    },
};

#[derive(Serialize, Deserialize)]
struct Bracket {
    team1: Option<Uuid>,
    team2: Option<Uuid>,
    team1_score: Option<i64>,
    team2_score: Option<i64>,
    winner: Option<bool>,
    bracket_tree_id: Uuid,
    layer: u8,
    // TODO: set min to 0
    position: i32,
}

#[derive(Serialize, Deserialize)]
struct ReturningRow {
    id: Uuid,
}

#[derive(Serialize, Deserialize)]
struct RowsAffected {
    rows_affected: u64,
}

#[post("/edit")]
pub async fn edit(
    pool: Data<PgPool>,
    data: Json<Bracket>,
    user: LoggedInUserWithAuthorities,
) -> impl Responder {
    check_user_authority!(user, "role::Tournament Manager");
    // TODO: recalculate tree (on demand)

    match query!(
        "update brackets set team1 = $1, team2 = $2, winner = $3, team1_score = coalesce($7, team1_score), team2_score = coalesce($8, team2_score) where bracket_tree_id = $4 and layer = $5 and position = $6",
        data.team1,
        data.team2,
        data.winner,
        data.bracket_tree_id,
        data.layer as i16,
        data.position,
        data.team1_score,
        data.team2_score
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(query_result) => {
            let rows_affected = RowsAffected {
                rows_affected: query_result.rows_affected(),
            };
            resp_200_Ok_json!(rows_affected)
        }
        Err(sqlx::Error::Database(error)) => {
            if error.is_unique_violation() {
                let err = crate::common::Error::new("request for bracket edit violates unique constraints");
                resp_400_BadReq_json!(err)
            } else if error.is_foreign_key_violation() {
                let err = crate::common::Error::new("request for bracket edit violates foreign key constraints (bracket_tree_id, team1, team2)");
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
    use actix_web::test::{self, read_body_json};

    use super::*;
    use crate::{common::TournamentType, tests::*};
    const URI: &str = "/brackets/edit";

    fn get_data() -> Bracket {
        Bracket {
            team1: None,
            team2: None,
            team1_score: None,
            team2_score: None,
            winner: None,
            bracket_tree_id: Uuid::new_v4(),
            layer: 255,
            position: 0,
        }
    }

    #[actix_web::test]
    pub async fn test_forbidden() {
        let data = get_data();

        let (app, rollbacker, _pool) = get_test_app().await;
        let reg_user_header = get_regular_users_auth_header(&app).await;

        let req = test::TestRequest::post()
            .uri(URI)
            .insert_header(reg_user_header)
            .set_json(data)
            .to_request();
        let resp = test::call_service(&app, req).await;

        rollbacker.rollback().await;
        assert_eq!(resp.status().as_u16(), 403);
    }

    #[actix_web::test]
    pub async fn test_ok() {
        let mut data = get_data();

        let (app, rollbacker, pool) = get_test_app().await;
        let auth_header = get_tournament_managers_auth_header(&app).await;

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
        ok_or_rollback_tournament!(tournament_id, rollbacker);
        let bracket_tree_id = new_bracket_tree_insert(tournament_id, 0, &pool).await;
        ok_or_rollback_bracket_tree!(bracket_tree_id, rollbacker);
        let _bracket_res = new_bracket_insert(bracket_tree_id, 255, 0, &pool).await;
        ok_or_rollback_bracket!(_bracket_res, rollbacker);

        data.bracket_tree_id = bracket_tree_id;
        data.winner = Some(true);
        let req = test::TestRequest::post()
            .uri(URI)
            .insert_header(auth_header)
            .set_json(data)
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_resp_status_eq_or_rollback!(resp, 200, rollbacker);
        let res: RowsAffected = read_body_json(resp).await;
        rollbacker.rollback().await;
        assert_eq!(res.rows_affected, 1);
    }
}
