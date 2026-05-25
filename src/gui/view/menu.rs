use crate::gui::state::{Cmd, State};
use eframe::egui;
use eframe::egui::{Color32, Key, Modifiers, TextEdit, Ui, Widget};

pub fn ui(ui: &mut Ui, state: &mut State) {
    egui::MenuBar::new().ui(ui, |ui| {
        if ui.button("🖹  New         Ctrl + S").clicked() {
            state.set_cmd(Cmd::EditorNew);
        }

        if ui.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::S)) {
            state.set_cmd(Cmd::EditorNew);
        }

        ui.menu_button("Vault", |ui| {
            if ui.button("💾  Backup").clicked() {
                state.set_cmd(Cmd::Backup);
                ui.close();
            }
            if ui.button("📤  Export JSON").clicked() {
                state.set_cmd(Cmd::Export);
                ui.close();
            }
            if ui.button("🔑  Change master password").clicked() {
                state.set_cmd(Cmd::OpenChangePassword);
                ui.close();
            }
            if ui.button("🔒  Lock now").clicked() {
                state.set_cmd(Cmd::Lock);
                ui.close();
            }
            if ui.button("⚙  Settings").clicked() {
                state.set_cmd(Cmd::OpenSettings);
                ui.close();
            }
        });

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);

        let response = TextEdit::singleline(&mut state.main.search_line)
            .background_color(Color32::WHITE)
            .text_color(Color32::BLACK)
            .ui(ui);

        if !state.main.is_open() {
            response.request_focus();
        }
    });
}
