use eframe::egui;

pub fn window_frame(
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    title: &str,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    use egui::*;

    let height = 28.;
    let text_color = ctx.style().visuals.text_color();

    CentralPanel::default()
        .frame(Frame::none())
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            let painter = ui.painter();

            painter.rect(
                rect.shrink(1.),
                10.0,
                ctx.style().visuals.window_fill(),
                Stroke::new(0., ctx.style().visuals.faint_bg_color),
            );

            painter.text(
                rect.center_top() + vec2(0.0, height / 2.),
                Align2::CENTER_CENTER,
                title,
                FontId::proportional(height * 0.8),
                text_color,
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
