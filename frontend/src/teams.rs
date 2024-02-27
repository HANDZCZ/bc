use uuid::Uuid;

use crate::{
    app::{default_err_fn, json_post, Nothing, OnlyId},
    downloadable::Downloadable,
    manipulator::ManipulatorTrait,
};

#[derive(serde::Deserialize, Clone)]
pub struct Team {
    pub name: String,
    pub description: String,
    pub id: Uuid,
    pub players: Vec<User>,
    pub managers: Vec<User>,
}

#[derive(serde::Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub nick: String,
}

pub fn teams_ui(ctx: &egui::Context, app: &mut crate::app::FrontendApp) {
    egui::Window::new("Teams")
        .collapsible(true)
        .resizable(true)
        .show(ctx, |ui| {
            if ui.button("Reload").clicked() {
                app.teams.start_download(
                    ehttp::Request::get(format!("{}/teams/with_players_and_managers", app.url)),
                    ctx.clone(),
                );
            }
            if app.token.is_some() && ui.button("New").clicked() {
                app.manipulator_window.set_editor(TeamManipulator::new(
                    app.token.clone().unwrap(),
                    app.url.clone(),
                ));
            }
            let is_tournament_manager = app.is_tournament_manager();
            app.teams.show_ui(
                ui,
                |ui, teams| {
                    egui::ScrollArea::both().show(ui, |ui| {
                        for team in teams {
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
                                let user = &*app.user.get_data();
                                let is_manager = user.is_some()
                                    && team
                                        .managers
                                        .iter()
                                        .any(|m| m.id == user.as_ref().unwrap().id);
                                if is_manager && ui.button("Edit").clicked() {
                                    app.manipulator_window.set_editor(
                                        TeamManipulator::new_with_data(
                                            app.token.clone().unwrap(),
                                            app.url.clone(),
                                            team.id.to_string(),
                                            team.name.clone(),
                                            team.description.clone(),
                                        ),
                                    );
                                }
                                if (is_manager || is_tournament_manager)
                                    && ui.button("Delete").clicked()
                                {
                                    ehttp::fetch(
                                        json_post(
                                            app.token.as_ref().unwrap(),
                                            &app.url,
                                            "/teams/delete",
                                            &OnlyId { id: team.id },
                                        ),
                                        |_response| {},
                                    );
                                }
                                egui::CollapsingHeader::new("Players").show(ui, |ui| {
                                    egui::Grid::new("player").show(ui, |ui| {
                                        for player in &team.players {
                                            ui.label("Nick");
                                            ui.label(&player.nick);
                                            ui.end_row();

                                            ui.label("Id");
                                            ui.label(player.id.to_string());
                                            ui.end_row();

                                            #[derive(serde::Serialize)]
                                            struct ReqData {
                                                team_id: Uuid,
                                                player_ids: Vec<Uuid>,
                                            }
                                            if is_manager && ui.button("Remove").clicked() {
                                                ehttp::fetch(
                                                    json_post(
                                                        app.token.as_ref().unwrap(),
                                                        &app.url,
                                                        "/teams/players/remove",
                                                        &ReqData {
                                                            team_id: team.id,
                                                            player_ids: vec![player.id],
                                                        },
                                                    ),
                                                    |_response| {},
                                                );
                                            }
                                            ui.end_row();
                                        }
                                    });
                                });
                                egui::CollapsingHeader::new("Managers").show(ui, |ui| {
                                    egui::Grid::new("manager").show(ui, |ui| {
                                        for manager in &team.managers {
                                            ui.label("Id");
                                            ui.label(manager.id.to_string());
                                            ui.end_row();

                                            ui.label("Nick");
                                            ui.label(&manager.nick);
                                            ui.end_row();

                                            #[derive(serde::Serialize)]
                                            struct ReqData {
                                                team_id: Uuid,
                                                manager_ids: Vec<Uuid>,
                                            }
                                            if is_manager && ui.button("Remove").clicked() {
                                                ehttp::fetch(
                                                    json_post(
                                                        app.token.as_ref().unwrap(),
                                                        &app.url,
                                                        "/teams/managers/remove",
                                                        &ReqData {
                                                            team_id: team.id,
                                                            manager_ids: vec![manager.id],
                                                        },
                                                    ),
                                                    |_response| {},
                                                );
                                            }
                                            ui.end_row();
                                        }
                                    });
                                });
                            });
                        }
                    });
                },
                default_err_fn,
            );
        });
}

struct TeamManipulator {
    data: Data,
    token: String,
    public_url: String,
    new_req: Downloadable<OnlyId>,
    edit_req: Downloadable<Nothing>,
}

#[derive(serde::Serialize)]
struct Data {
    id: String,
    name: String,
    description: String,
}

impl TeamManipulator {
    fn new(token: String, public_url: String) -> Self {
        Self {
            data: Data {
                id: String::new(),
                name: String::new(),
                description: String::new(),
            },
            token,
            public_url,
            new_req: Downloadable::new(),
            edit_req: Downloadable::new(),
        }
    }

    fn new_with_data(
        token: String,
        public_url: String,
        id: String,
        name: String,
        description: String,
    ) -> Self {
        Self {
            data: Data {
                id,
                name,
                description,
            },
            token,
            public_url,
            new_req: Downloadable::new(),
            edit_req: Downloadable::new(),
        }
    }
}

impl ManipulatorTrait for TeamManipulator {
    fn show_ui(&mut self, ui: &mut egui::Ui, ctx: egui::Context) {
        ui.add(egui::TextEdit::singleline(&mut self.data.name).hint_text("Name"));
        ui.add(egui::TextEdit::multiline(&mut self.data.description).hint_text("Description"));

        if ui.button("Create team").clicked() {
            self.new_req.start_download(
                json_post(&self.token, &self.public_url, "/teams/new", &self.data),
                ctx.clone(),
            )
        }
        self.new_req.show_ui(
            ui,
            |ui, team| {
                ui.label(format!("Created team with id: {}", team.id));
            },
            default_err_fn,
        );
        if ui.button("Edit team").clicked() {
            self.edit_req.start_download(
                json_post(&self.token, &self.public_url, "/teams/edit", &self.data),
                ctx,
            )
        }
        self.edit_req.show_ui(
            ui,
            |ui, _| {
                ui.label(format!("Edited team"));
            },
            default_err_fn,
        );
    }
}
