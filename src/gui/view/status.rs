use crate::gui::state::State;
use eframe::egui;
use eframe::egui::WidgetText;

pub fn ui(ui: &mut egui::Ui, state: &State) {
    ui.horizontal(|ui| {
        ui.label(WidgetText::from(state.status().to_string()).color(state.text_color()));
        ui.separator();
        ui.label(format!("📃  Number of records: {}", state.len()));
    });
}
