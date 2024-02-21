use crate::common::TournamentType;
use actix_web::web::Data;
use serde::{Deserialize, Serialize};
use sqlx::{query, PgPool};
use uuid::Uuid;

pub async fn generate_brackets(
    tournament_id: Uuid,
    pool: Data<PgPool>,
    number_of_final_places: u32,
) -> Result<(), ()> {
    let tournament_type = query!(
        r#"select tournament_type as "tournament_type!: TournamentType" from tournaments where id = $1"#,
        tournament_id
    )
    .fetch_one(pool.get_ref())
    .await
    .unwrap()
    .tournament_type;

    #[derive(Serialize, Deserialize, Debug)]
    struct Team {
        id: Uuid,
        valid_team_size: bool,
    }

    let teams = query!(
        r#"select teams as "teams!: sqlx::types::Json<Vec<Team>>" from tournaments_signed_up_teams where id = $1"#,
        tournament_id
    )
    .fetch_one(pool.get_ref())
    .await
    .unwrap().teams.0;

    let mut valid_teams = Vec::new();
    let mut invalid_teams = Vec::new();
    teams.into_iter().for_each(|team| {
        if team.valid_team_size {
            valid_teams.push(team.id)
        } else {
            invalid_teams.push(team.id)
        }
    });

    let Ok(mut tx) = pool.get_ref().begin().await else {
        tracing::error!("Could not begin transaction");
        return Err(());
    };

    query!(
        "delete from bracket_trees where tournament_id = $1",
        tournament_id
    )
    .execute(&mut *tx)
    .await
    .unwrap();

    let rows_affected = query!(
        "delete from teams_to_tournaments where team_id = any($1) and tournament_id = $2",
        &invalid_teams,
        tournament_id
    )
    .execute(&mut *tx)
    .await
    .unwrap()
    .rows_affected();

    if rows_affected as usize != invalid_teams.len() {
        tracing::error!("Number of deleted teams from tournaments is not equal to number of teams that were supposed to be deleted!");
        // TODO: return error
        if tx.rollback().await.is_err() {
            tracing::error!("Could not rollback successfully");
        }
        return Err(());
    }

    match tournament_type {
        TournamentType::FFA => generate_ffa_brackets(valid_teams, tournament_id, &mut tx),
        TournamentType::OneBracketTwoFinalPositions => todo!(),
        TournamentType::OneBracketOneFinalPositions => todo!(),
    }
    .await?;

    if tx.commit().await.is_err() {
        tracing::error!("Could not commit successfully");
        return Err(());
    }
    Ok(())
}

async fn generate_ffa_brackets(
    teams: Vec<Uuid>,
    tournament_id: Uuid,
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Result<(), ()> {
    let bracket_tree_id = query!(
        "insert into bracket_trees (position, tournament_id) values (-1, $1) returning id",
        tournament_id
    )
    .fetch_one(&mut **transaction)
    .await
    .unwrap()
    .id;

    let size = teams.len() * (teams.len() - 1) / 2;
    let mut team1: Vec<Uuid> = Vec::with_capacity(size);
    let mut team2: Vec<Uuid> = Vec::with_capacity(size);
    let position: Vec<i32> = (0..size as i32).collect();
    for (i, team) in teams.iter().enumerate().take(teams.len() - 1) {
        team1.extend(std::iter::repeat(team).take(teams.len() - i - 1));
        team2.extend(teams.iter().skip(i + 1));
    }

    let rows_affected = query!(
        "insert into brackets (bracket_tree_id, team1, team2, position, layer) select $1, *, -1 from unnest($2::uuid[], $3::uuid[], $4::integer[])",
        bracket_tree_id,
        &team1,
        &team2,
        &position
    )
    .execute(&mut **transaction)
    .await
    .unwrap().rows_affected();

    if rows_affected as usize != size {
        tracing::error!("Number of inserted brackets is not equal to number of brackets that were supposed to be inserted!");
        return Err(());
    }
    Ok(())
}
