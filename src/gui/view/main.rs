use crate::gui::state::State;
use crate::gui::view;
use eframe::egui;

pub fn ui(ctx: &egui::Context, state: &mut State) {
    egui::TopBottomPanel::top("menu").show(ctx, |ui| {
        view::menu::ui(ui, state);
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        view::table::ui(ui, state);
    });

    egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
        view::status::ui(ui, state);
    });
    view::edit::ui(ctx, state);
}
