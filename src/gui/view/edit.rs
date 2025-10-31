use crate::gui::state::{Cmd, EditorState, State};
use eframe::egui;
use eframe::egui::Color32;

pub fn ui(ctx: &egui::Context, state: &mut State) {
    let bg_color = state.main.color;
    let mut open = state.main.is_open();

    egui::Window::new("Entry")
        .default_width(400.0)
        .open(&mut open)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add_sized([100.0, 0.0], egui::Label::new("Service ❗"))
                        .on_hover_text("Required field");

                    let _ = ui.add(
                        egui::TextEdit::singleline(&mut state.main.service)
                            .text_color(Color32::BLACK)
                            .background_color(bg_color),
                    );
                });
                ui.horizontal(|ui| {
                    ui.add_sized([100.0, 0.0], egui::Label::new("Username ❗"))
                        .on_hover_text("Required field");
                    let _ = ui.add(
                        egui::TextEdit::singleline(&mut state.main.username)
                            .text_color(Color32::BLACK)
                            .background_color(bg_color),
                    );
                });
                ui.horizontal(|ui| {
                    ui.add_sized([100.0, 0.0], egui::Label::new("Password ❗"))
                        .on_hover_text("Required field");

                    {
                        let show = state.main.show;
                        let _ = ui.add(
                            egui::TextEdit::singleline(&mut state.main.password)
                                .text_color(Color32::BLACK)
                                .password(!show)
                                .background_color(bg_color),
                        );
                    }

                    let eye_clicked = ui.add(egui::Button::new("👁")).clicked();
                    ui.add_space(5.0);
                    let dice_clicked = ui.add(egui::Button::new("🎲")).clicked();

                    if eye_clicked {
                        let show = &mut state.main.show;
                        *show = !*show;
                    }
                    if dice_clicked {
                        let new = state.generate_password();
                        let password = &mut state.main.password;
                        *password = new;
                    }
                });

                ui.horizontal(|ui| {
                    ui.add_sized([100.0, 0.0], egui::Label::new("Description"));
                    let _ = ui.add(
                        egui::TextEdit::singleline(&mut state.main.notes)
                            .text_color(Color32::BLACK)
                            .background_color(Color32::WHITE),
                    );
                });

                ui.add_space(10.0);

                if ui
                    .add_sized([100.0, 30.0], egui::Button::new("Save"))
                    .clicked()
                {
                    match state.main.editor_state {
                        EditorState::New => {
                            state.set_cmd(Cmd::NewEntity);
                        }
                        EditorState::Edit => {
                            state.set_cmd(Cmd::UpdateEntity);
                            state.main.editor_state = EditorState::Close;
                        }
                        EditorState::Close => {}
                    }
                }
            });
        });

    if !open {
        state.main.editor_state = EditorState::Close;
    }
}
