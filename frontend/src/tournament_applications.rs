use uuid::Uuid;

use crate::{
    app::{default_err_fn, json_post, Nothing},
    downloadable::Downloadable,
    manipulator::ManipulatorTrait,
    teams, tournaments,
};

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
}

pub fn tournament_applications_ui(ctx: &egui::Context, app: &mut crate::app::FrontendApp) {
    egui::Window::new("Tournament applications")
        .collapsible(true)
        .resizable(true)
        .show(ctx, |ui| {
            if ui.button("Reload").clicked() {
                app.tournament_applications.start_download(
                    ehttp::Request::get(format!("{}/tournaments/team_applications", app.url)),
                    ctx.clone(),
                );
            }
            {
                let user = &*app.user.get_data();
                if app.token.is_some() && user.is_some() && ui.button("New").clicked() {
                    let user_id = user.as_ref().unwrap().id;
                    let tournaments = (*app.tournaments.get_data()).clone().unwrap_or_default();
                    let teams = (*app.teams.get_data())
                        .clone()
                        .unwrap_or_default()
                        .into_iter()
                        .filter(|t| t.managers.iter().any(|m| m.id == user_id))
                        .collect();
                    app.manipulator_window.set_editor(NewApplication::new(
                        app.token.clone().unwrap(),
                        app.url.clone(),
                        tournaments,
                        teams,
                    ));
                }
            }
            let is_tournament_manager = app.is_tournament_manager();
            app.tournament_applications.show_ui(
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
                                        #[derive(serde::Serialize)]
                                        struct ReqData {
                                            team_id: Uuid,
                                            tournament_id: Uuid,
                                            accepted: bool,
                                        }
                                        macro_rules! fire_req {
                                            ($accepted:expr) => {
                                                ehttp::fetch(
                                                    json_post(
                                                        app.token.as_ref().unwrap(),
                                                        &app.url,
                                                        "/tournaments/team_applications/handle",
                                                        &ReqData {
                                                            team_id: team.id,
                                                            tournament_id: tournament.id,
                                                            accepted: $accepted,
                                                        },
                                                    ),
                                                    |_response| {},
                                                );
                                            };
                                        }
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
                                            });

                                            if is_tournament_manager
                                                && ui.button("Accept").clicked()
                                            {
                                                fire_req!(true);
                                            }
                                            if is_tournament_manager
                                                && ui.button("Reject").clicked()
                                            {
                                                fire_req!(false);
                                            }

                                            let is_manager =
                                                {
                                                    let user = &*app.user.get_data();
                                                    let teams = &*app.teams.get_data();
                                                    let team = teams
                                                        .as_ref()
                                                        .map(|teams| {
                                                            teams
                                                                .iter()
                                                                .filter(|t| t.id == team.id)
                                                                .next()
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

struct NewApplication {
    data: NewApplicationData,
    tournaments: Vec<tournaments::Tournament>,
    teams: Vec<teams::Team>,
    token: String,
    public_url: String,
    req: Downloadable<Nothing>,
}

#[derive(serde::Serialize)]
struct NewApplicationData {
    team_id: Uuid,
    tournament_id: Uuid,
}

impl NewApplication {
    fn new(
        token: String,
        public_url: String,
        tournaments: Vec<tournaments::Tournament>,
        teams: Vec<teams::Team>,
    ) -> Self {
        Self {
            data: NewApplicationData {
                team_id: teams.first().map(|t| t.id).unwrap_or(Uuid::nil()),
                tournament_id: tournaments.first().map(|t| t.id).unwrap_or(Uuid::nil()),
            },
            tournaments: tournaments,
            teams,
            token,
            public_url,
            req: Downloadable::new(),
        }
    }
}

impl ManipulatorTrait for NewApplication {
    fn show_ui(&mut self, ui: &mut egui::Ui, ctx: egui::Context) {
        egui::ComboBox::new("Team", "Team")
            .selected_text(
                self.teams
                    .iter()
                    .filter(|t| t.id == self.data.team_id)
                    .next()
                    .map(|t| t.name.as_ref())
                    .unwrap_or("Not selected".into()),
            )
            .wrap(false)
            .show_ui(ui, |ui| {
                //ui.set_min_width(60.0);
                for team in &self.teams {
                    ui.selectable_value(&mut self.data.team_id, team.id, &team.name);
                }
            });
        egui::ComboBox::new("Tournament", "Tournament")
            .selected_text(
                self.tournaments
                    .iter()
                    .filter(|t| t.id == self.data.tournament_id)
                    .next()
                    .map(|t| t.name.as_ref())
                    .unwrap_or("Not selected".into()),
            )
            .wrap(false)
            .show_ui(ui, |ui| {
                //ui.set_min_width(60.0);
                for tournament in &self.tournaments {
                    ui.selectable_value(
                        &mut self.data.tournament_id,
                        tournament.id,
                        &tournament.name,
                    );
                }
            });
        if ui.button("Apply for").clicked() {
            let request = json_post(
                &self.token,
                &self.public_url,
                "/tournaments/apply_for",
                &self.data,
            );
            self.req.start_download(request, ctx.clone())
        }
        self.req.show_ui(
            ui,
            |ui, _| {
                ui.label("Successfully applied for tournament");
            },
            default_err_fn,
        );
    }
}
