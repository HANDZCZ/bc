use uuid::Uuid;

use crate::{
    app::{default_err_fn, json_post, Nothing, OnlyId},
    downloadable::Downloadable,
    games,
    manipulator::ManipulatorTrait,
    show_brackets::show_bracket_tree,
    teams,
};

#[derive(serde::Deserialize, Clone)]
pub struct Tournament {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub min_team_size: i32,
    pub max_team_size: i32,
    pub requires_application: bool,
    pub applications_closed: bool,
    pub tournament_type: TournamentType,

    pub game_id: Uuid,
    pub game_name: String,
    pub game_description: String,
    pub game_version: String,

    pub bracket_trees: Vec<BracketTree>,
}

#[derive(serde::Deserialize, Clone)]
pub struct BracketTree {
    pub id: Uuid,
    pub position: i32,
    pub brackets: Vec<Bracket>,
}

#[derive(serde::Deserialize, Clone)]
pub struct Bracket {
    pub team1: Option<Team>,
    pub team2: Option<Team>,
    pub winner: Option<bool>,
    pub team1_score: i64,
    pub team2_score: i64,
    pub layer: u8,
    pub position: i32,
}

#[derive(serde::Deserialize, Clone)]
pub struct Team {
    pub id: Uuid,
    pub name: String,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[allow(clippy::upper_case_acronyms)]
pub enum TournamentType {
    FFA,
    OneBracketTwoFinalPositions,
    OneBracketOneFinalPositions,
}

pub fn tournaments_ui(ctx: &egui::Context, app: &mut crate::app::FrontendApp) {
    egui::Window::new("Tournaments")
        .collapsible(true)
        .resizable(true)
        .show(ctx, |ui| {
            if ui.button("Reload").clicked() {
                app.tournaments.start_download(
                    ehttp::Request::get(format!("{}/tournaments/with_bracket_trees", app.url)),
                    ctx.clone(),
                );
            }
            let is_tournament_manager = app.is_tournament_manager();
            if is_tournament_manager && ui.button("New").clicked() {
                app.manipulator_window.set_editor(TournamentManipulator::new(app.token.clone().unwrap(), app.url.clone(), app.games.clone()));
            }
            let mut tree_to_show = None;
            app.tournaments.show_ui(
                ui,
                |ui, tournament| {
                    egui::ScrollArea::both().show(ui, |ui| {
                    for tournament in tournament {
                        egui::CollapsingHeader::new(&tournament.name).show(ui, |ui| {
                            egui::Grid::new("tournament").show(ui, |ui| {
                                macro_rules! struct_fields {
                                    ($($f:ident),+) => {
                                        $(
                                        let s = stringify!($f);
                                        let s: String = s.chars()
                                            .take(1)
                                            .flat_map(|f| f.to_uppercase())
                                            .chain(s.chars().skip(1))
                                            .collect();
                                        ui.label(s);
                                        ui.label(format!("{:?}", tournament.$f).replace("\"", ""));
                                        ui.end_row();
                                        )+
                                    };
                                }

                                struct_fields!(
                                    id,
                                    name,
                                    description,
                                    min_team_size,
                                    max_team_size,
                                    requires_application,
                                    applications_closed,
                                    tournament_type,
                                    game_id,
                                    game_name,
                                    game_description,
                                    game_version
                                );
                            });
                            if is_tournament_manager {
                                if ui.button("Edit").clicked() {
                                    app.manipulator_window.set_editor(TournamentManipulator::new_with_data(
                                        app.token.clone().unwrap(),
                                        app.url.clone(),
                                        app.games.clone(),
                                        tournament.id,
                                        tournament.name.clone(),
                                        tournament.description.clone(),
                                        tournament.game_id,
                                        tournament.min_team_size,
                                        tournament.max_team_size,
                                        tournament.requires_application,
                                        tournament.applications_closed,
                                        tournament.tournament_type
                                    ));
                                }
                                if ui.button("Delete").clicked() {
                                    ehttp::fetch(
                                        json_post(
                                            app.token.as_ref().unwrap(),
                                            &app.url,
                                            "/tournaments/delete",
                                            &OnlyId { id: tournament.id },
                                        ),
                                        |_response| {},
                                    );
                                }
                            }
                            egui::CollapsingHeader::new("Bracket trees").show(ui, |ui| {
                                for bracket_tree in &tournament.bracket_trees {
                                    egui::CollapsingHeader::new(format!(
                                        "Bracket tree - {}",
                                        bracket_tree.position
                                    ))
                                    .show(ui, |ui| {
                                        egui::Grid::new("bracket_tree").show(ui, |ui| {
                                            ui.label("Id");
                                            ui.label(bracket_tree.id.to_string());
                                            ui.end_row();

                                            ui.label("Position");
                                            ui.label(bracket_tree.position.to_string());
                                            ui.end_row();
                                        });
                                        if ui.button("Show tree").clicked() {
                                            tree_to_show = Some((tournament.name.clone(), bracket_tree.id, bracket_tree.position, tournament.tournament_type, bracket_tree.brackets.clone()));
                                        }

                                        egui::CollapsingHeader::new("Brackets").show(ui, |ui| {
                                            for bracket in &bracket_tree.brackets {
                                                egui::CollapsingHeader::new(format!(
                                                    "Bracket - {}:{}",
                                                    bracket.layer, bracket.position
                                                ))
                                                .show(ui, |ui| {
                                                    egui::Grid::new("bracket").show(ui, |ui| {
                                                        macro_rules! struct_fields {
                                                           ($($f:ident),+) => {
                                                               $(
                                                               let s = stringify!($f);
                                                               let s: String = s.chars()
                                                                   .take(1)
                                                                   .flat_map(|f| f.to_uppercase())
                                                                   .chain(s.chars().skip(1))
                                                                   .collect();
                                                               ui.label(s);
                                                               ui.label(format!("{:?}", bracket.$f));
                                                               ui.end_row();
                                                               )+
                                                            };
                                                        }

                                                        ui.label("Team1 - id");
                                                        ui.label(bracket.team1.as_ref().map(|team| team.id.to_string()).unwrap_or("None".into()));
                                                        ui.end_row();

                                                        ui.label("Team1 - name");
                                                        ui.label(bracket.team1.as_ref().map(|team| team.name.as_str()).unwrap_or("None"));
                                                        ui.end_row();

                                                        ui.label("Team2 - id");
                                                        ui.label(bracket.team2.as_ref().map(|team| team.id.to_string()).unwrap_or("None".into()));
                                                        ui.end_row();

                                                        ui.label("Team2 - name");
                                                        ui.label(bracket.team2.as_ref().map(|team| team.name.as_str()).unwrap_or("None"));
                                                        ui.end_row();

                                                        ui.label("Winner");
                                                        ui.label(bracket.winner.map(|w| if w { "Team 1" } else { "Team 2" }).unwrap_or("None"));
                                                        ui.end_row();

                                                        struct_fields!(team1_score, team2_score, layer, position);

                                                        if is_tournament_manager && ui.button("Edit").clicked() {
                                                            app.manipulator_window.set_editor(BracketManipulator::new_with_data(
                                                          app.token.clone().unwrap(),
                                                     app.url.clone(),
                                                                app.teams.clone(),
                                                                bracket.team1.as_ref().map(|t| t.id),
                                                                bracket.team2.as_ref().map(|t| t.id),
                                                                bracket.team1_score,
                                                                bracket.team2_score,
                                                                bracket.winner,
                                                                bracket_tree.id,
                                                                bracket.layer,
                                                                bracket.position
                                                            ));
                                                        }
                                                    });
                                                });
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
            if let Some((tournament_name, bracket_tree_id, position, tournament_type, brackets)) = tree_to_show {
                show_bracket_tree(app, tournament_name, bracket_tree_id, position, tournament_type, brackets);
            }
        });
}

struct TournamentManipulator {
    data: Data,
    token: String,
    games: Downloadable<Vec<games::Game>>,
    public_url: String,
    new_req: Downloadable<OnlyId>,
    edit_req: Downloadable<Nothing>,

    send_applications_closed: bool,
    applications_closed: bool,
    number_of_final_places: u32,
    regenerate_brackets: bool,
}

#[derive(serde::Serialize)]
struct Data {
    id: Uuid,
    name: String,
    description: String,
    game_id: Uuid,
    min_team_size: i32,
    max_team_size: i32,
    requires_application: bool,
    applications_closed: Option<ApplicationsClosed>,
    tournament_type: TournamentType,
}

#[derive(serde::Serialize)]
enum ApplicationsClosed {
    True {
        number_of_final_places: u32,
        regenerate_brackets: bool,
    },
    False,
}

impl TournamentManipulator {
    fn new(token: String, public_url: String, games: Downloadable<Vec<games::Game>>) -> Self {
        let game_id = {
            (*games.get_data())
                .as_ref()
                .map(|games| games.first().map(|g| g.id).unwrap_or(Uuid::nil()))
                .unwrap_or(Uuid::nil())
        };
        Self {
            data: Data {
                id: Uuid::nil(),
                name: String::new(),
                description: String::new(),
                game_id,
                min_team_size: 0,
                max_team_size: 0,
                requires_application: true,
                applications_closed: None,
                tournament_type: TournamentType::FFA,
            },

            send_applications_closed: false,
            applications_closed: false,
            number_of_final_places: 3,
            regenerate_brackets: false,

            games,
            token,
            public_url,
            new_req: Downloadable::new(),
            edit_req: Downloadable::new(),
        }
    }

    fn new_with_data(
        token: String,
        public_url: String,
        games: Downloadable<Vec<games::Game>>,
        id: Uuid,
        name: String,
        description: String,
        game_id: Uuid,
        min_team_size: i32,
        max_team_size: i32,
        requires_application: bool,
        applications_closed: bool,
        tournament_type: TournamentType,
    ) -> Self {
        Self {
            data: Data {
                id,
                name,
                description,
                game_id,
                min_team_size,
                max_team_size,
                requires_application,
                applications_closed: None,
                tournament_type,
            },

            send_applications_closed: false,
            applications_closed,
            number_of_final_places: 3,
            regenerate_brackets: false,

            token,
            public_url,
            games,
            new_req: Downloadable::new(),
            edit_req: Downloadable::new(),
        }
    }
}

impl ManipulatorTrait for TournamentManipulator {
    fn show_ui(&mut self, ui: &mut egui::Ui, ctx: egui::Context) {
        let games = self.games.get_data();
        let empty_vec = Vec::new();
        let games = (*games).as_ref().unwrap_or(&empty_vec);
        ui.add(egui::TextEdit::singleline(&mut self.data.name).hint_text("Name"));
        ui.add(egui::TextEdit::multiline(&mut self.data.description).hint_text("Description"));
        egui::ComboBox::new("Game", "Game")
            .selected_text(
                games
                    .iter()
                    .find(|g| g.id == self.data.game_id)
                    .map(|t| t.name.as_ref())
                    .unwrap_or("Not selected"),
            )
            .wrap(false)
            .show_ui(ui, |ui| {
                //ui.set_min_width(60.0);
                for game in games {
                    ui.selectable_value(&mut self.data.game_id, game.id, &game.name);
                }
            });
        ui.label("Min_team_size");
        ui.add(egui::DragValue::new(&mut self.data.min_team_size).clamp_range(0..=255));
        ui.label("Max_team_size");
        ui.add(egui::DragValue::new(&mut self.data.max_team_size).clamp_range(0..=255));
        ui.checkbox(&mut self.data.requires_application, "Requires_application");
        ui.checkbox(
            &mut self.send_applications_closed,
            "Send applications closed (Ui thing)",
        );
        if self.send_applications_closed {
            ui.checkbox(&mut self.applications_closed, "Applications_closed");
            ui.label("Number_of_final_places");
            ui.add(
                egui::DragValue::new(&mut self.number_of_final_places).clamp_range(1..=u32::MAX),
            );
            ui.checkbox(&mut self.regenerate_brackets, "Regenerate_brackets");
        }
        egui::ComboBox::new("Type", "Type")
            .selected_text(format!("{:?}", self.data.tournament_type))
            .wrap(false)
            .show_ui(ui, |ui| {
                //ui.set_min_width(60.0);
                ui.selectable_value(
                    &mut self.data.tournament_type,
                    TournamentType::FFA,
                    format!("{:?}", TournamentType::FFA),
                );
                ui.selectable_value(
                    &mut self.data.tournament_type,
                    TournamentType::OneBracketOneFinalPositions,
                    format!("{:?}", TournamentType::OneBracketOneFinalPositions),
                );
                ui.selectable_value(
                    &mut self.data.tournament_type,
                    TournamentType::OneBracketTwoFinalPositions,
                    format!("{:?}", TournamentType::OneBracketTwoFinalPositions),
                );
            });

        if ui.button("Create tournament").clicked() {
            self.new_req.start_download(
                json_post(
                    &self.token,
                    &self.public_url,
                    "/tournaments/new",
                    &self.data,
                ),
                ctx.clone(),
            )
        }
        self.new_req.show_ui(
            ui,
            |ui, game| {
                ui.label(format!("Created tournament with id: {}", game.id));
            },
            default_err_fn,
        );
        if ui.button("Edit tournament").clicked() {
            if self.send_applications_closed {
                self.data.applications_closed = Some(if self.applications_closed {
                    ApplicationsClosed::True {
                        number_of_final_places: self.number_of_final_places,
                        regenerate_brackets: self.regenerate_brackets,
                    }
                } else {
                    ApplicationsClosed::False
                });
            }

            self.edit_req.start_download(
                json_post(
                    &self.token,
                    &self.public_url,
                    "/tournaments/edit",
                    &self.data,
                ),
                ctx,
            )
        }
        self.edit_req.show_ui(
            ui,
            |ui, _| {
                ui.label("Edited tournament".to_string());
            },
            default_err_fn,
        );
    }
}

pub struct BracketManipulator {
    data: BracketData,
    token: String,
    teams: Downloadable<Vec<teams::Team>>,
    public_url: String,
    edit_req: Downloadable<Nothing>,
}

#[derive(serde::Serialize)]
struct BracketData {
    team1: Option<Uuid>,
    team2: Option<Uuid>,
    team1_score: i64,
    team2_score: i64,
    winner: Option<bool>,
    bracket_tree_id: Uuid,
    layer: u8,
    position: i32,
    suppress_propagation: bool,
}

impl BracketManipulator {
    pub fn new_with_data(
        token: String,
        public_url: String,
        teams: Downloadable<Vec<teams::Team>>,
        team1: Option<Uuid>,
        team2: Option<Uuid>,
        team1_score: i64,
        team2_score: i64,
        winner: Option<bool>,
        bracket_tree_id: Uuid,
        layer: u8,
        position: i32,
    ) -> Self {
        Self {
            data: BracketData {
                team1,
                team2,
                team1_score,
                team2_score,
                winner,
                bracket_tree_id,
                layer,
                position,
                suppress_propagation: false,
            },

            token,
            public_url,
            teams,
            edit_req: Downloadable::new(),
        }
    }
}

impl ManipulatorTrait for BracketManipulator {
    fn show_ui(&mut self, ui: &mut egui::Ui, ctx: egui::Context) {
        let teams = self.teams.get_data();
        let empty_vec = Vec::new();
        let teams = (*teams).as_ref().unwrap_or(&empty_vec);
        egui::ComboBox::new("Team1", "Team1")
            .selected_text(
                teams
                    .iter()
                    .find(|t| Some(t.id) == self.data.team1)
                    .map(|t| t.name.as_ref())
                    .unwrap_or("Not selected"),
            )
            .wrap(false)
            .show_ui(ui, |ui| {
                //ui.set_min_width(60.0);
                ui.selectable_value(&mut self.data.team1, None, "Not selected");
                for team in teams {
                    ui.selectable_value(&mut self.data.team1, Some(team.id), &team.name);
                }
            });
        egui::ComboBox::new("Team2", "Team2")
            .selected_text(
                teams
                    .iter()
                    .find(|t| Some(t.id) == self.data.team2)
                    .map(|t| t.name.as_ref())
                    .unwrap_or("Not selected"),
            )
            .wrap(false)
            .show_ui(ui, |ui| {
                //ui.set_min_width(60.0);
                ui.selectable_value(&mut self.data.team2, None, "Not selected");
                for team in teams {
                    ui.selectable_value(&mut self.data.team2, Some(team.id), &team.name);
                }
            });
        ui.label("Team1_score");
        ui.add(egui::DragValue::new(&mut self.data.team1_score).clamp_range(i64::MIN..=i64::MAX));
        ui.label("Team2_score");
        ui.add(egui::DragValue::new(&mut self.data.team2_score).clamp_range(i64::MIN..=i64::MAX));
        egui::ComboBox::new("Winner", "Winner")
            .selected_text(
                self.data
                    .winner
                    .map(|w| if w { "Team 1" } else { "Team 2" })
                    .unwrap_or("Not selected"),
            )
            .wrap(false)
            .show_ui(ui, |ui| {
                //ui.set_min_width(60.0);
                ui.selectable_value(&mut self.data.winner, None, "Not selected");
                ui.selectable_value(&mut self.data.winner, Some(true), "Team 1");
                ui.selectable_value(&mut self.data.winner, Some(false), "Team 2");
            });
        ui.checkbox(&mut self.data.suppress_propagation, "Suppress_propagation");

        if ui.button("Edit bracket").clicked() {
            self.edit_req.start_download(
                json_post(&self.token, &self.public_url, "/brackets/edit", &self.data),
                ctx,
            )
        }
        self.edit_req.show_ui(
            ui,
            |ui, _| {
                ui.label("Edited bracket".to_string());
            },
            default_err_fn,
        );
    }
}
