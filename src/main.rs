#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use egui::IconData;
use mypomodoro::MyApp;
use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init();

    let app = MyApp::default();
    // let icon_size = 64; // Standard icon size
    // let initial_icon = app.render_icon_data(icon_size);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([265.0, 380.0])
            .with_resizable(false)
            .with_always_on_top()
            .with_icon(IconData::default()),
        ..Default::default()
    };
    eframe::run_native(
        "My Pomodoro",
        options,
        Box::new(move |_cc| { // Move app into the closure
            Ok(Box::new(app))
        }),
    )
}
