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
    macros::{
        resp_200_Ok_json, resp_400_BadReq_json, resp_403_Forbidden_json, resp_500_IntSerErr_json,
    },
};

#[derive(Serialize, Deserialize)]
struct ToSet {
    players: Vec<Player>,
    team_id: Uuid,
}

#[derive(Serialize, Deserialize)]
struct Player {
    id: Uuid,
    playing: bool,
}

#[post("/set_playing")]
pub async fn set_playing(
    pool: Data<PgPool>,
    data: Json<ToSet>,
    user: LoggedInUser,
) -> impl Responder {
    let (playing, not_playing) = data.players.iter().fold(
        (Vec::new(), Vec::new()),
        |(mut playing, mut not_playing), player| {
            if player.playing {
                playing.push(player.id);
            } else {
                not_playing.push(player.id);
            }
            (playing, not_playing)
        },
    );

    match query!(
        "call set_playing($1, $2, $3, $4)",
        user.id,
        data.team_id,
        &playing,
        &not_playing
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
