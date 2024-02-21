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
    macros::{check_user_authority, resp_200_Ok_json, resp_500_IntSerErr_json},
};

#[derive(Serialize, Deserialize)]
struct Tournament {
    id: Uuid,
}

#[derive(Serialize, Deserialize)]
struct RowsAffected {
    rows_affected: u64,
}

#[post("/delete")]
pub async fn delete(
    pool: Data<PgPool>,
    data: Json<Tournament>,
    user: LoggedInUserWithAuthorities,
) -> impl Responder {
    check_user_authority!(user, "role::Tournament Manager");

    match query!("delete from tournaments where id = $1", data.id)
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
    use crate::{common::TournamentType, tests::*};
    const URI: &str = "/tournaments/delete";

    #[actix_web::test]
    async fn test_forbidden() {
        let data = Tournament { id: Uuid::new_v4() };

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
        let id =
            new_tournament_insert_random(game_id, false, false, TournamentType::FFA, &pool).await;
        ok_or_rollback_tournament!(id, rollbacker);

        let data = Tournament { id };

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
