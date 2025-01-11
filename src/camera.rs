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

        //let width = 500/10;
        //let height = 300/10;

        let mut imagebuffer  = Vec::with_capacity(4 * (width + 1) * (height + 1));

        let delta_angle = deg_to_rad(-2.0/15.0);

        let dpitch = delta_angle;
        let dyaw = delta_angle;

        let z = self.position.z + 1.0;

        for y in 0..height {
            for x in 0..width {

                let y_offset = y as f64 - (height as f64 / 2.0);
                let x_offset = x as f64 - (width as f64 / 2.0);
                let _theta = deg_to_rad(x_offset).atan2(deg_to_rad(y_offset));

                let sensor_pitch = y_offset * dpitch; // radians
                let sensor_yaw = x_offset * dyaw;

                let pitch_offset = -sensor_yaw * deg_to_rad(self.euler.roll).tan();

                let pixel_pitch = (sensor_pitch) + deg_to_rad(self.euler.altitude) + pitch_offset;
                let pixel_yaw= (sensor_yaw) + deg_to_rad(self.euler.azimouth);

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
                
                if (pixel_pitch - aircraft_pitch).abs() < deg_to_rad(0.5) && (pixel_pitch - aircraft_pitch).abs() > deg_to_rad(0.2) && (pixel_yaw - aircraft_yaw).abs() < deg_to_rad(0.5) && (pixel_yaw - aircraft_yaw).abs() > deg_to_rad(0.2) {
                    colours[0] = 255;
                    colours[1] = 0;
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