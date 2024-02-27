use uuid::Uuid;

use crate::app::default_err_fn;

#[derive(serde::Deserialize)]
pub struct Data {
    team_id: Uuid,
    team_name: String,
    players: Vec<Player>,
}

#[derive(serde::Deserialize)]
pub struct Player {
    player_id: Uuid,
    nick: String,
    playing: bool,
}

pub fn tournaments_playing_players_ui(ctx: &egui::Context, app: &mut crate::app::FrontendApp) {
    egui::Window::new("Tournaments playing players")
        .collapsible(true)
        .resizable(true)
        .show(ctx, |ui| {
            if ui.button("Reload").clicked() {
                app.tournaments_playing_players.start_download(
                    ehttp::Request::get(format!(
                        "{}/tournaments/playing/{}",
                        app.url, app.tournaments_playing_players_tournament_id
                    )),
                    ctx.clone(),
                );
            }
            ui.add(
                egui::TextEdit::singleline(&mut app.tournaments_playing_players_tournament_id)
                    .hint_text("Tournament uuid"),
            );

            app.tournaments_playing_players.show_ui(
                ui,
                |ui, data| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for row in data {
                            egui::CollapsingHeader::new(&row.team_name).show(ui, |ui| {
                                egui::Grid::new("row").show(ui, |ui| {
                                    ui.label("Team_id");
                                    ui.label(row.team_id.to_string());
                                    ui.end_row();

                                    ui.label("Team_name");
                                    ui.label(&row.team_name);
                                    ui.end_row();
                                });
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
