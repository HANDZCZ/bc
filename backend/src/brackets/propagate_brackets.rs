use actix_web::web::Data;
use sqlx::{query, PgPool};
use uuid::Uuid;

use crate::common::TournamentType;

pub async fn propagate_brackets(
    bracket_tree_id: Uuid,
    layer: u8,
    position: i32,
    pool: Data<PgPool>,
) -> Result<(), ()> {
    let bracket_tree = query!(
        r#"select t.tournament_type as "tournament_type!: TournamentType", tournament_id, position as bracket_tree_position from bracket_trees join tournaments t on t.id = tournament_id where bracket_trees.id = $1"#,
        bracket_tree_id,
    )
    .fetch_one(pool.get_ref())
    .await;
    let Ok(bracket_tree) = bracket_tree else {
        return Err(());
    };
    if matches!(bracket_tree.tournament_type, TournamentType::FFA) {
        return Ok(());
    }
    let Ok(mut tx) = pool.get_ref().begin().await else {
        tracing::error!("Could not begin transaction");
        return Err(());
    };
    if propagate_bracket(
        bracket_tree_id,
        layer,
        position,
        &mut tx,
        bracket_tree.tournament_type,
        bracket_tree.bracket_tree_position,
        bracket_tree.tournament_id,
    )
    .await
    .is_err()
    {
        tracing::error!("Failed to propagate brackets");
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

async fn propagate_bracket(
    bracket_tree_id: Uuid,
    layer: u8,
    position: i32,
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    tournament_type: TournamentType,
    bracket_tree_position: i32,
    tournament_id: Uuid,
) -> Result<(), ()> {
    let bracket = query!(
        r#"select team1, team2, winner from brackets where bracket_tree_id = $1 and layer = $2 and position = $3"#,
        bracket_tree_id,
        layer as i16,
        position
    )
    .fetch_one(&mut **transaction)
    .await;
    let Ok(bracket) = bracket else {
        return if let Err(sqlx::Error::RowNotFound) = bracket {
            Ok(())
        } else {
            Err(())
        };
    };
    let (Some(team1), Some(team2), Some(winner)) = (bracket.team1, bracket.team2, bracket.winner)
    else {
        return Ok(());
    };

    // propagate forward in tree
    if layer > 0 {
        let team_to_set = if winner { team1 } else { team2 };
        let (team1, team2) = if position % 2 == 0 {
            (Some(team_to_set), None)
        } else {
            (None, Some(team_to_set))
        };

        let layer_to_set = layer - 1;
        let position_to_set = {
            let layer_max_positions = 2usize.pow(layer as u32);
            let next_layer_max_positions = 2usize.pow(layer_to_set as u32);
            let pos =
                (position as f32 / layer_max_positions as f32) * next_layer_max_positions as f32;
            pos.floor() as i32
        };
        let res = query!(
            "update brackets set team1 = coalesce($1, team1), team2 = coalesce($2, team2) where bracket_tree_id = $3 and layer = $4 and position = $5",
            team1,
            team2,
            bracket_tree_id,
            layer_to_set as i16,
            position_to_set,
        )
        .execute(&mut **transaction)
        .await;
        if res.is_err() {
            return Err(());
        }
    }

    // propagate to next tree
    {
        // no nothing to propagate
        if matches!(tournament_type, TournamentType::OneBracketTwoFinalPositions)
            && layer == 0
            && position == 0
        {
            return Ok(());
        }

        let new_bracket_tree_id = query!(
            r#"select id from bracket_trees where tournament_id = $1 and position = $2"#,
            tournament_id,
            bracket_tree_position + 1
        )
        .fetch_one(&mut **transaction)
        .await;
        let Ok(new_bracket_tree_id) = new_bracket_tree_id else {
            return if let Err(sqlx::Error::RowNotFound) = new_bracket_tree_id {
                Ok(())
            } else {
                Err(())
            };
        };
        let new_bracket_tree_id = new_bracket_tree_id.id;

        let max_layer = query!(
            r#"select max(layer) as max_layer from brackets where bracket_tree_id = $1"#,
            bracket_tree_id,
        )
        .fetch_one(&mut **transaction)
        .await;
        let Ok(max_layer) = max_layer else {
            return Err(());
        };
        let Some(max_layer) = max_layer.max_layer.map(|n| n as u8) else {
            return Err(());
        };
        let no_brackets = query!(
            r#"select max(position) + 1 as no_brackets from brackets where bracket_tree_id = $1 and layer = $2"#,
            bracket_tree_id,
            max_layer as i16
        )
        .fetch_one(&mut **transaction)
        .await;
        let Ok(no_brackets) = no_brackets else {
            return Err(());
        };
        let Some(no_brackets) = no_brackets.no_brackets.map(|n| n as u8) else {
            return Err(());
        };

        let max_no_dropouts_in_tree =
            1 + (1..max_layer).map(|l| 2usize.pow(l as u32)).sum::<usize>() + no_brackets as usize;
        let no_dropouts: usize = match tournament_type {
            TournamentType::FFA => unreachable!(),
            TournamentType::OneBracketTwoFinalPositions => max_no_dropouts_in_tree - 1,
            TournamentType::OneBracketOneFinalPositions => max_no_dropouts_in_tree,
        };
        if no_dropouts < 1 {
            return Ok(());
        }
        let dropout_idx = {
            if layer == max_layer {
                position as usize
            } else {
                no_brackets as usize
                    + (layer + 1..max_layer)
                        .map(|l| 2usize.pow(l as u32))
                        .sum::<usize>()
                    + position as usize
            }
        };
        let no_layers = (no_dropouts as f32).log2();
        let base_no_layers = if no_dropouts != 1 {
            no_layers.floor() as u32
        } else {
            1
        };
        let no_stickout = no_dropouts.saturating_sub(2usize.pow(base_no_layers));

        let max_dropout_idx = no_dropouts as f32 - 1.0;
        let inverted_dropout_idx = if max_dropout_idx == 0.0 {
            0.0
        } else {
            (1.0 - dropout_idx as f32 / max_dropout_idx) * max_dropout_idx
        };
        let inverted_dropout_idx = inverted_dropout_idx.round() as usize;

        let (layer_to_set, position_to_set, set_team1) = match tournament_type {
            TournamentType::FFA => unreachable!(),
            TournamentType::OneBracketOneFinalPositions
            | TournamentType::OneBracketTwoFinalPositions => {
                if no_stickout * 2 > inverted_dropout_idx {
                    (
                        base_no_layers,
                        (inverted_dropout_idx as f32 / 2.0).floor() as usize,
                        inverted_dropout_idx % 2 == 0,
                    )
                } else {
                    let max_layer_idx = 2usize.pow(base_no_layers as u32) - 1;
                    let idx = max_layer_idx - dropout_idx;
                    let position_to_set = (idx as f32 / 2.0).floor() as usize;

                    (
                        base_no_layers - 1,
                        position_to_set,
                        inverted_dropout_idx % 2 != 0,
                    )
                }
            }
        };
        let team_to_set = if !winner { team1 } else { team2 };
        let (team1, team2) = if set_team1 {
            (Some(team_to_set), None)
        } else {
            (None, Some(team_to_set))
        };
        let res = query!(
            "update brackets set team1 = coalesce($1, team1), team2 = coalesce($2, team2) where bracket_tree_id = $3 and layer = $4 and position = $5",
            team1,
            team2,
            new_bracket_tree_id,
            layer_to_set as i16,
            position_to_set as i32,
        )
        .execute(&mut **transaction)
        .await;
        if res.is_err() {
            return Err(());
        }
    }

    Ok(())
}
