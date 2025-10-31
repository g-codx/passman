use crate::gui::state::{Cmd, EditorState, State};
use crate::gui::view;
use eframe::egui;

pub fn ui(ctx: &egui::Context, state: &mut State) {
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
                state.main.editor_state = EditorState::New;
            }
            Cmd::EditorUpdate(idx) => {
                state.main.editor_state = EditorState::Edit;
                state.edit_data(idx);
            }
        }
    }

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
