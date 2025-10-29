use crate::gui::state::State;
use eframe::egui;
use eframe::egui::Color32;

pub fn ui(ctx: &egui::Context, state: &mut State) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("Enter Master Password");

            ui.add_space(20.0);

            let master_input = ui.add(
                egui::TextEdit::singleline(state.master())
                    .password(true)
                    .background_color(Color32::WHITE)
                    .min_size([100.0, 0.0].into()),
            );

            master_input.request_focus();

            if master_input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
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
