use uuid::Uuid;

use crate::{
    app::{default_err_fn, json_post, Nothing},
    downloadable::Downloadable,
    manipulator::ManipulatorTrait,
    teams, users,
};

#[derive(serde::Deserialize)]
pub struct User {
    id: Uuid,
    player_invites: Vec<Team>,
    manager_invites: Vec<Team>,
}

#[derive(serde::Deserialize)]
struct Team {
    id: Uuid,
    name: String,
}

pub fn user_invites_ui(ctx: &egui::Context, app: &mut crate::app::FrontendApp) {
    egui::Window::new("User invites")
        .collapsible(true)
        .resizable(true)
        .show(ctx, |ui| {
            if ui.button("Reload").clicked() {
                app.user_invites.start_download(
                    ehttp::Request::get(format!("{}/users/invites", app.url)),
                    ctx.clone(),
                );
            }
            {
                let user = &*app.user.get_data();
                if app.token.is_some() && user.is_some() && ui.button("New").clicked() {
                    let user_id = user.as_ref().unwrap().id;
                    let users = (*app.users.get_data()).clone().unwrap_or_default();
                    let teams = (*app.teams.get_data())
                        .clone()
                        .unwrap_or_default()
                        .into_iter()
                        .filter(|t| t.managers.iter().any(|m| m.id == user_id))
                        .collect();
                    app.manipulator_window.set_editor(NewInvite::new(
                        app.token.clone().unwrap(),
                        app.url.clone(),
                        users,
                        teams,
                    ));
                }
            }
            app.user_invites.show_ui(
                ui,
                |ui, users| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for user in users {
                            let header_name = (*app.users.get_data())
                                .as_ref()
                                .map(|users| {
                                    users
                                        .iter()
                                        .filter(|u| u.id == user.id)
                                        .map(|u| u.nick.clone())
                                        .next()
                                })
                                .unwrap_or(None)
                                .unwrap_or(user.id.to_string());
                            egui::CollapsingHeader::new(header_name).show(ui, |ui| {
                                egui::Grid::new("user").show(ui, |ui| {
                                    ui.label("Id");
                                    ui.label(user.id.to_string());
                                    ui.end_row();
                                });
                                let is_loggedin_user = (*app.user.get_data())
                                    .as_ref()
                                    .map(|d| d.id == user.id)
                                    .unwrap_or(false);

                                #[derive(serde::Serialize)]
                                struct ReqData {
                                    team_id: Uuid,
                                    accepted: bool,
                                }
                                macro_rules! fire_req {
                                    ($url:expr, $team_id:expr, $accepted:expr) => {
                                        ehttp::fetch(
                                            json_post(
                                                app.token.as_ref().unwrap(),
                                                &app.url,
                                                $url,
                                                &ReqData {
                                                    team_id: $team_id,
                                                    accepted: $accepted,
                                                },
                                            ),
                                            |_response| {},
                                        );
                                    };
                                }
                                egui::CollapsingHeader::new("Player invites").show(ui, |ui| {
                                    for team in &user.player_invites {
                                        egui::CollapsingHeader::new(&team.name).show(ui, |ui| {
                                            egui::Grid::new("team").show(ui, |ui| {
                                                ui.label("Id");
                                                ui.label(team.id.to_string());
                                                ui.end_row();

                                                ui.label("Name");
                                                ui.label(&team.name);
                                                ui.end_row();
                                            });
                                            if is_loggedin_user && ui.button("Accept").clicked() {
                                                fire_req!(
                                                    "/teams/players/handle_invite",
                                                    team.id,
                                                    true
                                                );
                                            }
                                            if is_loggedin_user && ui.button("Reject").clicked() {
                                                fire_req!(
                                                    "/teams/players/handle_invite",
                                                    team.id,
                                                    false
                                                );
                                            }
                                        });
                                    }
                                });
                                egui::CollapsingHeader::new("Manager invites").show(ui, |ui| {
                                    for team in &user.manager_invites {
                                        egui::CollapsingHeader::new(&team.name).show(ui, |ui| {
                                            egui::Grid::new("team").show(ui, |ui| {
                                                ui.label("Id");
                                                ui.label(team.id.to_string());
                                                ui.end_row();

                                                ui.label("Name");
                                                ui.label(&team.name);
                                                ui.end_row();
                                            });
                                        });
                                        if is_loggedin_user && ui.button("Accept").clicked() {
                                            fire_req!(
                                                "/teams/managers/handle_invite",
                                                team.id,
                                                true
                                            );
                                        }
                                        if is_loggedin_user && ui.button("Reject").clicked() {
                                            fire_req!(
                                                "/teams/managers/handle_invite",
                                                team.id,
                                                false
                                            );
                                        }
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

struct NewInvite {
    selected_team: Uuid,
    selected_user: Uuid,
    users: Vec<users::User>,
    teams: Vec<teams::Team>,
    token: String,
    public_url: String,
    req: Downloadable<Nothing>,
    url_idx: usize,
}

#[derive(serde::Serialize)]
struct NewPlayerInviteData {
    team_id: Uuid,
    player_ids: Vec<Uuid>,
}

#[derive(serde::Serialize)]
struct NewManagerInviteData {
    team_id: Uuid,
    manager_ids: Vec<Uuid>,
}

const INVITE_URLS: [(&str, &str); 2] = [
    ("Player", "/teams/players/invite"),
    ("Manager", "/teams/managers/invite"),
];

impl NewInvite {
    fn new(
        token: String,
        public_url: String,
        users: Vec<users::User>,
        teams: Vec<teams::Team>,
    ) -> Self {
        Self {
            selected_team: teams.first().map(|t| t.id).unwrap_or(Uuid::nil()),
            selected_user: users.first().map(|u| u.id).unwrap_or(Uuid::nil()),
            users,
            teams,
            token,
            public_url,
            req: Downloadable::new(),
            url_idx: 0,
        }
    }
}

impl ManipulatorTrait for NewInvite {
    fn show_ui(&mut self, ui: &mut egui::Ui, ctx: egui::Context) {
        egui::ComboBox::new("Team", "Team")
            .selected_text(
                self.teams
                    .iter()
                    .filter(|t| t.id == self.selected_team)
                    .next()
                    .map(|t| t.name.as_ref())
                    .unwrap_or("Not selected".into()),
            )
            .wrap(false)
            .show_ui(ui, |ui| {
                //ui.set_min_width(60.0);
                for team in &self.teams {
                    ui.selectable_value(&mut self.selected_team, team.id, &team.name);
                }
            });
        egui::ComboBox::new("User", "User")
            .selected_text(
                self.users
                    .iter()
                    .filter(|u| u.id == self.selected_user)
                    .next()
                    .map(|u| u.nick.as_ref())
                    .unwrap_or("Not selected".into()),
            )
            .wrap(false)
            .show_ui(ui, |ui| {
                //ui.set_min_width(60.0);
                for user in &self.users {
                    ui.selectable_value(&mut self.selected_user, user.id, &user.nick);
                }
            });
        egui::ComboBox::new("InviteAS", "Invite as")
            .selected_text(INVITE_URLS[self.url_idx].0)
            .wrap(false)
            .show_ui(ui, |ui| {
                //ui.set_min_width(60.0);
                for (i, (invite_as, _url)) in INVITE_URLS.iter().enumerate() {
                    ui.selectable_value(&mut self.url_idx, i, *invite_as);
                }
            });

        if ui.button("Invite").clicked() {
            let request = match self.url_idx {
                0 => json_post(
                    &self.token,
                    &self.public_url,
                    INVITE_URLS[self.url_idx].1,
                    &NewPlayerInviteData {
                        team_id: self.selected_team,
                        player_ids: vec![self.selected_user],
                    },
                ),
                1 => json_post(
                    &self.token,
                    &self.public_url,
                    INVITE_URLS[self.url_idx].1,
                    &NewManagerInviteData {
                        team_id: self.selected_team,
                        manager_ids: vec![self.selected_user],
                    },
                ),
                _ => unreachable!(),
            };
            self.req.start_download(request, ctx.clone())
        }
        self.req.show_ui(
            ui,
            |ui, _| {
                ui.label("User invited");
            },
            default_err_fn,
        );
    }
}
