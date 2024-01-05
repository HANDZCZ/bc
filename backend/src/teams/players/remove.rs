use std::usize;

use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use sqlx::{PgPool, query};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{macros::{resp_200_Ok_json, resp_400_BadReq_json, resp_500_IntSerErr_json}, jwt_stuff::LoggedInUser};

#[derive(Serialize, Deserialize)]
struct PlayersToTeam {
    player_ids: Vec<Uuid>,
    team_id: Uuid,
}

#[post("/remove")]
pub async fn remove(pool: Data<PgPool>, data: Json<PlayersToTeam>, user: LoggedInUser) -> impl Responder {
    match query!(
        "call remove_players_from_team($1, $2, $3)",
        user.id,
        data.team_id,
        &data.player_ids
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            resp_200_Ok_json!()
        }
        Err(sqlx::Error::Database(error)) => {
            let err = crate::common::Error::new(error.message().to_owned());
            resp_400_BadReq_json!(err)
        }
        Err(_) => {
            resp_500_IntSerErr_json!()
        }
    }
}
