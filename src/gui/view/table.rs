use crate::core::entry::Entry;
use crate::gui::state::{Cmd, State};
use crate::gui::view::widgets;
use eframe::egui;
use egui_extras::{Column, TableBuilder};

enum CopyField {
    Service,
    Username,
    Password,
}

impl CopyField {
    fn copy_label(self) -> &'static str {
        match self {
            Self::Service => "Copy service",
            Self::Username => "Copy username",
            Self::Password => "Copy password",
        }
    }
}

pub fn ui(ui: &mut egui::Ui, state: &mut State) {
    let available_height = ui.available_height();
    let search = state.main.search_line.clone();

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
                if search_match(entry, &search) {
                    body.row(50.0, |mut row| {
                        row.col(|ui| {
                            interact_label(
                                ui,
                                idx,
                                &entry.service,
                                &mut cmd,
                                false,
                                CopyField::Service,
                            );
                        });
                        row.col(|ui| {
                            interact_label(
                                ui,
                                idx,
                                &entry.username,
                                &mut cmd,
                                false,
                                CopyField::Username,
                            );
                        });
                        row.col(|ui| {
                            interact_label(
                                ui,
                                idx,
                                entry.password(),
                                &mut cmd,
                                true,
                                CopyField::Password,
                            );
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
    field: CopyField,
) {
    use egui::{CursorIcon, Label, Sense};

    let visible_text = if is_password { "*******" } else { text };
    let copy_label = field.copy_label();
    let text_owned = text.to_owned();

    let resp = ui
        .add(Label::new(visible_text).sense(Sense::click()))
        .on_hover_text(format!("{copy_label} (click)"));

    if resp.hovered() {
        ui.output_mut(|o| o.cursor_icon = CursorIcon::Copy);
    }
    if resp.clicked() {
        ui.ctx().copy_text(text_owned.clone());
    }

    resp.context_menu(|ui| {
        match widgets::entry_context_menu(ui, copy_label) {
            Some(widgets::EntryMenuAction::Copy) => {
                ui.ctx().copy_text(text_owned);
            }
            Some(widgets::EntryMenuAction::Edit) => {
                *cmd = Some(Cmd::EditorUpdate(idx));
            }
            Some(widgets::EntryMenuAction::Remove) => {
                *cmd = Some(Cmd::RemoveEntry(idx));
            }
            None => {}
        }
    });
}

fn search_match(entry: &Entry, search_line: &str) -> bool {
    if search_line.is_empty() {
        return true;
    }

    let needle = search_line.to_lowercase();
    entry.service.to_lowercase().contains(&needle)
        || entry.username.to_lowercase().contains(&needle)
        || entry
            .notes
            .as_deref()
            .is_some_and(|n| n.to_lowercase().contains(&needle))
}
