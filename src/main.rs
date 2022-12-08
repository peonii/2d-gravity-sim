#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, NativeOptions};
use sym_swobodne::runner::Symulacja;

fn main() {
    let opts = NativeOptions {
        decorated: false,
        transparent: true,
        min_window_size: Some(egui::vec2(800., 600.)),
        ..Default::default()
    };

    eframe::run_native(
        "Symulacja",
        opts,
        Box::new(|_cc| Box::new(Symulacja::new())),
    );
}
