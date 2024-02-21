use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use serde_json::json;
use sqlx::{query, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    jwt_stuff::LoggedInUser,
    macros::{
        resp_200_Ok_json, resp_400_BadReq_json, resp_403_Forbidden_json, resp_500_IntSerErr_json,
    },
};

#[derive(Serialize, Deserialize)]
struct RequestData {
    team_id: Uuid,
    tournament_id: Uuid,
}

#[post("/leave_tournament")]
pub async fn leave_tournament(
    pool: Data<PgPool>,
    data: Json<RequestData>,
    user: LoggedInUser,
) -> impl Responder {
    match query!(
        "select handle_leave_tournament($1, $2, $3)",
        data.tournament_id,
        data.team_id,
        user.id
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(res) => match res.handle_leave_tournament {
            Some(val) => resp_200_Ok_json!(json!({
                "manual_edit_needed": !val
            })),
            None => resp_500_IntSerErr_json!(),
        },
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
