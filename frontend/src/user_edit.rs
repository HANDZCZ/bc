use std::{cell::RefCell, rc::Rc};

use crate::{
    app::{default_err_fn, json_post, Nothing},
    downloadable::Downloadable,
    manipulator::ManipulatorTrait,
};

pub struct EditUser {
    data: UserData,
    token: String,
    public_url: String,
    password_input: String,
    req: Downloadable<Nothing>,
    edit_fired: bool,
    edit_completed: bool,
}

#[derive(serde::Serialize)]
struct UserData {
    nick: String,
    email: String,
    password: Option<String>,
}

impl EditUser {
    pub fn new(token: String, public_url: String, nick: String, email: String) -> Self {
        Self {
            data: UserData {
                nick,
                email,
                password: None,
            },
            password_input: String::new(),
            token,
            public_url,
            req: Downloadable::new(),
            edit_fired: false,
            edit_completed: false,
        }
    }

    pub fn edited(&mut self) -> bool {
        if self.edit_fired && self.edit_completed {
            self.edit_fired = false;
            return true;
        }
        false
    }
}

impl ManipulatorTrait for Rc<RefCell<EditUser>> {
    fn show_ui(&mut self, ui: &mut egui::Ui, ctx: egui::Context) {
        self.borrow_mut().show_ui(ui, ctx);
    }
}

impl ManipulatorTrait for EditUser {
    fn show_ui(&mut self, ui: &mut egui::Ui, ctx: egui::Context) {
        ui.add(egui::TextEdit::singleline(&mut self.data.nick).hint_text("Nick"));
        ui.add(egui::TextEdit::singleline(&mut self.data.email).hint_text("Email"));
        ui.add(egui::TextEdit::singleline(&mut self.password_input).hint_text("Password"));

        if ui.button("Edit").clicked() {
            self.edit_fired = true;
            self.edit_completed = false;
            self.data.password = if self.password_input.is_empty() {
                None
            } else {
                Some(self.password_input.clone())
            };
            let request = json_post(&self.token, &self.public_url, "/users/edit", &self.data);
            self.req.start_download(request, ctx.clone())
        }
        self.req.show_ui(
            ui,
            |ui, _| {
                ui.label("User edited successfully");
                self.edit_completed = true;
            },
            default_err_fn,
        );
    }
}
