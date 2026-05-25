use crate::gui::state::{Cmd, EditorState, State};
use crate::gui::view;
use eframe::egui;

pub fn ui(ui: &mut egui::Ui, state: &mut State) {
    if let Some(cmd) = state.take_cmd() {
        match cmd {
            Cmd::NewEntity => {
                state.add();
            }
            Cmd::UpdateEntity => {
                state.update();
            }
            Cmd::RemoveEntry(idx) => {
                state.remove(idx);
            }
            Cmd::EditorNew => {
                state.main.reset_editor();
                state.main.editor_state = EditorState::New;
            }
            Cmd::EditorUpdate(idx) => {
                state.main.editor_state = EditorState::Edit;
                state.edit_data(idx);
            }
            Cmd::Backup => state.backup(),
            Cmd::Export => state.export(),
            Cmd::Lock => state.lock_session(),
            Cmd::OpenChangePassword => {
                state.security_mut().change_password_open = true;
                state.security_mut().clear_password_fields();
            }
            Cmd::OpenSettings => {
                state.security_mut().settings_open = true;
                state.security_mut().auto_lock_secs = state.auto_lock_secs();
            }
        }
    }

    egui::Panel::top("menu").show_inside(ui, |ui| {
        view::menu::ui(ui, state);
    });

    egui::Panel::bottom("status").show_inside(ui, |ui| {
        view::status::ui(ui, state);
    });

    egui::CentralPanel::default().show_inside(ui, |ui| {
        view::table::ui(ui, state);
    });

    view::edit::ui(ui, state);
    view::security::ui(ui, state);
}
