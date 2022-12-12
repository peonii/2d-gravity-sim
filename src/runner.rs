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

    // constant
    pub rho: f64, // density
    pub mass: f64,
    pub area: f64,

    // calculated at startup
    pub terminal_vel: f64,

    // dynamic
    pub y: f64,
    pub velocity: f64,
    pub y_accel: f64,
}

pub enum SimulationResult {
    Some(Vec<SimPoint>),
    None(Vec<SimPoint>),
    Terminal(Vec<SimPoint>),
}

impl Default for Symulacja {
    fn default() -> Self {
        Self {
            // customizable
            starting_y: 0.,
            starting_velocity: 0.,
            starting_y_accel: 9.81,
            drag_coefficient: 0.5,

            // constant
            rho: 1.,
            mass: 100.,
            area: 1.,

            // calculated at startup
            terminal_vel: -1.,

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
        let mut this = Self::default();

        this.terminal_vel = ((2. * this.mass * this.starting_y_accel)
            / (this.rho * this.area * this.drag_coefficient))
            .sqrt();

        this
    }

    fn update_object(&mut self) -> SimulationResult {
        let mut sim_points: Vec<SimPoint> = Vec::new();

        self.velocity = self.starting_velocity;
        self.y = self.starting_y;
        self.y_accel = -self.starting_y_accel;

        let mut time = 0.;
        let dt = 0.001;

        if self.starting_velocity == 0. && self.starting_y_accel == 0. {
            return SimulationResult::None(sim_points);
        }

        // this is set to true when we've hit the ground (y <= 0)
        let mut hit = false;
        while !hit {
            // y = y - vel
            self.y += self.velocity * dt;
            // vel = vel + accel
            self.velocity += self.y_accel * dt;

            // accel = 0.5 * vel^2 * drag coeff. - gravity (?)
            let drag_accel =
                (0.5 * (self.velocity).powi(2) * self.drag_coefficient * self.rho * self.area)
                    / self.mass
                    * self.velocity.signum();

            self.y_accel = -self.starting_y_accel + drag_accel;

            time += dt;

            sim_points.push(SimPoint::new(time, self.y, self.velocity));

            /*
            if self.velocity > self.terminal_vel {
                return SimulationResult::Terminal(sim_points);
            }
            */

            hit = self.y <= 0. && time > 0.;
        }

        SimulationResult::Some(sim_points)
    }
}

impl eframe::App for Symulacja {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let sim_res = self.update_object();

        let has_terminated = matches!(sim_res, SimulationResult::Terminal(_));

        let sim_points = match sim_res {
            SimulationResult::Some(s) => s,
            SimulationResult::None(s) => s,
            SimulationResult::Terminal(s) => s,
        };

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

            ui.horizontal(|ui| {
                ui.add(Label::new(RichText::new("g (m/s^2): ").size(20.0)));

                ui.add(Slider::new(&mut self.starting_y_accel, 1.0..=100.).clamp_to_range(true));
            });

            ui.horizontal(|ui| {
                ui.add(Label::new(RichText::new("v0 (m/s): ").size(20.0)));

                ui.add(Slider::new(&mut self.starting_velocity, 0.0..=500.));
            });
            ui.horizontal(|ui| {
                ui.add(Label::new(RichText::new("y0 (m): ").size(20.0)));

                ui.add(Slider::new(&mut self.starting_y, 0.0..=1000.));
            });

            ui.horizontal(|ui| {
                ui.add(Label::new(RichText::new("b (wsp. oporu): ").size(20.0)));

                ui.add(Slider::new(&mut self.drag_coefficient, 0.0..=5.));
            });
            ui.horizontal(|ui| {
                ui.add(Label::new(
                    RichText::new(format!("t (s): {}", time)).size(20.0),
                ));
            });

            if has_terminated {
                ui.horizontal(|ui| {
                    ui.add(Label::new(
                        RichText::new("We've hit terminal velocity!").size(20.0),
                    ));
                });
            }
        });
    }
}
