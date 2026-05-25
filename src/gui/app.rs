use crate::gui::state::State;
use crate::gui::view;
use eframe::egui;
use std::ops::{Deref, DerefMut};

#[derive(Default)]
pub struct PassmanApp(State);

impl eframe::App for PassmanApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.register_activity(ctx);
        self.schedule_idle_check(ctx);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.is_locked() {
            view::auth::ui(ui, self);
        } else {
            view::main::ui(ui, self);
        }
    }
}

impl Deref for PassmanApp {
    type Target = State;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PassmanApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
