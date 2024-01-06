use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use sqlx::{query, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    jwt_stuff::LoggedInUser,
    macros::{resp_200_Ok_json, resp_400_BadReq_json, resp_500_IntSerErr_json, resp_403_Forbidden_json},
};

#[derive(Serialize, Deserialize)]
struct ManagerToTeam {
    manager_ids: Vec<Uuid>,
    team_id: Uuid,
}

#[post("/invite")]
pub async fn invite(
    pool: Data<PgPool>,
    data: Json<ManagerToTeam>,
    user: LoggedInUser,
) -> impl Responder {
    match query!(
        "call invite_managers_to_team($1, $2, $3)",
        user.id,
        data.team_id,
        &data.manager_ids
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            resp_200_Ok_json!()
        }
        Err(sqlx::Error::Database(error)) => {
            if error.is_foreign_key_violation() {
                let err = crate::common::Error::new("inviting users to team failed - foreign key constraints violation (team_id, manager_id)");
                resp_400_BadReq_json!(err)
            } else if let Some(true) = error.code().map(|c| c == "66666") {
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
