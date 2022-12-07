use std::process;

use eframe::egui::{
    self,
    plot::{Legend, Line, Plot, PlotPoints},
    Label, RichText, Slider,
};

use crate::sim_point::SimPoint;
use crate::window_frame::window_frame;

use crate::line_from;

pub struct Symulacja {
    pub y: f64,
    pub velocity: f64,
    pub starting_velocity: f64,
    pub starting_y: f64,
    pub starting_y_accel: f64,
    pub y_accel: f64,
    pub drag_coefficient: f64,
}

impl Default for Symulacja {
    fn default() -> Self {
        Self {
            y: 0.,
            velocity: 0.,
            starting_velocity: 0.,
            starting_y: 0.,
            starting_y_accel: 10.,
            y_accel: 10.,
            drag_coefficient: 0.1,
            // we're assuming the area is 1m, so no need to store if
            // it's gonna be skipped in equations either way
            // let's also assume the object weighs 1kg
        }
    }
}

impl Symulacja {
    fn update_object(&mut self) -> Vec<SimPoint> {
        let mut sim_points: Vec<SimPoint> = Vec::new();

        self.velocity = self.starting_velocity;
        self.y = self.starting_y;
        self.y_accel = -self.starting_y_accel;

        let mut time = 0.;
        let dt = 0.001;

        // this is set to true when we've hit the ground (y <= 0)
        let mut hit = false;
        while !hit {
            // y = y - vel
            self.y = self.y + self.velocity * dt;
            // vel = vel + accel
            self.velocity = self.velocity + self.y_accel * dt;

            // accel = 0.5 * vel^2 * drag coeff. - gravity (?)
            self.y_accel =
                self.starting_y_accel * -1. + 0.5 * (self.velocity).powi(2) * self.drag_coefficient;

            time += dt;

            sim_points.push(SimPoint::new(time, self.y, self.velocity));
            hit = (self.y <= 0. && time > 0.) || self.velocity > self.starting_velocity;
        }

        sim_points
    }
}

impl eframe::App for Symulacja {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let sim_points = self.update_object();
        let time = match sim_points.last() {
            Some(el) => el,
            None => process::exit(1),
        }
        .time;

        let line_yt = line_from!(y => sim_points named "Funkcja y(t)");
        let line_yv = line_from!(vel => sim_points named "Funkcja v(t)");

        window_frame(ctx, frame, "Symulacja", |ui| {
            Plot::new("plot")
                .view_aspect(2.5)
                .legend(Legend::default())
                .show(ui, |pui| {
                    pui.line(line_yt);
                    pui.line(line_yv);
                });

            ui.horizontal_centered(|ui| {
                ui.horizontal(|ui| {
                    ui.add(Label::new(RichText::new("g (m/s^2): ").size(20.0)));

                    ui.add(
                        Slider::new(&mut self.starting_y_accel, 1.0..=100.).clamp_to_range(true),
                    );
                });

                ui.horizontal(|ui| {
                    ui.add(Label::new(RichText::new("v0 (m/s): ").size(20.0)));

                    ui.add(Slider::new(&mut self.starting_velocity, 0.0..=500.));
                });
            });
            ui.horizontal_centered(|ui| {
                ui.horizontal(|ui| {
                    ui.add(Label::new(RichText::new("y0 (m): ").size(20.0)));

                    ui.add(Slider::new(&mut self.starting_y, 0.0..=1000.));
                });

                ui.horizontal(|ui| {
                    ui.add(Label::new(RichText::new("b (wsp. oporu): ").size(20.0)));

                    ui.add(Slider::new(&mut self.drag_coefficient, 0.0..=5.));
                });
            });
            ui.horizontal(|ui| {
                ui.add(Label::new(
                    RichText::new(format!("t (s): {}", time)).size(20.0),
                ));
            });
        });
    }
}
