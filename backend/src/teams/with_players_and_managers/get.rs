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
    macros::{resp_200_Ok_json, resp_500_IntSerErr_json},
};

#[derive(Serialize, Deserialize)]
struct Info {
    name: String,
    description: String,
    players: JsonString,
    managers: JsonString,
}

#[get("/{id}")]
pub async fn get(pool: Data<PgPool>, id: web::Path<Uuid>) -> impl Responder {
    match query_as!(
        Info,
        r#"select name as "name!", description as "description!", players as "players!: String", managers as "managers!: String" from teams_with_players_and_managers where id = $1"#,
        id.into_inner()
    )
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
    use actix_web::test;

    use crate::tests::*;
    const URI: &str = "/teams/with_players_and_managers";

    #[actix_web::test]
    async fn test_ok_new_team() {
        let (app, rollbacker, pool) = get_test_app().await;
        let (_, user_id) = new_user_insert_random(&app).await;

        let team_id = new_team_insert_random(user_id, &pool).await;
        ok_or_rollback_team!(team_id, rollbacker);

        let req = test::TestRequest::get()
            .uri(&format!("{}/{}", URI, team_id))
            .to_request();
        let resp = test::call_service(&app, req).await;

        rollbacker.rollback().await;
        assert_eq!(resp.status().as_u16(), 200);
    }
}
