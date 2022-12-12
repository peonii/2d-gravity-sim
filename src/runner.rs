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
    // customizable
    pub starting_velocity: f64,
    pub starting_y: f64,
    pub starting_y_accel: f64,
    pub drag_coefficient: f64,
    pub mass: f64,

    // dynamic
    pub y: f64,
    pub velocity: f64,
    pub y_accel: f64,
}

impl Default for Symulacja {
    fn default() -> Self {
        Self {
            // customizable
            starting_y: 0.,
            starting_velocity: 0.,
            starting_y_accel: 9.81,
            drag_coefficient: 0.5,
            mass: 100.,

            // dynamic
            y: 0.,
            velocity: 0.,
            y_accel: 10.,
        }
    }
}

impl Symulacja {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn update_object(&mut self) -> Vec<SimPoint> {
        let mut sim_points: Vec<SimPoint> = Vec::new();

        self.velocity = self.starting_velocity;
        self.y = self.starting_y;
        self.y_accel = -self.starting_y_accel;

        let mut time = 0.;
        let dt = 0.001;

        if self.starting_velocity == 0. && self.starting_y_accel == 0. {
            return sim_points;
        }

        // this is set to true when we've hit the ground (y <= 0)
        let mut hit = false;
        while !hit {
            // y = y - vel
            self.y += self.velocity * dt;
            // vel = vel + accel
            self.velocity += self.y_accel * dt;
            
            // accel = 0.5 * vel^2 * drag coeff. - gravity (?)
            let drag_accel = 0.5 * self.velocity.powi(2) * self.drag_coefficient / self.mass * -self.velocity.signum();

            self.y_accel = -self.starting_y_accel + drag_accel;

            time += dt;

            sim_points.push(SimPoint::new(time, self.y, self.velocity));

            hit = self.y <= 0. && time > 0.;
        }

        return sim_points
    }

    fn inputs(&mut self, ui: &mut egui::Ui, time: f64) {
        ui.add(Label::new(RichText::new("g (m/s^2): ").size(20.0)));
        ui.add(Slider::new(&mut self.starting_y_accel, 1.0..=100.).clamp_to_range(true));
        ui.end_row();

        ui.add(Label::new(RichText::new("y0 (m): ").size(20.0)));
        ui.add(Slider::new(&mut self.starting_y, 0.0..=1000.).clamp_to_range(false));
        ui.end_row();

        ui.add(Label::new(RichText::new("v0 (m/s): ").size(20.0)));
        ui.add(Slider::new(&mut self.starting_velocity, 0.0..=500.).clamp_to_range(false));
        ui.end_row();

        ui.add(Label::new(RichText::new("b (wsp. oporu): ").size(20.0)));
        ui.add(Slider::new(&mut self.drag_coefficient, 0.0..=5.).clamp_to_range(false));
        ui.end_row();

        ui.add(Label::new(RichText::new("m (kg): ").size(20.0)));
        ui.add(Slider::new(&mut self.mass, 0.0..=100000.).logarithmic(true));
        ui.end_row();

        ui.add(Label::new(
            RichText::new("t (s):").size(20.0),
        ));
        ui.add(Label::new(
            RichText::new(format!("{:.2}s", time)).size(20.0),
        ));
        ui.end_row();
    }
}

impl eframe::App for Symulacja {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let sim_points = self.update_object();

        let time = match sim_points.last() {
            Some(s) => s,
            None => process::exit(1),
        }
        .time;

        let line_y = line_from!(y => sim_points named "Funkcja y(t)");
        let line_velocity = line_from!(vel => sim_points named "Funkcja v(t)");

        window_frame(ctx, frame, "Symulacja", |ui| {
            Plot::new("plot")
                .view_aspect(2.5)
                .legend(Legend::default())
                .show(ui, |pui| {
                    pui.line(line_y);
                    pui.line(line_velocity);
                });
            ui.vertical_centered_justified(|ui| {
                egui::Grid::new("inputs")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        self.inputs(ui, time);
                    });
            });
        });
    }
}
