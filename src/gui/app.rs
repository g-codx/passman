use crate::gui::state::State;
use crate::gui::view;
use eframe::egui;
use std::ops::{Deref, DerefMut};

#[derive(Default)]
pub struct PassmanApp(State);

impl eframe::App for PassmanApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.is_locked() {
            view::auth::ui(ctx, self);
        } else {
            view::main::ui(ctx, self);
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
