#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use anyhow::Result;
use eframe::{NativeOptions, egui};
use sym_swobodne::Symulacja;


fn main() -> Result<()> {
    let opts = NativeOptions {
        decorated: false,
        transparent: true,
        min_window_size: Some(egui::vec2(800., 600.)),
        ..Default::default()
    };

    eframe::run_native("Symulacja", opts, Box::new(|_cc| Box::new(Symulacja::default())));

    Ok(())
}