use uuid::Uuid;

use crate::app::{default_err_fn, json_post, OnlyId};

#[derive(serde::Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub nick: String,
    pub email: String,
    pub roles: Vec<String>,
}

pub fn users_ui(ctx: &egui::Context, app: &mut crate::app::FrontendApp) {
    egui::Window::new("Users")
        .collapsible(true)
        .resizable(true)
        .show(ctx, |ui| {
            if ui.button("Reload").clicked() {
                app.users.start_download(
                    ehttp::Request::get(format!("{}/users", app.url)),
                    ctx.clone(),
                );
            }
            app.users.show_ui(
                ui,
                |ui, users| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for user in users {
                            egui::CollapsingHeader::new(&user.nick).show(ui, |ui| {
                                egui::Grid::new("user").show(ui, |ui| {
                                    ui.label("Id");
                                    ui.label(user.id.to_string());
                                    ui.end_row();

                                    ui.label("Nick");
                                    ui.label(&user.nick);
                                    ui.end_row();

                                    ui.label("Email");
                                    ui.label(&user.email);
                                    ui.end_row();

                                    ui.label("Roles");
                                    ui.label(format!("{:?}", user.roles));
                                    ui.end_row();
                                });
                                if app.is_tournament_manager() && ui.button("Delete").clicked() {
                                    ehttp::fetch(
                                        json_post(
                                            app.token.as_ref().unwrap(),
                                            &app.url,
                                            "/users/delete",
                                            &OnlyId { id: user.id },
                                        ),
                                        |_response| {},
                                    );
                                }
                            });
                        }
                    });
                },
                default_err_fn,
            );
        });
}
