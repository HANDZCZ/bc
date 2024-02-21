use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use sqlx::{query, PgPool};

use serde::{Deserialize, Serialize};

use crate::{
    hash_utils::{make_hash, make_salt},
    jwt_stuff::LoggedInUser,
    macros::{resp_200_Ok_json, resp_400_BadReq_json, resp_500_IntSerErr_json},
};

#[derive(Serialize, Deserialize)]
struct User {
    nick: Option<String>,
    email: Option<String>,
    password: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct RowsAffected {
    rows_affected: u64,
}

#[post("/edit")]
pub async fn edit(pool: Data<PgPool>, data: Json<User>, user: LoggedInUser) -> impl Responder {
    let (salt, hash) = if let Some(password) = data.password.as_ref() {
        let salt = make_salt();
        let hash = make_hash(password, &salt);
        (Some(salt), Some(hash.to_vec()))
    } else {
        (None, None)
    };

    match query!(
        "update users set email = coalesce($1, email), nick = coalesce($2, nick), hash = coalesce($3, hash), salt = coalesce($4, salt) where id = $5",
        data.email,
        data.nick,
        hash,
        salt,
        user.id
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
                let err = crate::common::Error::new("request for user edit violates unique constraints");
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
    use crate::tests::*;
    const URI: &str = "/users/edit";

    #[actix_web::test]
    pub async fn test_unauthorized() {
        let (app, rollbacker, _pool) = get_test_app().await;

        let data = User {
            email: None,
            nick: None,
            password: None,
        };
        let req = test::TestRequest::post()
            .uri(URI)
            .set_json(data)
            .to_request();
        let resp = test::call_service(&app, req).await;

        rollbacker.rollback().await;
        assert_eq!(resp.status().as_u16(), 401);
    }

    #[actix_web::test]
    pub async fn test_ok() {
        let (app, rollbacker, _pool) = get_test_app().await;
        let auth_header = get_tournament_managers_auth_header(&app).await;

        let data = User {
            email: None,
            nick: None,
            password: None,
        };
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
