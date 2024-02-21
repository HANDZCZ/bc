use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use sqlx::{query_as, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    jwt_stuff::LoggedInUser,
    macros::{resp_200_Ok_json, resp_400_BadReq_json, resp_500_IntSerErr_json},
};

#[derive(Serialize, Deserialize)]
struct Team {
    name: String,
    description: String,
}

#[derive(Deserialize, Serialize)]
struct ReturningRow {
    id: Uuid,
}

#[post("/new")]
pub async fn new(pool: Data<PgPool>, data: Json<Team>, user: LoggedInUser) -> impl Responder {
    match query_as!(
        ReturningRow,
        r#"select new_team($1, $2, $3) as "id!""#,
        user.id,
        data.name,
        data.description,
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(row) => resp_200_Ok_json!(row),
        Err(sqlx::Error::Database(error)) => {
            if error.is_unique_violation() {
                let err =
                    crate::common::Error::new("request for new team violates unique constraints");
                resp_400_BadReq_json!(err)
            } else {
                let err = crate::common::Error::new(format!("unhandled error - {}", error));
                resp_400_BadReq_json!(err)
            }
        }
        Err(_) => resp_500_IntSerErr_json!(),
    }
}

#[cfg(test)]
pub mod tests {
    use actix_web::test;

    use super::*;
    use crate::tests::*;
    const URI: &str = "/teams/new";

    #[actix_web::test]
    pub async fn test_ok() {
        let (app, rollbacker, _pool) = get_test_app().await;
        let auth_header = get_regular_users_auth_header(&app).await;

        let data = Team {
            name: "test-team".into(),
            description: "test-team".into(),
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
