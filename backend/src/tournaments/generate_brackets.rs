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
        if tx.rollback().await.is_err() {
            tracing::error!("Could not rollback successfully");
        }
        return Err(());
    }

    let res = match tournament_type {
        TournamentType::FFA => generate_ffa_brackets(valid_teams, tournament_id, &mut tx).await,
        TournamentType::OneBracketTwoFinalPositions => {
            generate_one_bracket_two_final_positions_brackets(
                valid_teams,
                tournament_id,
                number_of_final_places,
                &mut tx,
            )
            .await
        }
        TournamentType::OneBracketOneFinalPositions => {
            generate_one_bracket_one_final_positions_brackets(
                valid_teams,
                tournament_id,
                number_of_final_places,
                &mut tx,
            )
            .await
        }
    };
    if res.is_err() {
        tracing::error!("Failed to generate brackets");
        if tx.rollback().await.is_err() {
            tracing::error!("Could not rollback successfully");
        }
        return Err(());
    }

    if tx.commit().await.is_err() {
        tracing::error!("Could not commit successfully");
        return Err(());
    }
    Ok(())
}

async fn generate_one_bracket_one_final_positions_brackets(
    teams: Vec<Uuid>,
    tournament_id: Uuid,
    number_of_final_places: u32,
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Result<(), ()> {
    let teams_len = teams.len();
    generate_tree_brackets(Some(teams), teams_len, tournament_id, 0, transaction).await?;
    let max_num_final_places = teams_len - 1;
    for position in 1..(number_of_final_places as usize).min(max_num_final_places) {
        generate_tree_brackets(
            None,
            teams_len - position,
            tournament_id,
            position as i32,
            transaction,
        )
        .await?;
    }
    Ok(())
}

async fn generate_one_bracket_two_final_positions_brackets(
    teams: Vec<Uuid>,
    tournament_id: Uuid,
    number_of_final_places: u32,
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Result<(), ()> {
    let teams_len = teams.len();
    generate_tree_brackets(Some(teams), teams_len, tournament_id, 0, transaction).await?;
    let max_num_trees = (teams_len as f32 / 2.0).ceil() as usize;
    let num_trees = (number_of_final_places as f32 / 2.0).ceil() as usize;
    for position in 1..(num_trees).min(max_num_trees) {
        generate_tree_brackets(
            None,
            teams_len - position * 2,
            tournament_id,
            position as i32,
            transaction,
        )
        .await?;
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
        "insert into brackets (bracket_tree_id, team1, team2, position, layer) select $1, *, 0 from unnest($2::uuid[], $3::uuid[], $4::integer[])",
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

async fn generate_tree_brackets(
    teams: Option<Vec<Uuid>>,
    no_teams: usize,
    tournament_id: Uuid,
    position: i32,
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Result<(), ()> {
    if no_teams == 0 {
        return Ok(());
    }

    let bracket_tree_id = query!(
        "insert into bracket_trees (tournament_id, position) values ($1, $2) returning id",
        tournament_id,
        position
    )
    .fetch_one(&mut **transaction)
    .await
    .unwrap()
    .id;

    let no_layers = (no_teams as f32).log2();
    let base_no_layers = if no_teams != 1 {
        no_layers.floor() as u32
    } else {
        1
    };
    let no_stickout = no_teams.saturating_sub(2usize.pow(base_no_layers));
    let teams = teams.unwrap_or_default();
    let mut teams_iter = teams.iter();

    let meet_point_pos = (no_stickout as f32 / 2.0).round() - 1.0;

    macro_rules! insert_bracket {
        ($layer:ident, $position:ident) => {
            let bracket_team2 = teams_iter.next();
            let bracket_team1 = teams_iter.next();
            let bracket_winner = match (bracket_team1, bracket_team2) {
                (Some(_), Some(_)) => None,
                (Some(_), None) | (None, Some(_)) if (meet_point_pos - $position as f32).abs() <= f32::EPSILON => None,
                (Some(_), None) => Some(true),
                (None, Some(_)) => Some(false),
                (None, None) => None,
            };
            query!(
                "insert into brackets (bracket_tree_id, team1, team2, layer, position, winner) values ($1, $2, $3, $4, $5, $6)",
                bracket_tree_id,
                bracket_team1,
                bracket_team2,
                $layer as i16,
                $position as i32,
                bracket_winner
            )
            .execute(&mut **transaction)
            .await
            .unwrap();
        };
    }

    for position in 0..no_stickout {
        insert_bracket!(base_no_layers, position);
    }

    for layer in (0..base_no_layers).rev() {
        let no_positions = 2usize.pow(layer);
        for position in (0..no_positions).rev() {
            insert_bracket!(layer, position);
        }
    }

    Ok(())
}
