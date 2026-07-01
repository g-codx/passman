use crate::gui::state::{Cmd, State};
use crate::gui::view::widgets;
use eframe::egui;
use eframe::egui::containers::menu::MenuButton;
use eframe::egui::{Button, Key, Modifiers, Ui, Vec2};

const SEARCH_SLOT: f32 = 296.0;

pub fn ui(ui: &mut Ui, state: &mut State) {
    let total_w = ui.available_width();
    let search_w = SEARCH_SLOT.min(total_w * 0.5).max(140.0);
    let menu_w = (total_w - search_w).max(80.0);

    ui.horizontal(|ui| {
        ui.allocate_ui_with_layout(
            Vec2::new(menu_w, ui.available_height()),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                egui::MenuBar::new().ui(ui, |ui| {
                    widgets::toolbar_item(ui, |ui| {
                        if ui.add(Button::new("New").frame(false)).clicked() {
                            state.set_cmd(Cmd::EditorNew);
                        }
                    });

                    if ui.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::S)) {
                        state.set_cmd(Cmd::EditorNew);
                    }

                    widgets::toolbar_item(ui, |ui| {
                        MenuButton::from_button(Button::new("Vault").frame(false)).ui(ui, |ui| {
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
                    });
                });
            },
        );

        ui.allocate_ui_with_layout(
            Vec2::new(search_w, ui.available_height()),
            egui::Layout::right_to_left(egui::Align::Center),
            |ui| {
                ui.add_space(8.0);
                widgets::search_bar(ui, &mut state.main.search_line);
            },
        );
    });
}
