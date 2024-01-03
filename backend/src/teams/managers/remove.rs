use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use sqlx::{PgPool, query};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::macros::{resp_200_Ok_json, resp_400_BadReq_json, resp_500_IntSerErr_json};

#[derive(Serialize, Deserialize)]
struct ManagersToTeam {
    managers_ids: Vec<Uuid>,
    team_id: Uuid,
}

#[post("/remove")]
pub async fn remove(pool: Data<PgPool>, data: Json<ManagersToTeam>) -> impl Responder {
    let Ok(mut tx) = pool.get_ref().begin().await else {
        return resp_500_IntSerErr_json!();
    };

    macro_rules! rollback {
        () => {
            if tx.rollback().await.is_err() {
                return resp_500_IntSerErr_json!();
            }
        };
    }

    for manager in &data.managers_ids {
        match query!(
            "delete from managers_to_teams where manager_id = $1 and team_id = $2",
            manager,
            data.team_id,
        )
        .execute(&mut *tx)
        .await
        {
            Ok(_) => {}
            Err(sqlx::Error::Database(error)) => {
                if error.is_unique_violation() {
                    continue;
                } else if error.is_foreign_key_violation() {
                    let err = crate::common::Error {
                        error: "removing managers from team failed - foreign key constraints violation (team_id, manager_id)".to_owned(),
                    };
                    rollback!();
                    return resp_400_BadReq_json!(err);
                }
            }
            Err(_) => {
                rollback!();
                return resp_500_IntSerErr_json!()
            }
        }
    }

    if tx.commit().await.is_err() {
        return resp_500_IntSerErr_json!();
    };
    resp_200_Ok_json!()
}
