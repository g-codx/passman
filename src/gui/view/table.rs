use crate::gui::state::State;
use eframe::egui;
use egui_extras::{Column, TableBuilder};

pub fn ui(ui: &mut egui::Ui, state: &mut State) {
    let available_height = ui.available_height();

    let mut cmd = Command::new();

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
            for (idx, entry) in state.list().iter().enumerate() {
                body.row(50.0, |mut row| {
                    row.col(|ui| {
                        interact_label(ui, idx, &entry.service, &mut cmd);
                    });
                    row.col(|ui| {
                        interact_label(ui, idx, &entry.username, &mut cmd);
                    });
                    row.col(|ui| {
                        interact_label(ui, idx, &entry.password, &mut cmd);
                    });
                    row.col(|ui| {
                        ui.label(entry.notes.as_deref().unwrap_or_default());
                    });
                });
            }
        });

    match cmd.0 {
        Kind::EditEntry(idx) => {
            state.edit_data(idx);
            state.main().edit();
        }
        Kind::RemoveEntry(idx) => {}
        Kind::None => {}
    }
}

fn interact_label(ui: &mut egui::Ui, idx: usize, text: &str, cmd: &mut Command) {
    use egui::{CursorIcon, Label, Sense};

    let resp = ui
        .add(Label::new(text).sense(Sense::click()))
        .on_hover_text("Click to copy");

    if resp.hovered() {
        ui.output_mut(|o| o.cursor_icon = CursorIcon::Copy);
    }
    if resp.clicked() {
        ui.ctx().copy_text(text.to_owned());
    }

    resp.context_menu(|ui| {
        if ui.button("Edit").clicked() {
            cmd.set(Kind::EditEntry(idx));
        }
        if ui.button("Remove").clicked() {
            cmd.set(Kind::RemoveEntry(idx));
        }
    });
}

enum Kind {
    EditEntry(usize),
    RemoveEntry(usize),
    None,
}

struct Command(Kind);

impl Command {
    pub fn new() -> Self {
        Command(Kind::None)
    }

    pub fn set(&mut self, kind: Kind) {
        if matches!(self.0, Kind::None) && !matches!(kind, Kind::None) {
            self.0 = kind;
        }
    }
}
