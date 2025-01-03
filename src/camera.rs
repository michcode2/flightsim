use chrono::format::InternalFixed;

use crate::common_math;
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


    pub fn render(&mut self) -> egui::ColorImage {
        let width = 500;
        let height = 300;
        let mut imagebuffer  = Vec::with_capacity(4 * (width + 1) * (height + 1));

        let delta_angle = -2.0/15.0;

        let dpitch = delta_angle;
        let droll= delta_angle;
        let dyaw = delta_angle;

        let z = self.position.z + 1.0;

        for x in 0..height {
            for y in 0..width {
                

                let yaw_deg = (y as f64 * dyaw) - (dyaw * width as f64 * 0.5) + self.euler.azimouth;
                let pixel_yaw = deg_to_rad(yaw_deg);
                
                let pitch_deg = (x as f64 * dpitch) + 20.0 + self.euler.altitude;
                let roll_offset = deg_to_rad(-self.euler.roll) * pixel_yaw;
                let mut pixel_pitch = deg_to_rad(pitch_deg);
                pixel_pitch += roll_offset;

                let mut colours = vec![0,0,0];
                
                if pixel_pitch < 0.0 {
                    let intersect_distance_x = z/(pixel_pitch.tan() * deg_to_rad(self.euler.azimouth).cos());
                    let intersect_distance_y = z/(pixel_pitch.tan() * deg_to_rad(self.euler.azimouth.sin()));
                    let intersect_distance_tot = z/pixel_pitch.tan();

                    let delta = deg_to_rad(dpitch);

                    let x_plusabit = self.position.x - (z / (pixel_pitch + delta).tan() * pixel_yaw.cos()); 
                    let x_minusabit = self.position.x - (z / (pixel_pitch - delta).tan() * pixel_yaw.cos()); 

                    let horizon_colour = (intersect_distance_tot * 0.01).abs() as u8;

                    if (x_plusabit / 100.0) as isize % 2 == 0 && (x_minusabit / 100.0) as isize % 2 == 0{
                        colours[0] = horizon_colour.min(50);
                        colours[1] = 255;
                        colours[2] = horizon_colour.min(50);
                    } else {
                        colours[0] = horizon_colour.min(50);
                        colours[1] = 128_u8.saturating_add(2*horizon_colour);
                        colours[2] = horizon_colour.min(50);
                    } 
                    let y_intercept = self.position.y - (intersect_distance_tot * (pixel_yaw).sin());
                    if (y_intercept / 50.0) as isize % 2 == 0 {
                        colours[0] = 75;
                        colours[2] = 100;
                    }

                } else {
                    colours[0] = 25;
                    colours[1] = 50;
                    colours[2] = 200;
                }

                imagebuffer.push(colours[0]);
                imagebuffer.push(colours[1]);
                imagebuffer.push(colours[2]);
                imagebuffer.push(128);
            }
        }
        egui::ColorImage::from_rgba_unmultiplied([width, height], &imagebuffer)
    }
}