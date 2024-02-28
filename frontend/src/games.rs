use uuid::Uuid;

use crate::{
    app::{default_err_fn, json_post, Nothing, OnlyId},
    downloadable::Downloadable,
    manipulator::ManipulatorTrait,
};

#[derive(serde::Deserialize, Clone)]
pub struct Game {
    pub name: String,
    pub description: String,
    pub version: String,
    pub id: Uuid,
}

pub fn games_ui(ctx: &egui::Context, app: &mut crate::app::FrontendApp) {
    egui::Window::new("Games")
        .collapsible(true)
        .resizable(true)
        .show(ctx, |ui| {
            if ui.button("Reload").clicked() {
                app.games.start_download(
                    ehttp::Request::get(format!("{}/games", app.url)),
                    ctx.clone(),
                );
            }
            let is_tournament_manager = app.is_tournament_manager();
            if is_tournament_manager && ui.button("New").clicked() {
                app.manipulator_window.set_editor(GameManipulator::new(
                    app.token.clone().unwrap(),
                    app.url.clone(),
                ));
            }
            app.games.show_ui(
                ui,
                |ui, games| {
                    egui::ScrollArea::both().show(ui, |ui| {
                        for game in games {
                            egui::CollapsingHeader::new(&game.name).show(ui, |ui| {
                                egui::Grid::new("game").show(ui, |ui| {
                                    ui.label("Id");
                                    ui.label(game.id.to_string());
                                    ui.end_row();

                                    ui.label("Name");
                                    ui.label(&game.name);
                                    ui.end_row();

                                    ui.label("Version");
                                    ui.label(&game.version);
                                    ui.end_row();

                                    ui.label("Description");
                                    ui.label(&game.description);
                                    ui.end_row();
                                });
                                if is_tournament_manager {
                                    if ui.button("Edit").clicked() {
                                        app.manipulator_window.set_editor(
                                            GameManipulator::new_with_data(
                                                app.token.clone().unwrap(),
                                                app.url.clone(),
                                                game.id.to_string(),
                                                game.name.clone(),
                                                game.description.clone(),
                                                game.version.clone(),
                                            ),
                                        );
                                    }
                                    if ui.button("Delete").clicked() {
                                        ehttp::fetch(
                                            json_post(
                                                app.token.as_ref().unwrap(),
                                                &app.url,
                                                "/games/delete",
                                                &OnlyId { id: game.id },
                                            ),
                                            |_response| {},
                                        );
                                    }
                                }
                            });
                        }
                    });
                },
                default_err_fn,
            );
        });
}

struct GameManipulator {
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
    version: String,
}

impl GameManipulator {
    fn new(token: String, public_url: String) -> Self {
        Self {
            data: Data {
                id: String::new(),
                name: String::new(),
                description: String::new(),
                version: String::new(),
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
        version: String,
    ) -> Self {
        Self {
            data: Data {
                id,
                name,
                description,
                version,
            },
            token,
            public_url,
            new_req: Downloadable::new(),
            edit_req: Downloadable::new(),
        }
    }
}

impl ManipulatorTrait for GameManipulator {
    fn show_ui(&mut self, ui: &mut egui::Ui, ctx: egui::Context) {
        ui.add(egui::TextEdit::singleline(&mut self.data.name).hint_text("Name"));
        ui.add(egui::TextEdit::multiline(&mut self.data.description).hint_text("Description"));
        ui.add(egui::TextEdit::singleline(&mut self.data.version).hint_text("Version"));

        if ui.button("Create game").clicked() {
            self.new_req.start_download(
                json_post(&self.token, &self.public_url, "/games/new", &self.data),
                ctx.clone(),
            )
        }
        self.new_req.show_ui(
            ui,
            |ui, game| {
                ui.label(format!("Created game with id: {}", game.id));
            },
            default_err_fn,
        );
        if ui.button("Edit game").clicked() {
            self.edit_req.start_download(
                json_post(&self.token, &self.public_url, "/games/edit", &self.data),
                ctx,
            )
        }
        self.edit_req.show_ui(
            ui,
            |ui, _| {
                ui.label("Edited game".to_string());
            },
            default_err_fn,
        );
    }
}
