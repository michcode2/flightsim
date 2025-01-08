use crate::{aircraft, common_math};
use common_math::deg_to_rad;

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


    pub fn render(&mut self, aircraft_pitch: f64, aircraft_yaw: f64) -> egui::ColorImage {
        let width = 500;
        let height = 300;
        let mut imagebuffer  = Vec::with_capacity(4 * (width + 1) * (height + 1));

        let delta_angle = -2.0/15.0;

        let dpitch = delta_angle;
        let dyaw = delta_angle;

        let z = self.position.z + 1.0;

        for x in 0..height {
            for y in 0..width {
                let yaw_deg = (y as f64 * dyaw) - (dyaw * width as f64 * 0.5) + self.euler.azimouth;
                let mut pixel_yaw = deg_to_rad(yaw_deg);
                let roll_offset_yaw = deg_to_rad(-self.euler.roll) * pixel_yaw.acos();
                //pixel_yaw += roll_offset_yaw;
                
                let pitch_deg = (x as f64 * dpitch) + 20.0 + self.euler.altitude;
                let roll_offset_pitch = deg_to_rad(-self.euler.roll) * pixel_yaw.asin();
                let mut pixel_pitch = deg_to_rad(pitch_deg);
                pixel_pitch += roll_offset_pitch;

                let mut colours = vec![0,0,0];
                
                if pixel_pitch < 0.0 {
                    let intersect_distance_tot = z/pixel_pitch.tan();

                    let delta = deg_to_rad(dpitch);

                    let x_plusabit = self.position.x - (z / (pixel_pitch + delta).tan() * pixel_yaw.cos()); 
                    let x_minusabit = self.position.x - (z / (pixel_pitch - delta).tan() * pixel_yaw.cos()); 

                    let horizon_colour = (intersect_distance_tot * 0.01).abs() as u8;

                    if (x_plusabit / 100.0) as isize % 2 == 0 && (x_minusabit / 100.0) as isize % 2 == 0{
                        colours[0] = horizon_colour.min(50);
                        colours[1] = 200;
                        colours[2] = horizon_colour.min(50);
                    } else {
                        colours[0] = horizon_colour.min(50);
                        colours[1] = 150_u8.saturating_add(horizon_colour.saturating_mul(2));
                        colours[2] = horizon_colour.min(50);
                    } 
                    let y_intercept = self.position.y - (intersect_distance_tot * (pixel_yaw).sin());
                    if (y_intercept / 100.0) as isize % 2 == 0 {
                        colours[0] = 138;
                        colours[2] = 150;
                    }

                } else {
                    colours[0] = 25;
                    colours[1] = 50;
                    colours[2] = 200;
                }
                
                if (pixel_pitch - aircraft_pitch).abs() < deg_to_rad(0.1) {
                    colours[0] = 0;
                    colours[1] = 255;
                    colours[2] = 0;
                }
    
                if (pixel_yaw - aircraft_yaw).abs() < deg_to_rad(0.1) {
                    colours[0] = 0;
                    colours[1] = 255;
                    colours[2] = 0;
                }



                imagebuffer.push(colours[0]);
                imagebuffer.push(colours[1]);
                imagebuffer.push(colours[2]);
                imagebuffer.push(255);
            }
        }
        egui::ColorImage::from_rgba_unmultiplied([width, height], &imagebuffer)
    }
}