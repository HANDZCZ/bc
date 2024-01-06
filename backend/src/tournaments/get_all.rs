use actix_web::{get, web::Data, Responder};
use sqlx::{query_as, PgPool};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{macros::{resp_200_Ok_json, resp_500_IntSerErr_json}, common::TournamentType};

#[derive(Serialize, Deserialize)]
struct Tournament {
    id: Uuid,
    name: String,
    description: String,
    max_team_size: i32,
    requires_application: bool,
    applications_closed: bool,
    tournament_type: TournamentType,
    game_id: Uuid,
    game_name: String,
    game_description: String,
    game_version: String,
}

#[get("")]
pub async fn get_all(pool: Data<PgPool>) -> impl Responder {
    match query_as!(
        Tournament,
        r#"select id as "id!",
        name as "name!",
        description as "description!",
        max_team_size as "max_team_size!",
        requires_application as "requires_application!",
        applications_closed as "applications_closed!",
        tournament_type as "tournament_type!: TournamentType",
        game_id as "game_id!", game_name as "game_name!",
        game_description as "game_description!",
        game_version as "game_version!"
        from tournaments_and_game_info"#
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
