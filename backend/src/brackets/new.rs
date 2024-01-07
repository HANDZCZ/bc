use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use sqlx::{query_as, PgPool};

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

#[post("/new")]
pub async fn new(
    pool: Data<PgPool>,
    data: Json<Bracket>,
    user: LoggedInUserWithAuthorities,
) -> impl Responder {
    check_user_authority!(user, "role::Tournament Manager");

    match query_as!(
        ReturningRow,
        "insert into brackets (team1, team2, winner, bracket_tree_id, layer, position) values ($1, $2, $3, $4, $5, $6)",
        data.team1,
        data.team2,
        data.winner,
        data.bracket_tree_id,
        data.layer as i32,
        data.position
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            resp_200_Ok_json!()
        }
        Err(sqlx::Error::Database(error)) => {
            if error.is_unique_violation() {
                let err = crate::common::Error::new("request for new bracket violates unique constraints");
                resp_400_BadReq_json!(err)
            } else if error.is_foreign_key_violation() {
                let err = crate::common::Error::new("request for new bracket violates foreign key constraints (bracket_tree_id, team1, team2)");
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
    use actix_web::test::{self, read_body};

    use super::*;
    use crate::{tests::*, common::TournamentType};
    const URI: &str = "/brackets/new";

    fn get_data() -> Bracket {
        Bracket {
            team1: None,
            team2: None,
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
        let tournament_id = new_tournament_insert(game_id, false, false, TournamentType::OneBracketOneFinalPositions, &pool).await;
        ok_or_rollback_tournament!(tournament_id, rollbacker);
        let bracket_tree_id = new_bracket_tree_insert(tournament_id, 0, &pool).await;
        ok_or_rollback_bracket_tree!(bracket_tree_id, rollbacker);

        data.bracket_tree_id = bracket_tree_id;
        let req = test::TestRequest::post()
            .uri(URI)
            .insert_header(auth_header)
            .set_json(data)
            .to_request();
        let resp = test::call_service(&app, req).await;

        rollbacker.rollback().await;
        let code = resp.status().as_u16();
        let body = String::from_utf8(read_body(resp).await.to_vec()).unwrap();
        assert_eq!(code, 200, "{}", body);
    }
}
