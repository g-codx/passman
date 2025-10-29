use crate::gui::state::State;
use eframe::egui;
use eframe::egui::Ui;

pub fn ui(ui: &mut Ui, state: &mut State) {
    egui::MenuBar::new().ui(ui, |ui| {
        if ui.button("🖹 New").clicked() {
            state.main().new();
        }
    });
}
