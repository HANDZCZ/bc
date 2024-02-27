use std::{cell::RefCell, rc::Rc};

pub trait ManipulatorTrait {
    fn show_ui(&mut self, ui: &mut egui::Ui, ctx: egui::Context);
}

#[derive(Clone)]
pub struct ManipulatorWindow {
    inner: Rc<RefCell<ManipulatorWindowInner>>,
} 

struct ManipulatorWindowInner {
    inner: Box<dyn ManipulatorTrait>,
    opened: bool,
}

impl ManipulatorTrait for () {
    fn show_ui(&mut self, _ui: &mut egui::Ui, _ctx: egui::Context) {}
}

impl ManipulatorWindowInner {
    pub fn show_ui(&mut self, ctx: &egui::Context) {
        egui::Window::new("Manipulator")
            .collapsible(true)
            .resizable(true)
            .open(&mut self.opened)
            .show(ctx, |ui| {
                egui::ScrollArea::both().show(ui, |ui| {
                    self.inner.show_ui(ui, ctx.clone());
                });
            });
    }

    pub fn set_editor(&mut self, editor: impl ManipulatorTrait + 'static) {
        self.inner = Box::new(editor);
        self.opened = true;
    }

    pub fn init() -> Self {
        Self {
            inner: Box::new(()),
            opened: false,
        }
    }

    pub fn clear(&mut self) {
        self.opened = false;
        self.inner = Box::new(());
    }
}

impl ManipulatorWindow {
    pub fn show_ui(&self, ctx: &egui::Context) {
        self.inner.borrow_mut().show_ui(ctx);
    }

    pub fn set_editor(&self, editor: impl ManipulatorTrait + 'static) {
        self.inner.borrow_mut().set_editor(editor);
    }

    pub fn init() -> Self {
        Self {
            inner: Rc::new(RefCell::new(ManipulatorWindowInner::init())),
        }
    }

    pub fn clear(&self) {
        self.inner.borrow_mut().clear();
    }
}