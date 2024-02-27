use uuid::Uuid;

use crate::app::{default_err_fn, json_post};

#[derive(serde::Deserialize)]
pub struct Data {
    tournament_id: Uuid,
    tournament_name: String,
    players: Vec<Player>,
}

#[derive(serde::Deserialize)]
pub struct Player {
    player_id: Uuid,
    nick: String,
    playing: bool,
}

pub fn teams_playing_players_ui(ctx: &egui::Context, app: &mut crate::app::FrontendApp) {
    egui::Window::new("Teams playing players")
        .collapsible(true)
        .resizable(true)
        .show(ctx, |ui| {
            if ui.button("Reload").clicked() {
                app.teams_playing_players.start_download(
                    ehttp::Request::get(format!(
                        "{}/teams/players/playing/{}",
                        app.url, app.teams_playing_players_team_id
                    )),
                    ctx.clone(),
                );
            }
            ui.add(
                egui::TextEdit::singleline(&mut app.teams_playing_players_team_id)
                    .hint_text("Team uuid"),
            );

            let is_manager = {
                let user = &*app.user.get_data();
                let teams = &*app.teams.get_data();
                let is_loaded_team = app
                    .teams_playing_players
                    .get_response()
                    .map(|r| r.url.contains(&app.teams_playing_players_team_id))
                    .unwrap_or(false);
                if is_loaded_team {
                    let team = teams
                        .as_ref()
                        .map(|teams| {
                            teams
                                .iter()
                                .filter(|t| t.id.to_string() == app.teams_playing_players_team_id)
                                .next()
                        })
                        .unwrap_or(None);
                    user.is_some()
                        && team.is_some()
                        && team
                            .unwrap()
                            .managers
                            .iter()
                            .any(|m| m.id == user.as_ref().unwrap().id)
                } else {
                    false
                }
            };

            app.teams_playing_players.show_ui(
                ui,
                |ui, data| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for row in data {
                            egui::CollapsingHeader::new(&row.tournament_name).show(ui, |ui| {
                                egui::Grid::new("row").show(ui, |ui| {
                                    ui.label("Tournament_id");
                                    ui.label(row.tournament_id.to_string());
                                    ui.end_row();

                                    ui.label("Tournament_name");
                                    ui.label(&row.tournament_name);
                                    ui.end_row();
                                });

                                if is_manager && ui.button("Leave tournament").clicked() {
                                    #[derive(serde::Serialize)]
                                    struct ReqData {
                                        team_id: String,
                                        tournament_id: Uuid,
                                    }

                                    ehttp::fetch(
                                        json_post(
                                            app.token.as_ref().unwrap(),
                                            &app.url,
                                            "/teams/leave_tournament",
                                            &ReqData {
                                                team_id: app.teams_playing_players_team_id.clone(),
                                                tournament_id: row.tournament_id,
                                            },
                                        ),
                                        |_response| {},
                                    );
                                }
                                egui::CollapsingHeader::new("Players").show(ui, |ui| {
                                    for player in &row.players {
                                        egui::CollapsingHeader::new(&player.nick).show(ui, |ui| {
                                            egui::Grid::new("player").show(ui, |ui| {
                                                ui.label("Player_id");
                                                ui.label(player.player_id.to_string());
                                                ui.end_row();

                                                ui.label("Nick");
                                                ui.label(&player.nick);
                                                ui.end_row();

                                                ui.label("Playing");
                                                ui.label(player.playing.to_string());
                                                ui.end_row();

                                                #[derive(serde::Serialize)]
                                                struct ReqData {
                                                    players: Vec<ReqPlayer>,
                                                    team_id: String,
                                                    tournament_id: Uuid,
                                                }
                                                #[derive(serde::Serialize)]
                                                struct ReqPlayer {
                                                    id: Uuid,
                                                    playing: bool,
                                                }

                                                if is_manager
                                                    && ui.button("Toggle playing").clicked()
                                                {
                                                    ehttp::fetch(
                                                        json_post(
                                                            app.token.as_ref().unwrap(),
                                                            &app.url,
                                                            "/teams/players/set_playing",
                                                            &ReqData {
                                                                team_id: app
                                                                    .teams_playing_players_team_id
                                                                    .clone(),
                                                                tournament_id: row.tournament_id,
                                                                players: vec![ReqPlayer {
                                                                    id: player.player_id,
                                                                    playing: !player.playing,
                                                                }],
                                                            },
                                                        ),
                                                        |_response| {},
                                                    );
                                                }
                                            });
                                        });
                                    }
                                });
                            });
                        }
                    });
                },
                default_err_fn,
            );
        });
}
