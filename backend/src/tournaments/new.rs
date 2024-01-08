use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use sqlx::{query_as, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    common::TournamentType,
    jwt_stuff::LoggedInUserWithAuthorities,
    macros::{
        check_user_authority, resp_200_Ok_json, resp_400_BadReq_json, resp_500_IntSerErr_json,
    },
};

#[derive(Serialize, Deserialize)]
struct Tournament {
    name: String,
    description: String,
    game_id: Uuid,
    min_team_size: i32,
    max_team_size: i32,
    requires_application: bool,
    tournament_type: TournamentType,
}

#[derive(Serialize, Deserialize)]
struct ReturningRow {
    id: Uuid,
}

#[post("/new")]
pub async fn new(
    pool: Data<PgPool>,
    data: Json<Tournament>,
    user: LoggedInUserWithAuthorities,
) -> impl Responder {
    check_user_authority!(user, "role::Tournament Manager");

    match query_as!(
        ReturningRow,
        "insert into tournaments (name, description, game_id, min_team_size, max_team_size, requires_application, tournament_type) values ($1, $2, $3, $7, $4, $5, $6) returning tournaments.id",
        data.name,
        data.description,
        data.game_id,
        data.max_team_size,
        data.requires_application,
        data.tournament_type as TournamentType,
        data.min_team_size
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(game_row) => {
            resp_200_Ok_json!(game_row)
        }
        Err(sqlx::Error::Database(error)) => {
            if error.is_unique_violation() {
                let err = crate::common::Error::new("request for new tournament violates unique constraints");
                resp_400_BadReq_json!(err)
            } else if error.is_foreign_key_violation() {
                let err = crate::common::Error::new("request for new tournament violates foreign key constraints (game id doesn't exists)");
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
    use actix_web::test;

    use super::*;
    use crate::tests::*;
    const URI: &str = "/tournaments/new";

    #[actix_web::test]
    pub async fn test_forbidden() {
        let (app, rollbacker, _pool) = get_test_app().await;
        let reg_user_header = get_regular_users_auth_header(&app).await;

        let data = Tournament {
            name: "test-tournament".into(),
            description: "test-tournament".into(),
            game_id: Uuid::new_v4(),
            min_team_size: 4,
            max_team_size: 4,
            requires_application: false,
            tournament_type: TournamentType::FFA,
        };

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
        let (app, rollbacker, pool) = get_test_app().await;
        let auth_header = get_tournament_managers_auth_header(&app).await;
        let game_id = new_game_insert(&pool).await;
        ok_or_rollback_game!(game_id, rollbacker);

        let data = Tournament {
            name: "test-tournament".into(),
            description: "test-tournament".into(),
            game_id,
            min_team_size: 4,
            max_team_size: 4,
            requires_application: false,
            tournament_type: TournamentType::FFA,
        };

        let req = test::TestRequest::post()
            .uri(URI)
            .insert_header(auth_header)
            .set_json(data)
            .to_request();
        let resp = test::call_service(&app, req).await;

        rollbacker.rollback().await;
        assert_eq!(resp.status().as_u16(), 200);
    }
}
