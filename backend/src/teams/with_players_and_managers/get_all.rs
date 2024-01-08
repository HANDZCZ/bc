use actix_web::{
    get,
    web::Data,
    Responder,
};
use sqlx::{query_as, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    common::JsonString,
    macros::{resp_200_Ok_json, resp_500_IntSerErr_json},
};

#[derive(Serialize, Deserialize)]
struct Info {
    id: Uuid,
    name: String,
    description: String,
    players: JsonString,
    managers: JsonString,
}

#[get("")]
pub async fn get_all(pool: Data<PgPool>) -> impl Responder {
    match query_as!(Info, r#"select id as "id!", name as "name!", description as "description!", players as "players!: String", managers as "managers!: String" from teams_with_players_and_managers"#)
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(teams) => {
            resp_200_Ok_json!(teams)
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
    use crate::tests::*;
    const URI: &str = "/teams/with_players_and_managers";

    #[actix_web::test]
    async fn test_ok_new_team() {
        let (app, rollbacker, pool) = get_test_app().await;

        let req = test::TestRequest::get()
            .uri(URI)
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_resp_status_eq_or_rollback!(resp, 200, rollbacker);
        let res: Vec<Info> = read_body_json(resp).await;
        let res_num = res.len();

        let (_auth_header, user_id) = new_user_insert_random(&app).await;
        let _team_id = new_team_insert_random(user_id, &pool).await;

        let req = test::TestRequest::get()
            .uri(URI)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_resp_status_eq_or_rollback!(resp, 200, rollbacker);
        let res: Vec<Info> = read_body_json(resp).await;
        rollbacker.rollback().await;
        assert_ne!(res.len(), res_num);
    }
}