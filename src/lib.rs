use anyhow::{anyhow, Result};
use eframe::egui::{self, plot::{Plot, PlotPoint, Line, PlotPoints}, DragValue};


pub struct Symulacja {
    playing: bool,
    x: i32,
    y: i32,
    yv: i32,
    mass: i32,
    gravity: i32
}

impl Default for Symulacja {
    fn default() -> Self {
        Self {
            playing: false,
            x: 0,
            y: 0,
            yv: 0,
            mass: 1,
            gravity: 10
        }
    }
}

impl eframe::App for Symulacja {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let object = (0..1000).map(|i| {
            self.y = self.yv * i;
            self.yv = self.mass * self.gravity * i;

            [i as f64, self.y as f64] // todo: plotting
        });

        let line = Line::new(PlotPoints::from_iter(object));

        egui::CentralPanel::default().show(ctx, |ui| {
            Plot::new("plot").view_aspect(2.).show(ui, |pui| pui.line(line));

            ui.add(DragValue::new(&mut self.gravity).speed(0.1).clamp_range(0..=10));
        });
    }
}