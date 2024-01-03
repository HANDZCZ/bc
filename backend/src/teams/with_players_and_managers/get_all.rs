use actix_web::{
    get,
    web::Data,
    Responder,
};
use sqlx::{query_as, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    common::JsonString,
    macros::{resp_200_Ok_json, resp_500_IntSerErr_json},
};

#[derive(Serialize, Deserialize)]
struct Info {
    id: Uuid,
    name: String,
    description: String,
    players: JsonString,
    managers: JsonString,
}

#[get("")]
pub async fn get_all(pool: Data<PgPool>) -> impl Responder {
    match query_as!(Info, r#"select id as "id!", name as "name!", description as "description!", players as "players!: String", managers as "managers!: String" from teams_with_players_and_managers"#)
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(teams) => {
            resp_200_Ok_json!(teams)
        }
        Err(_) => {
            resp_500_IntSerErr_json!()
        }
    }
}
