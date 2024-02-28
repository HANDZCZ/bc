use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;

use crate::{
    downloadable::Downloadable,
    manipulator::ManipulatorWindow,
    pan_zoom::{PanZoom, PanZoomObject},
    teams,
    tournaments::{self, BracketManipulator, TournamentType},
    FrontendApp,
};

pub fn show_bracket_tree(
    app: &mut FrontendApp,
    tournament_name: String,
    bracket_tree_id: Uuid,
    position: i32,
    tournament_type: TournamentType,
    brackets: Vec<tournaments::Bracket>,
) {
    let brackets = brackets
        .into_iter()
        .map(|b| {
            app.show_brackets_linked_brackets
                .get(&(bracket_tree_id, b.layer, b.position))
                .unwrap()
                .clone()
        })
        .collect::<Vec<_>>();
    let manipulator_data = ManipulatorData {
        is_tournament_manager: app.is_tournament_manager(),
        teams: app.teams.clone(),
        manipulator_window: app.manipulator_window.clone(),
        token: app.token.clone(),
        url: app.url.clone(),
        bracket_tree_id,
    };

    let objects = match tournament_type {
        TournamentType::FFA => get_ffa_objects(
            brackets,
            manipulator_data,
            tournament_name.clone(),
            position,
        ),
        TournamentType::OneBracketTwoFinalPositions
        | TournamentType::OneBracketOneFinalPositions => get_tree_objects(
            brackets,
            manipulator_data,
            tournament_name.clone(),
            position,
        ),
    };

    let pan_zoom = PanZoom::new(
        format!("{} - Bracket tree ({})", tournament_name, position),
        objects,
    );
    app.pan_zooms.push(pan_zoom);
}

fn get_color(winner: Option<bool>) -> (egui::Color32, egui::Color32) {
    use egui::Color32;
    let winner_color = Color32::LIGHT_GREEN;
    let looser_color = Color32::LIGHT_RED;
    let default_color = Color32::PLACEHOLDER;
    match winner {
        Some(true) => (winner_color, looser_color),
        Some(false) => (looser_color, winner_color),
        None => (default_color, default_color),
    }
}

#[derive(Clone)]
struct ManipulatorData {
    is_tournament_manager: bool,
    teams: Downloadable<Vec<teams::Team>>,
    manipulator_window: ManipulatorWindow,
    token: Option<String>,
    url: String,
    bracket_tree_id: Uuid,
}

fn set_manipulator(
    data: &ManipulatorData,
    team1: Option<tournaments::Team>,
    team2: Option<tournaments::Team>,
    team1_score: i64,
    team2_score: i64,
    winner: Option<bool>,
    layer: u8,
    position: i32,
) {
    if data.is_tournament_manager {
        data.manipulator_window
            .set_editor(BracketManipulator::new_with_data(
                data.token.clone().unwrap(),
                data.url.clone(),
                data.teams.clone(),
                team1.as_ref().map(|t| t.id),
                team2.as_ref().map(|t| t.id),
                team1_score,
                team2_score,
                winner,
                data.bracket_tree_id,
                layer,
                position,
            ));
    }
}

fn get_ffa_objects(
    brackets: Vec<Rc<RefCell<tournaments::Bracket>>>,
    manipulator_data: ManipulatorData,
    tournament_name: String,
    bracket_tree_position: i32,
) -> Vec<Box<dyn PanZoomObject>> {
    struct FFABracket {
        bracket: Rc<RefCell<tournaments::Bracket>>,
        manipulator_data: Rc<ManipulatorData>,
        tournament_name: String,
        bracket_tree_position: i32,
    }
    impl PanZoomObject for FFABracket {
        fn id(&self) -> String {
            let bracket = self.bracket.borrow();
            format!(
                "{} ({}) - bracket - {}:{}",
                self.bracket_tree_position, self.tournament_name, bracket.layer, bracket.position
            )
        }

        fn pos(&self) -> egui::Pos2 {
            let bracket = self.bracket.borrow();
            egui::Pos2::new(0.0, bracket.position as f32 * 50.0)
        }

        fn ui(&mut self, ui: &mut egui::Ui) {
            let bracket = self.bracket.borrow();
            egui::Grid::new(format!(
                "{} ({}) - grid - {}:{}",
                self.bracket_tree_position, self.tournament_name, bracket.layer, bracket.position
            ))
            .show(ui, |ui| {
                let team1 = bracket
                    .team1
                    .as_ref()
                    .map(|t| t.name.as_str())
                    .unwrap_or_default();
                let team2 = bracket
                    .team2
                    .as_ref()
                    .map(|t| t.name.as_str())
                    .unwrap_or_default();
                let (team1_color, team2_color) = get_color(bracket.winner);

                ui.label(egui::RichText::new(team1).color(team1_color));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(bracket.team1_score.to_string());
                });
                ui.label(" ");
                ui.label(bracket.team2_score.to_string());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new(team2).color(team2_color));
                });
                if self.manipulator_data.is_tournament_manager && ui.button("Edit").clicked() {
                    set_manipulator(
                        &self.manipulator_data,
                        bracket.team1.clone(),
                        bracket.team2.clone(),
                        bracket.team1_score,
                        bracket.team2_score,
                        bracket.winner,
                        bracket.layer,
                        bracket.position,
                    )
                }
                ui.end_row();
            });
        }
    }
    let manipulator_data = Rc::new(manipulator_data);
    brackets
        .into_iter()
        .map(|bracket| {
            Box::new(FFABracket {
                bracket,
                manipulator_data: manipulator_data.clone(),
                tournament_name: tournament_name.clone(),
                bracket_tree_position,
            }) as Box<dyn PanZoomObject>
        })
        .collect()
}

