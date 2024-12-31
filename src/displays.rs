use egui::{pos2, Align2, Color32, FontId, Pos2, Rect, Rounding, Sense, Stroke, Vec2};

pub struct Dial {
    name: String,
    unit: String,
    max: f64,
    min: f64,
}

pub struct Gauge {
    name: String,
    unit: String,
    max: f64,
    min: f64,
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
        let angle = (percent_complete * std::f64::consts::PI * 2.0) as f32;

        let dial_end = Pos2::new(c.x - (angle.cos() * r * 0.9), c.y - (angle.sin() * r * 0.9));
        painter.circle_filled(c, r, Color32::from_rgb(150, 150, 150));
        for i in 0..6 {
            let angle = 2.0 * std::f32::consts::PI * i as f32 / 6.0;
            painter.circle_filled(pos2(c.x + angle.cos() * 0.85 * r, c.y + angle.sin() * 0.85 * r), r / 20.0, Color32::WHITE);

        }
        painter.text(Pos2::new(c.x, c.y + (0.4*r)), Align2::CENTER_CENTER, self.name.clone(), FontId::monospace(8.0), Color32::BLACK);
        painter.text(Pos2::new(c.x, c.y + (0.55*r)), Align2::CENTER_CENTER, self.unit.clone(), FontId::monospace(6.0), Color32::BLACK);
        painter.text(Pos2::new(c.x, c.y - (0.75*r)), Align2::CENTER_CENTER, self.get_at_percent(0.25).to_string(), FontId::monospace(6.0), Color32::WHITE);
        painter.text(Pos2::new(c.x - (0.75*r), c.y), Align2::CENTER_CENTER, self.min.to_string(), FontId::monospace(6.0), Color32::WHITE);
        painter.line(vec!(c, dial_end), stroke);
    }

    fn get_at_percent(&self, percent: f64) -> f64 {
        self.max * percent + (1.0 - percent) * self.min
    }
}

impl Gauge {
    pub fn test() -> Gauge {
        Gauge {
            name: "test".to_string(),
            unit: "na".to_string(),
            max: 1.0,
            min: 0.0,
        }
    }

    pub fn new(name: String, unit: String, max: f64, min: f64) -> Gauge {
        Gauge {
            name, 
            unit, 
            max, 
            min,
        }
    }

    pub fn draw(&self, ui: &mut egui::Ui, current_value: f64) {
        let size = Vec2::new(40.0, 100.0);
        let (response, painter) = ui.allocate_painter(size, Sense::hover());
        let rect = response.rect;
        let c = rect.center();
        let color = Color32::from_rgb(200, 175, 50);

        let border_bottom = c.y + rect.height()/2.0 - 30.0;
        let border_top = c.y - rect.height()/2.0 + 6.0;

        let border_min = Pos2::new(c.x - rect.width()/2.0 + 6.0, border_top);
        let border_max= Pos2::new(c.x + rect.width()/2.0 - 6.0, border_bottom);

        let percent_complete = (current_value - self.min) / (self.max - self.min);

        let indicator_bottom = border_bottom - 3.0;
        let indicator_top = border_top + 3.0;
        let indicator_height = indicator_bottom - indicator_top;
        let visible_top = indicator_bottom - percent_complete as f32 * indicator_height;
        let indicator_tl = Pos2::new(border_min.x + 3.0, visible_top);
        let indicator_br = Pos2::new(border_max.x - 3.0, indicator_bottom);

        painter.rect(rect, Rounding::ZERO, Color32::from_rgb(150, 150, 150), Stroke::NONE);
        painter.rect_stroke(Rect::from_min_max(border_min, border_max), Rounding::ZERO, Stroke::new(3.0, Color32::BLACK));
        painter.rect(Rect::from_min_max(indicator_tl, indicator_br), Rounding::ZERO, color, Stroke::NONE);
        painter.text(Pos2::new(c.x, indicator_bottom + 16.0), Align2::CENTER_CENTER, self.name.clone(), FontId::monospace(8.0), Color32::BLACK);
        painter.text(Pos2::new(c.x, indicator_bottom + 24.0), Align2::CENTER_CENTER, self.unit.clone(), FontId::monospace(6.0), Color32::BLACK);
    }
}