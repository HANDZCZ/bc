use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use sqlx::{query, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    common::Error,
    macros::{resp_200_Ok_json, resp_400_BadReq_json, resp_500_IntSerErr_json}, jwt_stuff::LoggedInUser,
};

#[derive(Serialize, Deserialize)]
struct Invite {
    team_id: Uuid,
    accepted: bool,
}

#[post("/handle_invite")]
pub async fn handle_invite(pool: Data<PgPool>, data: Json<Invite>, user: LoggedInUser) -> impl Responder {
    match query!(
        "call handle_player_invite($1, $2, $3)",
        user.id,
        data.team_id,
        data.accepted
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            resp_200_Ok_json!()
        }
        Err(sqlx::Error::Database(error)) => {
            let err = Error::new(error.message().to_owned());
            resp_400_BadReq_json!(err)
        }
        Err(_) => {
            resp_500_IntSerErr_json!()
        }
    }
}