fn get_tree_objects(
    brackets: Vec<Rc<RefCell<tournaments::Bracket>>>,
    manipulator_data: ManipulatorData,
    tournament_name: String,
    bracket_tree_position: i32,
) -> Vec<Box<dyn PanZoomObject>> {
    const BRACKET_WIDTH: f32 = 250.0;
    const BRACKET_HEIGHT: f32 = 60.0;
    const BRACKET_WIDTH_GAP: f32 = 80.0;
    const BRACKET_HEIGHT_GAP: f32 = 50.0;

    fn get_xy(layer: u8, position: i32, max_layer: u8) -> (f32, f32) {
        let mut x = -(layer as f32) * BRACKET_WIDTH - layer as f32 * BRACKET_WIDTH_GAP;
        let mut y = -position as f32 * BRACKET_HEIGHT - position as f32 * BRACKET_HEIGHT_GAP;

        let layer_delta = max_layer - layer;
        let base_move =
            (2.0f32.powi(layer_delta as i32) - 1.0) * (BRACKET_HEIGHT + BRACKET_HEIGHT_GAP) / 2.0;
        y -= position as f32 * base_move * 2.0 + base_move;

        // translate to (0,0) and flip on y
        x += max_layer as f32 * (BRACKET_WIDTH + BRACKET_WIDTH_GAP);
        y *= -1.0;
        y += 4.0;
        (x, y)
    }

    struct TreeBracket {
        bracket: Rc<RefCell<tournaments::Bracket>>,
        max_layer: u8,
        manipulator_data: Rc<ManipulatorData>,
        tournament_name: String,
        bracket_tree_position: i32,
    }
    impl PanZoomObject for TreeBracket {
        fn id(&self) -> String {
            let bracket = self.bracket.borrow();
            format!(
                "{} ({}) - bracket - {}:{}",
                self.bracket_tree_position, self.tournament_name, bracket.layer, bracket.position
            )
        }

        fn pos(&self) -> egui::Pos2 {
            let bracket = self.bracket.borrow();
            let (x, y) = get_xy(bracket.layer, bracket.position, self.max_layer);
            egui::pos2(x, y)
        }

        fn ui(&mut self, ui: &mut egui::Ui) {
            let bracket = self.bracket.borrow();
            ui.set_max_size(egui::vec2(BRACKET_WIDTH, BRACKET_HEIGHT));
            ui.set_min_size(egui::vec2(BRACKET_WIDTH, BRACKET_HEIGHT));
            egui::Grid::new(format!(
                "{} ({}) - grid - {}:{}",
                self.bracket_tree_position,
                self.tournament_name,
                bracket.layer,
                bracket.position
            ))
            .show(ui, |ui| {
                let team1 = bracket
                    .team1
                    .as_ref()
                    .map(|t| t.name.as_str())
                    .unwrap_or_default();
                let team2 = bracket
                    .team2
                    .as_ref()
                    .map(|t| t.name.as_str())
                    .unwrap_or_default();
                let (team1_color, team2_color) = get_color(bracket.winner);

                ui.allocate_ui(egui::vec2(BRACKET_WIDTH, BRACKET_HEIGHT / 3.0), |ui| {
                    ui.label(egui::RichText::new(team1).color(team1_color));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(bracket.team1_score.to_string());
                    });
                });
                ui.end_row();

                ui.allocate_ui(egui::vec2(BRACKET_WIDTH, BRACKET_HEIGHT / 3.0), |ui| {
                    ui.label(egui::RichText::new(team2).color(team2_color));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(bracket.team2_score.to_string());
                    });
                });
                ui.end_row();
                ui.allocate_ui(egui::vec2(BRACKET_WIDTH, BRACKET_HEIGHT / 3.0), |ui| {
                    ui.vertical_centered(|ui| {
                        if self.manipulator_data.is_tournament_manager
                            && ui.button("Edit").clicked()
                        {
                            set_manipulator(
                                &self.manipulator_data,
                                bracket.team1.clone(),
                                bracket.team2.clone(),
                                bracket.team1_score,
                                bracket.team2_score,
                                bracket.winner,
                                bracket.layer,
                                bracket.position,
                            )
                        }
                    });
                });
            });

            if bracket.layer != 0 {
                use egui::epaint::*;
                let painter = ui.painter();
                let mut points: [Pos2; 4] = [Pos2::ZERO; 4];
                points[0] = self.pos() + vec2(BRACKET_WIDTH + 16.0, (BRACKET_HEIGHT) / 2.0 + 8.0);
                let (ex, ey) = get_xy(
                    bracket.layer - 1,
                    (bracket.position as f32 / 2.0).floor() as i32,
                    self.max_layer,
                );
                points[3] = pos2(ex, ey + BRACKET_HEIGHT / 2.0 + 8.0);
                points[1] = pos2((points[0].x + points[3].x) / 2.0, points[0].y);
                points[2] = pos2((points[0].x + points[3].x) / 2.0, points[3].y);
                painter.add(CubicBezierShape::from_points_stroke(
                    points,
                    false,
                    Color32::TRANSPARENT,
                    Stroke::new(1.5, ui.style().visuals.weak_text_color()),
                ));
            }
        }
    }
    let max_layer = brackets.iter().map(|b| b.borrow().layer).max().unwrap_or(0);
    let manipulator_data = Rc::new(manipulator_data);
    brackets
        .into_iter()
        .map(|bracket| {
            Box::new(TreeBracket {
                bracket,
                max_layer,
                manipulator_data: manipulator_data.clone(),
                tournament_name: tournament_name.clone(),
                bracket_tree_position,
            }) as Box<dyn PanZoomObject>
        })
        .collect()
}
