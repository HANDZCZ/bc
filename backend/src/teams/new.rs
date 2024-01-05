use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use sqlx::{query, query_as, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{macros::{
    begin_transaction, resp_200_Ok_json, resp_400_BadReq_json, resp_500_IntSerErr_json,
}, jwt_stuff::LoggedInUser};

#[derive(Serialize, Deserialize)]
struct Team {
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct ReturningRow {
    id: Uuid,
}

#[post("/new")]
pub async fn new(pool: Data<PgPool>, data: Json<Team>, user: LoggedInUser) -> impl Responder {
    // TODO: redo in db + add user to managers
    begin_transaction!(pool, tx);

    let team_row = match query_as!(
        ReturningRow,
        "insert into teams (name, description) values ($1, $2) returning teams.id",
        data.name,
        data.description,
    )
    .fetch_one(&mut *tx)
    .await
    {
        Ok(row) => row,
        Err(sqlx::Error::Database(error)) => {
            rollback_tx!();
            return if error.is_unique_violation() {
                let err = crate::common::Error::new("request for new team violates unique constraints");
                resp_400_BadReq_json!(err)
            } else {
                let err = crate::common::Error::new(format!("unhandled error - {}", error));
                resp_400_BadReq_json!(err)
            };
        }
        Err(_) => {
            rollback_tx!();
            return resp_500_IntSerErr_json!();
        }
    };

    // TODO: finish after auth system - more like redo in DB
    /*match query!(
        "insert into managers_to_teams (manager_id, team_id) values ($1, $2)",
        manager_id,
        team_row.id,
    )
    .execute(&mut *tx)
    .await
    {
        Ok(_) => {}
        Err(sqlx::Error::Database(error)) => {
            // should be impossible to reach
            rollback_tx!();
            return resp_500_IntSerErr_json!();
        }
        Err(_) => {
            rollback_tx!();
            return resp_500_IntSerErr_json!();
        }
    }*/

    commit_tx!();
    resp_200_Ok_json!(team_row)
}
