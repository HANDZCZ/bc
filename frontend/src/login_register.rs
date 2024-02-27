use crate::app::default_err_fn;

#[derive(serde::Serialize)]
struct ReqData<'a> {
    nick: &'a str,
    email: &'a str,
    password: &'a str,
}

pub fn login_register_ui(ctx: &egui::Context, app: &mut crate::app::FrontendApp) {
    egui::Window::new("Login / Register")
        .collapsible(true)
        .resizable(true)
        .show(ctx, |ui| {
            ui.add(egui::TextEdit::singleline(&mut app.login_register_nick).hint_text("Nick"));
            ui.add(egui::TextEdit::singleline(&mut app.login_register_email).hint_text("Email"));
            ui.add(
                egui::TextEdit::singleline(&mut app.login_register_password).hint_text("Password"),
            );

            fn fire_req(url: String, app: &mut crate::app::FrontendApp, ctx: &egui::Context) {
                app.token = None;
                app.login_register.start_download(
                    ehttp::Request::json(
                        url,
                        &ReqData {
                            nick: app.login_register_nick.as_str(),
                            email: app.login_register_email.as_str(),
                            password: app.login_register_password.as_str(),
                        },
                    )
                    .unwrap(),
                    ctx.clone(),
                );
            }

            if ui.button("Login").clicked() {
                fire_req(format!("{}/users/login", app.url), app, ctx)
            }
            if ui.button("Register").clicked() {
                fire_req(format!("{}/users/register", app.url), app, ctx)
            }
            app.login_register.show_ui(
                ui,
                |_ui, _| {
                    if let Some(response) = app.login_register.get_response() {
                        if let Some(token) = response.headers.get("authorization") {
                            match &app.token {
                                Some(old_token) if old_token == token => {}
                                _ => {
                                    let mut req =
                                        ehttp::Request::get(format!("{}/users/self", app.url));
                                    req.headers.insert("authorization", token);
                                    app.user.start_download(req, ctx.clone());
                                }
                            }
                            app.token = Some(token.into());
                        }
                    }
                },
                default_err_fn,
            );
        });
}
