use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use sqlx::{query, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{macros::{resp_200_Ok_json, resp_500_IntSerErr_json, check_user_authority}, jwt_stuff::LoggedInUserWithAuthorities};

#[derive(Serialize, Deserialize)]
struct Bracket {
    bracket_tree_id: Uuid,
    layer: u8,
    // TODO: set min to 0
    position: i32,
}

#[derive(Serialize, Deserialize)]
struct RowsAffected {
    rows_affected: u64,
}

#[post("/delete")]
pub async fn delete(pool: Data<PgPool>, data: Json<Bracket>, user: LoggedInUserWithAuthorities) -> impl Responder {
    check_user_authority!(user, "role::Tournament Manager");

    match query!(
        "delete from brackets where bracket_tree_id = $1 and layer = $2 and position = $3",
        data.bracket_tree_id,
        data.layer as i16,
        data.position
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
    const URI: &str = "/brackets/delete";

    #[actix_web::test]
    async fn test_forbidden() {
        let data = Bracket {
            bracket_tree_id: Uuid::new_v4(),
            layer: 0,
            position: 0
        };

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
    async fn test_ok() {
        let (app, rollbacker, pool) = get_test_app().await;
        let auth_header = get_tournament_managers_auth_header(&app).await;

        let game_id = new_game_insert(&pool).await;
        ok_or_rollback_game!(game_id, rollbacker);
        let tournament_id = new_tournament_insert_random(game_id, false, false, TournamentType::OneBracketOneFinalPositions, &pool).await;
        ok_or_rollback_tournament!(tournament_id, rollbacker);
        let bracket_tree_id = new_bracket_tree_insert(tournament_id, 0, &pool).await;
        ok_or_rollback_bracket_tree!(bracket_tree_id, rollbacker);
        let _bracket_res = new_bracket_insert(bracket_tree_id, 255, 0, &pool).await;
        ok_or_rollback_bracket!(_bracket_res, rollbacker);

        let data = Bracket {
            bracket_tree_id: bracket_tree_id,
            layer: 255,
            position: 0
        };

        let req = test::TestRequest::post()
            .uri(URI)
            .insert_header(auth_header)
            .set_json(data)
            .to_request();
        let resp = test::call_service(&app, req).await;

        rollbacker.rollback().await;
        assert_eq!(resp.status().as_u16(), 200);
        let res: RowsAffected = read_body_json(resp).await;
        assert_eq!(res.rows_affected, 1);
    }
}