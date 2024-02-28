use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use sqlx::{query, PgPool};

use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;
use uuid::Uuid;

use crate::{
    common::TournamentType,
    jwt_stuff::LoggedInUserWithAuthorities,
    macros::{
        check_user_authority, resp_200_Ok_json, resp_400_BadReq_json, resp_500_IntSerErr_json,
    },
    tournaments::generate_brackets::generate_brackets,
};

#[derive(Serialize, Deserialize)]
struct Tournament {
    id: Uuid,
    name: Option<String>,
    description: Option<String>,
    game_id: Option<Uuid>,
    min_team_size: Option<i32>,
    max_team_size: Option<i32>,
    requires_application: Option<bool>,
    applications_closed: Option<ApplicationsClosed>,
    tournament_type: Option<TournamentType>,
}

#[derive(Serialize, Deserialize)]
enum ApplicationsClosed {
    True {
        #[serde(default = "default_number_of_final_places")]
        number_of_final_places: NonZeroU32,
        #[serde(default)]
        regenerate_brackets: bool,
    },
    False,
}

fn default_number_of_final_places() -> NonZeroU32 {
    NonZeroU32::new(3).unwrap()
}

#[derive(Serialize, Deserialize)]
struct RowsAffected {
    rows_affected: u64,
}

#[post("/edit")]
pub async fn edit(
    pool: Data<PgPool>,
    data: Json<Tournament>,
    user: LoggedInUserWithAuthorities,
) -> impl Responder {
    check_user_authority!(user, "role::Tournament Manager");

    match query!(
        r#"update tournaments set
        name = coalesce($1, name),
        description = coalesce($2, description),
        game_id = coalesce($3, game_id),
        max_team_size = coalesce($4, max_team_size),
        min_team_size = coalesce($8, min_team_size),
        requires_application = coalesce($5, requires_application),
        applications_closed = coalesce($6, applications_closed),
        tournament_type = coalesce($9, tournament_type)
        where id = $7"#,
        data.name,
        data.description,
        data.game_id,
        data.max_team_size,
        data.requires_application,
        data.applications_closed
            .as_ref()
            .map(|s| !matches!(s, ApplicationsClosed::False)),
        data.id,
        data.min_team_size,
        data.tournament_type as Option<TournamentType>
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(query_result) => {
            // generate brackets
            if let Some(ApplicationsClosed::True {
                number_of_final_places,
                regenerate_brackets,
            }) = data.applications_closed.as_ref()
            {
                match query!(
                    r#"select exists(select id from bracket_trees where tournament_id = $1) as "exists!""#,
                    data.id,
                )
                .fetch_one(pool.get_ref())
                .await
                .map(|r| r.exists)
                {
                    // (re)generate brackets
                    Ok(exists) if !exists || *regenerate_brackets => {
                        if generate_brackets(data.id, pool, *number_of_final_places).await.is_err() {
                            let err = crate::common::Error::new(
                                "Edit was successful, but bracket generation failed!",
                            );
                            return resp_500_IntSerErr_json!(err);
                        }
                    }
                    // exists && !regenerate_brackets
                    Ok(_) => {
                        let err = crate::common::Error::new("Tournament already has brackets generated. If you wish to regenerate them set 'regenerate_brackets' to true.");
                        return resp_400_BadReq_json!(err);
                    }
                    Err(sqlx::Error::Database(error)) => {
                        let err = crate::common::Error::new(format!("unhandled error - {}", error));
                        return resp_400_BadReq_json!(err);
                    }
                    Err(_) => return resp_500_IntSerErr_json!(),
                }
            }

            let rows_affected = RowsAffected {
                rows_affected: query_result.rows_affected(),
            };
            resp_200_Ok_json!(rows_affected)
        }
        Err(sqlx::Error::Database(error)) => {
            if error.is_unique_violation() {
                let err = crate::common::Error::new(
                    "request for tournament edit violates unique constraints",
                );
                resp_400_BadReq_json!(err)
            } else if error.is_foreign_key_violation() {
                let err = crate::common::Error::new("request for tournament edit violates foreign key constraints (game id doesn't exists)");
                resp_400_BadReq_json!(err)
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
mod tests {
    use actix_web::test::{self, read_body_json};

    use super::*;
    use crate::tests::*;
    const URI: &str = "/tournaments/edit";

    #[actix_web::test]
    async fn test_forbidden() {
        let data = Tournament {
            id: Uuid::new_v4(),
            name: None,
            description: None,
            game_id: None,
            max_team_size: None,
            min_team_size: None,
            requires_application: None,
            applications_closed: None,
            tournament_type: None,
        };

        let (app, rollbacker, _pool) = get_test_app().await;
        let reg_user_header = get_regular_users_auth_header(&app).await;

        let req = test::TestRequest::post()
            .uri(URI)
            .insert_header(reg_user_header)
            .set_json(data)
            .to_request();
        let resp = test::call_service(&app, req).await;

        rollbacker.rollback().await;
        assert_eq!(resp.status().as_u16(), 403);
    }

    #[actix_web::test]
    async fn test_ok() {
        let (app, rollbacker, pool) = get_test_app().await;
        let auth_header = get_tournament_managers_auth_header(&app).await;

        let game_id = new_game_insert(&pool).await;
        ok_or_rollback_game!(game_id, rollbacker);
        let id =
            new_tournament_insert_random(game_id, false, false, TournamentType::FFA, &pool).await;
        ok_or_rollback_tournament!(id, rollbacker);

        let data = Tournament {
            id,
            name: Some("test-tournament-edited".to_owned()),
            description: None,
            game_id: None,
            min_team_size: None,
            max_team_size: None,
            requires_application: None,
            applications_closed: None,
            tournament_type: None,
        };

        let req = test::TestRequest::post()
            .uri(URI)
            .insert_header(auth_header)
            .set_json(data)
            .to_request();
        let resp = test::call_service(&app, req).await;

        rollbacker.rollback().await;
        assert_eq!(resp.status().as_u16(), 200);
        let res: RowsAffected = read_body_json(resp).await;
        assert_eq!(res.rows_affected, 1);
    }
}
