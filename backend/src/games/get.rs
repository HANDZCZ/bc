use actix_web::{
    get,
    web::{self, Data},
    Responder,
};
use sqlx::{query_as, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::macros::{resp_200_Ok_json, resp_500_IntSerErr_json, resp_400_BadReq_json};

#[derive(Serialize, Deserialize)]
struct Game {
    name: String,
    description: String,
    version: String,
}

#[get("/{id}")]
pub async fn get(pool: Data<PgPool>, id: web::Path<Uuid>) -> impl Responder {
    match query_as!(Game, "select name, description, version from games where id = $1", id.into_inner())
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(game) => {
            resp_200_Ok_json!(game)
        }
        Err(sqlx::Error::RowNotFound) => {
            let err = crate::common::Error::new("game not found");
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

    use super::*;
    use crate::tests::*;
    const URI: &str = "/games";

    #[actix_web::test]
    async fn test_ok() {
        let (app, rollbacker, pool) = get_test_app().await;

        let id = new_game_insert(&pool).await;
        ok_or_rollback_game!(id, rollbacker);
        let req = test::TestRequest::get()
            .uri(&format!("{}/{}", URI, id))
            .to_request();
        let resp = test::call_service(&app, req).await;
        rollbacker.rollback().await;
        assert_eq!(resp.status().as_u16(), 200);
    }

    #[actix_web::test]
    async fn test_bad_req() {
        let (app, rollbacker, _pool) = get_test_app().await;

        let req = test::TestRequest::get()
            .uri(&format!("{}/{}", URI, Uuid::new_v4()))
            .to_request();
        let resp = test::call_service(&app, req).await;
        rollbacker.rollback().await;
        assert_eq!(resp.status().as_u16(), 400);
    }
}