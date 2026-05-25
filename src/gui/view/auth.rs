use crate::gui::state::State;
use eframe::egui;
use eframe::egui::{Color32, RichText, WidgetText};

pub fn ui(ui: &mut egui::Ui, state: &mut State) {
    egui::Panel::bottom("status").show_inside(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(WidgetText::from(state.status().to_string()).color(state.text_color()));
        });
    });

    egui::CentralPanel::default().show_inside(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(ui.available_height() / 3.);

            ui.heading(RichText::new("P🔒ssm🔓n").color(Color32::LIGHT_YELLOW));
            ui.add_space(10.0);

            let master_input = ui.add(
                egui::TextEdit::singleline(state.master_mut())
                    .password(true)
                    .background_color(Color32::WHITE)
                    .hint_text("Enter password")
                    .min_size([100.0, 0.0].into()),
            );

            master_input.request_focus();

            if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                state.load();
            }

            ui.add_space(20.0);

            if ui
                .add(egui::Button::new("Unlock 🔓").min_size([100.0, 0.0].into()))
                .clicked()
            {
                state.load();
            }
        });
    });
}
