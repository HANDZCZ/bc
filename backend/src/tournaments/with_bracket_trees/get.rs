use actix_web::{
    get,
    web::{self, Data},
    Responder,
};
use sqlx::{query_as, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    common::{JsonString, TournamentType},
    macros::{resp_200_Ok_json, resp_400_BadReq_json, resp_500_IntSerErr_json},
};

#[derive(Serialize, Deserialize)]
struct Tournament {
    name: String,
    description: String,
    min_team_size: i32,
    max_team_size: i32,
    requires_application: bool,
    applications_closed: bool,
    tournament_type: TournamentType,

    game_id: Uuid,
    game_name: String,
    game_description: String,
    game_version: String,

    bracket_trees: JsonString,
}

#[get("/{id}")]
pub async fn get(pool: Data<PgPool>, id: web::Path<Uuid>) -> impl Responder {
    match query_as!(
        Tournament,
        r#"select name as "name!",
        description as "description!",
        min_team_size as "min_team_size!",
        max_team_size as "max_team_size!",
        requires_application as "requires_application!",
        applications_closed as "applications_closed!",
        tournament_type as "tournament_type!: TournamentType",
        game_id as "game_id!",
        game_name as "game_name!",
        game_description as "game_description!",
        game_version as "game_version!",
        bracket_trees as "bracket_trees!: String"
        from tournaments_with_bracket_trees_and_game_info
        where id = $1"#,
        id.into_inner()
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(tournament) => {
            resp_200_Ok_json!(tournament)
        }
        Err(sqlx::Error::RowNotFound) => {
            let err = crate::common::Error::new("tournament not found");
            resp_400_BadReq_json!(err)
        }
        Err(_) => {
            resp_500_IntSerErr_json!()
        }
    }
}
