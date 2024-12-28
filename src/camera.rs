use crate::common_math;

pub struct Camera {
    pub position: common_math::Vec3,
    pub euler: common_math::Angles, // euler angles
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: common_math::Vec3::new(0.0, 0.0, 0.0),
            euler: common_math::Angles::new(0.0, 0.0, 0.0),
        }
    }


    pub fn render(&mut self) -> egui::ColorImage {
        let width = 500;
        let height = 500;
        let mut imagebuffer  = Vec::with_capacity(4 * (width + 1) * (height + 1));

        let dpitch = -40.0 / height as f64;
        let droll= -40.0 / height as f64;

        for x in 0..width {
            for y in 0..height {
                let pitch_deg = (x as f64 * dpitch) + 20.0 + self.euler.altitude;
                let mut pitch = common_math::deg_to_rad(pitch_deg);
                pitch += common_math::deg_to_rad(self.euler.roll).tan() * droll * (y as f64 - width as f64/2.0);
                let hypotenuse_length = self.position.z/-pitch.sin();
                let g = (1e6/hypotenuse_length).min(255.0).max(0.0);
                imagebuffer.push(0);
                if y == 0 {
                }
                if pitch < 0.0 {
                    imagebuffer.push(g as u8);
                    //println!("{}", g as u8);
                } else {
                    imagebuffer.push(0);
                }
                imagebuffer.push(0);
                imagebuffer.push(255);
            }
        }
        egui::ColorImage::from_rgba_unmultiplied([width, height], &imagebuffer)
    }
}