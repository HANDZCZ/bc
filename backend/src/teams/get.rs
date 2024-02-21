use actix_web::{
    get,
    web::{self, Data},
    Responder,
};
use sqlx::{query_as, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::macros::{resp_200_Ok_json, resp_400_BadReq_json, resp_500_IntSerErr_json};

#[derive(Serialize, Deserialize)]
struct Team {
    name: String,
    description: String,
}

#[get("/{id}")]
pub async fn get(pool: Data<PgPool>, id: web::Path<Uuid>) -> impl Responder {
    match query_as!(
        Team,
        "select name, description from teams where id = $1",
        id.into_inner()
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(team) => {
            resp_200_Ok_json!(team)
        }
        Err(sqlx::Error::RowNotFound) => {
            let err = crate::common::Error::new("team not found");
            resp_400_BadReq_json!(err)
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
    const URI: &str = "/teams";

    #[actix_web::test]
    async fn test_ok() {
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
