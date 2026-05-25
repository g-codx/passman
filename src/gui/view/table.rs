use crate::core::entry::Entry;
use crate::gui::state::{Cmd, State};
use eframe::egui;
use egui_extras::{Column, TableBuilder};

pub fn ui(ui: &mut egui::Ui, state: &mut State) {
    let available_height = ui.available_height();

    let table = TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::remainder())
        .column(Column::remainder())
        .column(Column::remainder())
        .column(Column::remainder())
        .min_scrolled_height(0.0)
        .max_scroll_height(available_height);

    table
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.strong("Service");
            });
            header.col(|ui| {
                ui.strong("Username");
            });
            header.col(|ui| {
                ui.strong("Password");
            });
            header.col(|ui| {
                ui.strong("Description");
            });
        })
        .body(|mut body| {
            let mut cmd = None;

            for (idx, entry) in state.list().iter().enumerate() {
                if search_match(entry, &state.main.search_line) {
                    body.row(50.0, |mut row| {
                        row.col(|ui| {
                            interact_label(ui, idx, &entry.service, &mut cmd, false);
                        });
                        row.col(|ui| {
                            interact_label(ui, idx, &entry.username, &mut cmd, false);
                        });
                        row.col(|ui| {
                            interact_label(ui, idx, entry.password(), &mut cmd, true);
                        });
                        row.col(|ui| {
                            ui.label(entry.notes.as_deref().unwrap_or_default());
                        });
                    });
                }

                if cmd.is_some() {
                    break;
                }
            }

            if let Some(cmd) = cmd {
                state.set_cmd(cmd);
            }
        });
}

fn interact_label(
    ui: &mut egui::Ui,
    idx: usize,
    text: &str,
    cmd: &mut Option<Cmd>,
    is_password: bool,
) {
    use egui::{CursorIcon, Label, Sense};

    let visible_text = if is_password { "*******" } else { text };

    let resp = ui
        .add(Label::new(visible_text).sense(Sense::click()))
        .on_hover_text("Click to copy");

    if resp.hovered() {
        ui.output_mut(|o| o.cursor_icon = CursorIcon::Copy);
    }
    if resp.clicked() {
        ui.ctx().copy_text(text.to_owned());
    }

    resp.context_menu(|ui| {
        if ui.button("Edit").clicked() {
            *cmd = Some(Cmd::EditorUpdate(idx));
        }
        if ui.button("Remove").clicked() {
            *cmd = Some(Cmd::RemoveEntry(idx));
        }
    });
}

fn search_match(entry: &Entry, search_line: &str) -> bool {
    entry
        .service
        .to_lowercase()
        .contains(search_line.to_lowercase().as_str())
        || entry
            .username
            .to_lowercase()
            .contains(search_line.to_lowercase().as_str())
        || search_line.is_empty()
}
