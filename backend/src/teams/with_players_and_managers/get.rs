use actix_web::{
    get,
    web::{self, Data},
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
    name: String,
    description: String,
    players: JsonString,
    managers: JsonString,
}

#[get("/{id}")]
pub async fn get(pool: Data<PgPool>, id: web::Path<Uuid>) -> impl Responder {
    match query_as!(
        Info,
        r#"select name as "name!", description as "description!", players as "players!: String", managers as "managers!: String" from teams_with_players_and_managers where id = $1"#,
        id.into_inner()
    )
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
