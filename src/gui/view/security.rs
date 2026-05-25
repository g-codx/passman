use crate::gui::state::State;
use eframe::egui;

pub fn ui(ui: &mut egui::Ui, state: &mut State) {
    change_password_window(ui, state);
    settings_window(ui, state);
}

fn change_password_window(ui: &mut egui::Ui, state: &mut State) {
    let mut open = state.security_mut().change_password_open;

    egui::Window::new("Change master password")
        .default_width(420.0)
        .open(&mut open)
        .show(ui.ctx(), |ui| {
            ui.label("A new salt will be generated and the vault re-encrypted.");

            ui.horizontal(|ui| {
                ui.label("Current:");
                ui.add(
                    egui::TextEdit::singleline(&mut state.security_mut().current)
                        .password(true),
                );
            });
            ui.horizontal(|ui| {
                ui.label("New:");
                ui.add(
                    egui::TextEdit::singleline(&mut state.security_mut().new).password(true),
                );
            });
            ui.horizontal(|ui| {
                ui.label("Confirm:");
                ui.add(
                    egui::TextEdit::singleline(&mut state.security_mut().confirm)
                        .password(true),
                );
            });

            if ui.button("Apply").clicked() {
                if let Err(err) = state.change_master_password() {
                    state.show_error(err.to_string());
                }
            }
        });

    state.security_mut().change_password_open = open;
}

fn settings_window(ui: &mut egui::Ui, state: &mut State) {
    let mut open = state.security_mut().settings_open;

    egui::Window::new("Settings")
        .default_width(360.0)
        .open(&mut open)
        .show(ui.ctx(), |ui| {
            ui.label("Auto-lock after inactivity (seconds):");

            let security = state.security_mut();
            ui.add(
                egui::Slider::new(&mut security.auto_lock_secs, 60..=3600).logarithmic(true),
            );
            ui.label(format!(
                "≈ {} minutes",
                security.auto_lock_secs / 60
            ));

            if ui.button("Save").clicked() {
                state.save_settings();
            }
        });

    state.security_mut().settings_open = open;
}
