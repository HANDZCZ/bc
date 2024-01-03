use actix_web::{get, web::Data, Responder};
use sqlx::{query_as, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    common::JsonString,
    macros::{resp_200_Ok_json, resp_500_IntSerErr_json},
};

#[derive(Serialize, Deserialize)]
struct RowData {
    id: Uuid,
    name: String,
    description: String,

    game_id: Uuid,
    game_name: String,
    game_description: String,
    game_version: String,

    bracket_trees: JsonString,
}

#[get("")]
pub async fn get_all(pool: Data<PgPool>) -> impl Responder {
    match query_as!(
        RowData,
        r#"select id as "id!",
        name as "name!",
        description as "description!",
        game_id as "game_id!",
        game_name as "game_name!",
        game_description as "game_description!",
        game_version as "game_version!",
        bracket_trees as "bracket_trees!: String"
 from tournaments_with_brackets_and_game_info"#
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(tournaments) => {
            resp_200_Ok_json!(tournaments)
        }
        Err(_) => {
            resp_500_IntSerErr_json!()
        }
    }
}
