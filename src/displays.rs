use egui::{Align2, Color32, FontId, Pos2, Sense, Stroke, Vec2};

pub struct Dial {
    name: String,
    unit: String,
    max: f64,
    min: f64
}

impl Dial {
    pub fn test() -> Dial {
        Dial {
            name: "test".to_string(),
            unit: "m/s".to_string(),
            max: 20.0,
            min: -5.0,
        }
    }

    pub fn new(name: String, unit: String, max: f64, min: f64) -> Dial {
        Dial {
            name,
            unit,
            max,
            min,
        }
    }

    pub fn draw(&self, ui: &mut egui::Ui, current_value: f64) {
            let size = Vec2::splat(100.0);
            let (response, painter) = ui.allocate_painter(size, Sense::hover());
            let rect = response.rect;
            let c = rect.center();
            let r = rect.width() / 2.0 - 1.0;
            let color = Color32::from_rgb(200, 175, 50);
            let stroke = Stroke::new(2.0, color);

            let percent_complete = (current_value - self.min) / (self.max - self.min);
            let angle = (percent_complete * std::f64::consts::PI * 3.0 / 2.0) as f32;
            let dial_end = Pos2::new(c.x - (angle.cos() * r * 0.9), c.y - (angle.sin() * r * 0.9));

            painter.circle_filled(c, r, Color32::from_rgb(200,200, 200));
            painter.text(Pos2::new(c.x, c.y + (0.5*r)), Align2::CENTER_CENTER, self.name.clone(), FontId::monospace(8.0), Color32::from_rgb(0, 0, 0));
            painter.line(vec!(c, dial_end), stroke);
    }
}