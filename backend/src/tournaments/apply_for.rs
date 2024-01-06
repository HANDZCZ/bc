use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use sqlx::{PgPool, query};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{macros::{resp_200_Ok_json, resp_400_BadReq_json, resp_500_IntSerErr_json, resp_403_Forbidden_json}, jwt_stuff::LoggedInUser};

#[derive(Serialize, Deserialize)]
struct ReqData {
    tournament_id: Uuid,
    team_id: Uuid,
}

#[post("/apply_for")]
pub async fn apply_for(pool: Data<PgPool>, data: Json<ReqData>, user: LoggedInUser) -> impl Responder {
    match query!(
        "call apply_for_tournament($1, $2, $3)",
        user.id,
        data.team_id,
        data.tournament_id
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            resp_200_Ok_json!()
        }
        Err(sqlx::Error::Database(error)) => {
            if let Some(true) = error.code().map(|c| c == "66666") {
                let err = crate::common::Error::new(error.message());
                resp_403_Forbidden_json!(err)
            } else if let Some(true) = error.code().map(|c| c == "44444") {
                let err = crate::common::Error::new(error.message());
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
