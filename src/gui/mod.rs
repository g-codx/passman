mod app;
mod error;
mod state;
mod view;

use crate::gui::app::PassmanApp;
use eframe::egui;

pub fn run() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Passman",
        options,
        Box::new(|_| Ok(Box::<PassmanApp>::default())),
    )
    .expect("Failed to start eframe");
}
