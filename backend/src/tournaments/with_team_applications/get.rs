use actix_web::{
    get,
    web::{self, Data},
    Responder,
};
use sqlx::{query_as, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{macros::{resp_200_Ok_json, resp_500_IntSerErr_json, resp_400_BadReq_json}, common::JsonString};

#[derive(Serialize, Deserialize)]
struct Tournament {
    teams: JsonString,
}

#[get("/{id}")]
pub async fn get(pool: Data<PgPool>, id: web::Path<Uuid>) -> impl Responder {
    match query_as!(
        Tournament,
        r#"select teams as "teams!: String"
        from tournaments_with_team_applications
        where id = $1"#,
        id.into_inner()
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(tournament) => {
            resp_200_Ok_json!(tournament)
        }
        Err(sqlx::Error::RowNotFound) => {
            let err = crate::common::Error::new("tournament not found");
            resp_400_BadReq_json!(err)
        }
        Err(_) => {
            resp_500_IntSerErr_json!()
        }
    }
}
