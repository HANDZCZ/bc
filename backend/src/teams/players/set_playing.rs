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
    tournament_id: Uuid,
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
        "call set_playing($1, $2, $3, $4, $5)",
        user.id,
        data.team_id,
        data.tournament_id,
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
            if error.is_unique_violation() {
                let err = crate::common::Error::new("user is already playing for another team");
                resp_403_Forbidden_json!(err)
            } else if let Some(true) = error.code().map(|c| c == "66666") {
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

#[cfg(test)]
pub mod tests {
    use actix_web::test;

    use super::*;
    use crate::{common::TournamentType, tests::*};
    const URI: &str = "/teams/players/set_playing";

    #[actix_web::test]
    pub async fn test_ok() {
        let (app, rollbacker, pool) = get_test_app().await;
        let (auth_header, user_id) = new_user_insert_random(&app).await;
        let team_id = new_team_insert_random(user_id, &pool).await;
        ok_or_rollback_team!(team_id, rollbacker);
        let ins_res = new_player_to_team_insert(user_id, team_id, &pool).await;
        ok_or_rollback_player_to_team!(ins_res, _ins_res, rollbacker);
        let game_id = new_game_insert(&pool).await;
        ok_or_rollback_game!(game_id, rollbacker);
        let tournament_id =
            new_tournament_insert_random(game_id, false, false, TournamentType::FFA, &pool).await;
        ok_or_rollback_tournament!(tournament_id, rollbacker);

        let data = ToSet {
            team_id,
            tournament_id,
            players: vec![Player {
                id: user_id,
                playing: true,
            }],
        };
        let req = test::TestRequest::post()
            .uri(URI)
            .insert_header(auth_header)
            .set_json(data)
            .to_request();
        let resp = test::call_service(&app, req).await;
        let playing = sqlx::query!(
            "select exists(select * from players_to_tournaments_playing where tournament_id = $1 and player_id = $2)",
            tournament_id,
            user_id
        )
        .fetch_one(&pool)
        .await
        .unwrap()
        .exists
        .unwrap();

        rollbacker.rollback().await;
        assert_eq!(resp.status().as_u16(), 200);
        assert!(playing);
    }

    #[actix_web::test]
    pub async fn test_forbidden() {
        let (app, rollbacker, pool) = get_test_app().await;
        let (_auth_header, user_id) = new_user_insert_random(&app).await;
        let (other_auth_header, _other_user_id) = new_user_insert_random(&app).await;
        let team_id = new_team_insert_random(user_id, &pool).await;
        ok_or_rollback_team!(team_id, rollbacker);
        let game_id = new_game_insert(&pool).await;
        ok_or_rollback_game!(game_id, rollbacker);
        let tournament_id =
            new_tournament_insert_random(game_id, false, false, TournamentType::FFA, &pool).await;
        ok_or_rollback_tournament!(tournament_id, rollbacker);

        let data = ToSet {
            team_id,
            tournament_id,
            players: vec![Player {
                id: user_id,
                playing: true,
            }],
        };
        let req = test::TestRequest::post()
            .uri(URI)
            .insert_header(other_auth_header)
            .set_json(data)
            .to_request();
        let resp = test::call_service(&app, req).await;

        rollbacker.rollback().await;
        assert_eq!(resp.status().as_u16(), 403);
    }

    #[actix_web::test]
    pub async fn test_playing_in_another_team() {
        let (app, rollbacker, pool) = get_test_app().await;
        let (auth_header, user_id) = new_user_insert_random(&app).await;
        let team_id = new_team_insert_random(user_id, &pool).await;
        ok_or_rollback_team!(team_id, rollbacker);
        let ins_res = new_player_to_team_insert(user_id, team_id, &pool).await;
        ok_or_rollback_player_to_team!(ins_res, _ins_res, rollbacker);
        let game_id = new_game_insert(&pool).await;
        ok_or_rollback_game!(game_id, rollbacker);
        let tournament_id =
            new_tournament_insert_random(game_id, false, false, TournamentType::FFA, &pool).await;
        ok_or_rollback_tournament!(tournament_id, rollbacker);

        let data = ToSet {
            team_id,
            tournament_id,
            players: vec![Player {
                id: user_id,
                playing: true,
            }],
        };
        let req = test::TestRequest::post()
            .uri(URI)
            .insert_header(auth_header.clone())
            .set_json(data)
            .to_request();
        let resp = test::call_service(&app, req).await;
        let playing = sqlx::query!(
            "select exists(select * from players_to_tournaments_playing where tournament_id = $1 and player_id = $2)",
            tournament_id,
            user_id
        )
        .fetch_one(&pool)
        .await
        .unwrap()
        .exists
        .unwrap();

        assert_eq!(resp.status().as_u16(), 200);
        assert!(playing);

        let team_id = new_team_insert_random(user_id, &pool).await;
        ok_or_rollback_team!(team_id, rollbacker);
        let ins_res = new_player_to_team_insert(user_id, team_id, &pool).await;
        ok_or_rollback_player_to_team!(ins_res, _ins_res, rollbacker);

        let data = ToSet {
            team_id,
            tournament_id,
            players: vec![Player {
                id: user_id,
                playing: true,
            }],
        };
        let req = test::TestRequest::post()
            .uri(URI)
            .insert_header(auth_header)
            .set_json(data)
            .to_request();
        let resp = test::call_service(&app, req).await;
        rollbacker.rollback().await;
        assert_eq!(resp.status().as_u16(), 403);
    }
}
