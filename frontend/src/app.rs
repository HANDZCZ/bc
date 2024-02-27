use std::{cell::RefCell, rc::Rc};

use crate::{
    downloadable::Downloadable, games, manipulator, pan_zoom::PanZoom, signed_up_teams, teams,
    teams_playing_players, tournament_applications, tournaments, tournaments_playing_players,
    user_edit, user_invites, users,
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct FrontendApp {
    pub token: Option<String>,
    pub url: String,
    #[serde(skip)]
    pub manipulator_window: manipulator::ManipulatorWindow,

    #[serde(skip)]
    pub games: Downloadable<Vec<games::Game>>,
    #[serde(skip)]
    pub teams: Downloadable<Vec<teams::Team>>,
    #[serde(skip)]
    pub tournaments: Downloadable<Vec<tournaments::Tournament>>,
    #[serde(skip)]
    pub users: Downloadable<Vec<users::User>>,
    #[serde(skip)]
    pub signed_up_teams: Downloadable<Vec<signed_up_teams::Tournament>>,
    #[serde(skip)]
    pub tournament_applications: Downloadable<Vec<tournament_applications::Tournament>>,
    #[serde(skip)]
    pub teams_playing_players: Downloadable<Vec<teams_playing_players::Data>>,
    pub teams_playing_players_team_id: String,
    #[serde(skip)]
    pub user_invites: Downloadable<Vec<user_invites::User>>,

    #[serde(skip)]
    pub login_register_nick: String,
    #[serde(skip)]
    pub login_register_email: String,
    #[serde(skip)]
    pub login_register_password: String,
    #[serde(skip)]
    pub login_register: Downloadable<Nothing>,
    #[serde(skip)]
    pub user: Downloadable<users::User>,
    #[serde(skip)]
    pub user_editor: Option<Rc<RefCell<user_edit::EditUser>>>,
    #[serde(skip)]
    pub tournaments_playing_players: Downloadable<Vec<tournaments_playing_players::Data>>,
    pub tournaments_playing_players_tournament_id: String,

    #[serde(skip)]
    pub pan_zooms: Vec<PanZoom>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct OnlyId {
    pub id: uuid::Uuid,
}

#[derive(serde::Deserialize)]
pub struct Nothing {
    #[serde(skip)]
    _bs: (),
}

pub fn default_err_fn(ui: &mut egui::Ui, _response: &ehttp::Response) {
    ui.label("Something went wrong!");
}

pub fn json_post<T: serde::Serialize>(
    token: &str,
    public_url: &str,
    url: &str,
    data: &T,
) -> ehttp::Request {
    let mut req = ehttp::Request::json(format!("{}{}", public_url, url), data).unwrap();
    req.headers.insert("authorization", token);
    req
}

impl Default for FrontendApp {
    fn default() -> Self {
        Self {
            token: None,
            games: Downloadable::new(),
            teams: Downloadable::new(),
            tournaments: Downloadable::new(),
            users: Downloadable::new(),
            signed_up_teams: Downloadable::new(),
            tournament_applications: Downloadable::new(),
            teams_playing_players: Downloadable::new(),
            teams_playing_players_team_id: String::new(),
            user_invites: Downloadable::new(),

            login_register_nick: String::new(),
            login_register_email: String::new(),
            login_register_password: String::new(),
            login_register: Downloadable::new(),
            user: Downloadable::new(),
            user_editor: None,

            tournaments_playing_players: Downloadable::new(),
            tournaments_playing_players_tournament_id: String::new(),

            manipulator_window: manipulator::ManipulatorWindow::init(),
            url: String::new(),
            pan_zooms: Vec::new(),
        }
    }
}

impl FrontendApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        let mut app: FrontendApp = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        #[cfg(target_arch = "wasm32")]
        if app.url.is_empty() {
            app.url = format!("{}/api", cc.integration_info.web_info.location.origin);
        }

        if let Some(token) = &app.token {
            let mut req = ehttp::Request::get(format!("{}/users/self", app.url));
            req.headers.insert("authorization", token.clone());
            app.user.start_download(req, cc.egui_ctx.clone());
        }

        app
    }

    pub fn is_tournament_manager(&self) -> bool {
        let user = &*self.user.get_data();
        if self.token.is_some() && user.is_some() {
            return user
                .as_ref()
                .unwrap()
                .roles
                .contains(&"Tournament Manager".to_string());
        }
        false
    }
}

impl eframe::App for FrontendApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.style_mut(|egui_style| {
            egui_style.visuals.window_shadow.extrusion = 15.0;
            egui_style.visuals.popup_shadow.extrusion = 15.0;
        });
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(17.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
                ui.add(egui::TextEdit::singleline(&mut self.url).hint_text("Url"));
                if ui.button("Organize windows").clicked() {
                    ctx.memory_mut(|mem| mem.reset_areas());
                }

                let mut logout_clicked = false;
                ui.with_layout(
                    egui::Layout::right_to_left(egui::Align::RIGHT),
                    |ui| match self.token {
                        Some(_) => {
                            logout_clicked = ui.button("Logout").clicked();
                            let mut refetch_user = false;
                            match &*self.user.get_data() {
                                Some(user) => {
                                    let clicked = ui
                                        .label(&user.nick)
                                        .on_hover_ui(|ui| {
                                            egui::Grid::new("LoggedUser").show(ui, |ui| {
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
                                        })
                                        .clicked();
                                    if clicked {
                                        let user_editor =
                                            Rc::new(RefCell::new(user_edit::EditUser::new(
                                                self.token.clone().unwrap(),
                                                self.url.clone(),
                                                user.nick.clone(),
                                                user.email.clone(),
                                            )));
                                        self.user_editor = Some(user_editor.clone());
                                        self.manipulator_window.set_editor(user_editor);
                                    }

                                    if let Some(user_editor) = &self.user_editor {
                                        if user_editor.borrow_mut().edited() {
                                            refetch_user = true;
                                        }
                                    }
                                }
                                None => {
                                    ui.label("Fetching data");
                                }
                            };
                            if refetch_user {
                                let mut req =
                                    ehttp::Request::get(format!("{}/users/self", self.url));
                                req.headers
                                    .insert("authorization", self.token.clone().unwrap());
                                self.user.start_download(req, ctx.clone());
                            }
                        }
                        None => {
                            ui.label("Not logged in");
                        }
                    },
                );
                if logout_clicked {
                    self.user_editor = None;
                    self.manipulator_window.clear();
                    self.user.clear_data();
                    self.token = None;
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            if self.token.is_none() {
                crate::login_register::login_register_ui(ctx, self);
            }
            crate::games::games_ui(ctx, self);
            crate::teams::teams_ui(ctx, self);
            crate::tournaments::tournaments_ui(ctx, self);
            crate::users::users_ui(ctx, self);
            crate::signed_up_teams::signed_up_teams_ui(ctx, self);
            crate::tournament_applications::tournament_applications_ui(ctx, self);
            crate::tournaments_playing_players::tournaments_playing_players_ui(ctx, self);
            crate::teams_playing_players::teams_playing_players_ui(ctx, self);
            crate::user_invites::user_invites_ui(ctx, self);

            self.manipulator_window.show_ui(ctx);

            let mut to_delete = Vec::new();
            for (i, pan_zoom) in self.pan_zooms.iter_mut().enumerate() {
                pan_zoom.show(ctx);
                if !pan_zoom.open {
                    to_delete.push(i);
                }
            }
            for i in to_delete.into_iter().rev() {
                self.pan_zooms.remove(i);
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(5)
    }

    fn persist_egui_memory(&self) -> bool {
        false
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
