use crate::gui::state::State;
use eframe::egui;
use eframe::egui::Color32;

pub fn ui(ctx: &egui::Context, state: &mut State) {
    let main = &mut *state.main();

    let color = main.color();
    let show = *main.show_mut();
    let mut open = main.is_open();

    egui::Window::new("Entry")
        .default_width(400.0)
        .open(&mut open)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add_sized([100.0, 0.0], egui::Label::new("Service ❗"))
                        .on_hover_text("Required field");

                    let _ = ui.add(
                        egui::TextEdit::singleline(main.service_mut())
                            .text_color(Color32::BLACK)
                            .background_color(color),
                    );
                });
                ui.horizontal(|ui| {
                    ui.add_sized([100.0, 0.0], egui::Label::new("Username ❗"))
                        .on_hover_text("Required field");
                    let _ = ui.add(
                        egui::TextEdit::singleline(main.username_mut())
                            .text_color(Color32::BLACK)
                            .background_color(color),
                    );
                });
                ui.horizontal(|ui| {
                    ui.add_sized([100.0, 0.0], egui::Label::new("Password ❗"))
                        .on_hover_text("Required field");
                    let _ = ui.add(
                        egui::TextEdit::singleline(main.password_mut())
                            .text_color(Color32::BLACK)
                            .password(!show)
                            .background_color(color),
                    );
                    if ui.add(egui::Button::new("👁")).clicked() {
                        *main.show_mut() = !(*main.show_mut());
                    }
                    ui.add_space(5.0);
                    if ui.add(egui::Button::new("🎲")).clicked() {
                        *main.password_mut() = state.generate_password();
                    }
                });
                ui.horizontal(|ui| {
                    ui.add_sized([100.0, 0.0], egui::Label::new("Description"));
                    let _ = ui.add(
                        egui::TextEdit::singleline(main.notes_mut())
                            .text_color(Color32::BLACK)
                            .background_color(Color32::WHITE),
                    );
                });

                ui.add_space(10.0);

                if ui
                    .add_sized([100.0, 30.0], egui::Button::new("Save"))
                    .clicked()
                {
                    state.add(main);
                }
            });
        });

    if !open {
        main.close();
    }
}
