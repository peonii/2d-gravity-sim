#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use anyhow::Result;
use eframe::NativeOptions;
use sym_swobodne::Symulacja;


fn main() -> Result<()> {
    eframe::run_native("Symulacja", NativeOptions::default(), Box::new(|cc| Box::new(Symulacja::default())));

    Ok(())
}
