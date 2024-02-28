use uuid::Uuid;

use crate::app::{default_err_fn, json_post};

#[derive(serde::Deserialize)]
pub struct Tournament {
    id: Uuid,
    teams: Vec<Team>,
}

#[derive(serde::Deserialize)]
struct Team {
    id: Uuid,
    name: String,
    description: String,
    num_playing: i32,
    valid_team_size: bool,
}

pub fn signed_up_teams_ui(ctx: &egui::Context, app: &mut crate::app::FrontendApp) {
    egui::Window::new("Signed up teams")
        .collapsible(true)
        .resizable(true)
        .show(ctx, |ui| {
            if ui.button("Reload").clicked() {
                app.signed_up_teams.start_download(
                    ehttp::Request::get(format!("{}/tournaments/signed_up_teams", app.url)),
                    ctx.clone(),
                );
            }
            let is_tournament_manager = app.is_tournament_manager();
            app.signed_up_teams.show_ui(
                ui,
                |ui, tournaments| {
                    egui::ScrollArea::both().show(ui, |ui| {
                        for tournament in tournaments {
                            let header_name = (*app.tournaments.get_data())
                                .as_ref()
                                .map(|tournaments| {
                                    tournaments
                                        .iter()
                                        .filter(|t| t.id == tournament.id)
                                        .map(|t| t.name.clone())
                                        .next()
                                })
                                .unwrap_or(None)
                                .unwrap_or(tournament.id.to_string());
                            egui::CollapsingHeader::new(header_name).show(ui, |ui| {
                                egui::Grid::new("tournament").show(ui, |ui| {
                                    ui.label("Id");
                                    ui.label(tournament.id.to_string());
                                    ui.end_row();
                                });
                                egui::CollapsingHeader::new("Teams").show(ui, |ui| {
                                    for team in &tournament.teams {
                                        egui::CollapsingHeader::new(&team.name).show(ui, |ui| {
                                            egui::Grid::new("team").show(ui, |ui| {
                                                ui.label("Id");
                                                ui.label(team.id.to_string());
                                                ui.end_row();

                                                ui.label("Name");
                                                ui.label(&team.name);
                                                ui.end_row();

                                                ui.label("Description");
                                                ui.label(&team.description);
                                                ui.end_row();

                                                ui.label("Num_playing");
                                                ui.label(team.num_playing.to_string());
                                                ui.end_row();

                                                ui.label("Valid_team_size");
                                                ui.label(team.valid_team_size.to_string());
                                                ui.end_row();
                                            });

                                            let is_manager =
                                                {
                                                    let user = &*app.user.get_data();
                                                    let teams = &*app.teams.get_data();
                                                    let team = teams
                                                        .as_ref()
                                                        .map(|teams| {
                                                            teams.iter().find(|t| t.id == team.id)
                                                        })
                                                        .unwrap_or(None);
                                                    user.is_some()
                                                        && team.is_some()
                                                        && team.unwrap().managers.iter().any(|m| {
                                                            m.id == user.as_ref().unwrap().id
                                                        })
                                                };
                                            #[derive(serde::Serialize)]
                                            struct ReqLeaveData {
                                                team_id: Uuid,
                                                tournament_id: Uuid,
                                            }

                                            macro_rules! fire_leave_req {
                                                () => {
                                                    ehttp::fetch(
                                                        json_post(
                                                            app.token.as_ref().unwrap(),
                                                            &app.url,
                                                            "/teams/leave_tournament",
                                                            &ReqLeaveData {
                                                                team_id: team.id,
                                                                tournament_id: tournament.id,
                                                            },
                                                        ),
                                                        |_response| {},
                                                    );
                                                };
                                            }
                                            if is_manager && ui.button("Leave tournament").clicked()
                                            {
                                                fire_leave_req!();
                                            }
                                            if is_tournament_manager
                                                && ui.button("Kick from tournament").clicked()
                                            {
                                                fire_leave_req!();
                                            }
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
