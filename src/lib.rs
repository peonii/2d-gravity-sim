use eframe::egui::{self, plot::{Plot, Line, PlotPoints, Legend}, RichText, Label, Slider};


pub struct Symulacja {
    y: f64,
    yv: f64,
    syv: f64,
    sy: f64,
    gravity: f64,
}

impl Default for Symulacja {
    fn default() -> Self {
        Self {
            y: 0.,
            yv: 0.,
            syv: 0.,
            sy: 0.,
            gravity: 10.,
        }
    }
}

impl eframe::App for Symulacja {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let time = ((self.syv * self.syv + self.sy * 2. * self.gravity).sqrt() + self.syv) / self.gravity;


        let time_range = 0..((time * 1000.) as i32);
        let tr_2 = time_range.clone(); // thisi s so stupid

        self.yv = -self.syv;
        self.y = self.sy;
        // y(t)
        let object_yt = time_range.map(|i| {
            let t = f64::from(i) / 1000.;
            self.y -= self.yv / 1000.;
            self.yv += self.gravity / 1000.;

            if self.y >= 0. {
                return [t, self.y as f64]
            } else {
                return [t, 0.]
            }
        });

        let line_yt = Line::new(PlotPoints::from_iter(object_yt)).name("Funkcja y(t)");

        self.yv = -self.syv;
        self.y = self.sy;

        let object_yv = tr_2.map(|i| {
            let t = f64::from(i) / 1000.;
            self.y -= self.yv / 1000.;
            self.yv += self.gravity / 1000.;

            if self.y >= 0. {
                return [t, -self.yv as f64]
            } else {
                return [t, 0.]
            }
        });
        
        let line_yv = Line::new(PlotPoints::from_iter(object_yv)).name("Funkcja v(t)");

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
                    ui.add(
                        Label::new(RichText::new("g (m/s^2): ").size(20.0))
                    );

                    ui.add(
                        Slider::new(&mut self.gravity, 1.0..=100.).clamp_to_range(true)
                    );
                });


                ui.horizontal(|ui| {
                    ui.add(
                        Label::new(RichText::new("v0 (m/s): ").size(20.0))
                    );

                    ui.add(
                        Slider::new(&mut self.syv, 0.0..=1000.)
                    );
                });


                ui.horizontal(|ui| {
                    ui.add(
                        Label::new(RichText::new("y0 (m): ").size(20.0))
                    );

                    ui.add(
                        Slider::new(&mut self.sy, 0.0..=1000.)
                    );
                });

            });
            
            ui.horizontal(|ui| {
                ui.add(
                    Label::new(RichText::new(format!("t (s): {}", time)).size(20.0))
                );
            });
        });
    }
}

fn window_frame(ctx: &egui::Context, frame: &mut eframe::Frame, title: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
    use egui::*;

    let height = 28.;
    let text_color = ctx.style().visuals.text_color();

    CentralPanel::default()
        .frame(Frame::none())
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            let painter = ui.painter();

            painter.rect(rect.shrink(1.), 10.0, ctx.style().visuals.window_fill(), Stroke::new(0., ctx.style().visuals.faint_bg_color));

            painter.text(
                rect.center_top() + vec2(0.0, height / 2.),
                Align2::CENTER_CENTER,
                title,
                FontId::proportional(height * 0.8),
                text_color
            );

             let close_response = ui.put(
                Rect::from_min_size(rect.left_top(), Vec2::splat(height)),
                Button::new(RichText::new("❌").size(height - 4.0)).frame(false),
            );

            if close_response.clicked() {
                frame.close();
            }

            let title_bar_rect = {
                let mut rect = rect;
                rect.max.y = rect.min.y + height;
                rect
            };

            let title_bar_response =
                ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());
            if title_bar_response.is_pointer_button_down_on() {
                frame.drag_window();
            }

            let content_rect = {
                let mut rect = rect;
                rect.min.y = title_bar_rect.max.y;
                rect
            }
            .shrink(4.0);
            let mut content_ui = ui.child_ui(content_rect, *ui.layout());
            add_contents(&mut content_ui);
        });
}