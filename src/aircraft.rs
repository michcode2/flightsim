use crate::common_math::{Vec3, deg_to_rad, rad_to_deg};
use crate::state;

pub struct Aircraft {
    pub state: state::State,
    pub throttle_percent: f64,
    mass: f64,
    max_power: f64,
    area: f64,
}

impl Aircraft {
    pub fn new() -> Aircraft {
        Aircraft {
            state: state::State::runway(),
            throttle_percent: 0.0,
            mass: 1156.0,
            max_power: 120e3,
            area: 16.17,
        }
    }

    pub fn flying() -> Aircraft {
        Aircraft {
            state: state::State::flying(),
            throttle_percent: 0.7,
            mass: 1000.0,
            max_power: 120e3,
            area: 16.17,
        }
    }
    
    pub fn flying_high() -> Aircraft {
        Aircraft {
            state: state::State::flying_high(),
            throttle_percent: 0.7,
            mass: 1000.0,
            max_power: 120e3,
            area: 16.17,
        }
    }

    pub fn do_step(&mut self, dt: f64) {
        // this also needs something for the combo roll and pitch to give yaw
        let weight = self.mass * 9.81;
        let weight_vector = Vec3::new(0.0, 0.0, -weight);

        let self_forces = self.free_body_diagram().transform_coordinates(&self.state.pointing_global);
        let mut next_acceleration = (self_forces + &weight_vector) * (1.0/self.mass);

        let mut next_velocity = self.state.velocity + &(next_acceleration * dt);
        let mut next_position = self.state.position + &(next_velocity * dt);
        let next_pointing_global = self.state.pointing_global + &(self.state.angular_rate * dt);

        if next_position.z <= 0.0 {
            if self.state.velocity.z < -1. {
                panic!("shit landing dumbass. {} meters per second", self.state.velocity.z);
            }
            if self.state.velocity.z < 0.0 {
                println!("{} is acceptable", self.state.velocity.z);
            }
            next_position.z = self.state.position.z.max(0.0);
            next_acceleration.z = self.state.acceleration.z.max(0.0);
            next_velocity.z = self.state.velocity.z.max(0.0);
        }
        
        self.state = state::State {
            position: next_position,
            velocity: next_velocity,
            pointing_global: next_pointing_global,
            acceleration: next_acceleration,
            angular_rate: self.state.angular_rate,
        }
    }

    #[allow(non_snake_case)]
    fn free_body_diagram(&mut self) -> Vec3 {
        let thrust = self.calculate_thrust();
        let thrust_vectors = Vec3::new(thrust, 0.0, 0.0);

        let alpha = self.get_alpha();
        let CL = 1.2_f64.min(rad_to_deg(alpha)/10.0).max(-0.8);
        let lift = 0.5 * 1.225 * self.state.velocity.magnitude().powf(2.0) * self.area * CL;
        let lift_vectors = Vec3::new(0.0, 0.0, lift);

        let CD = (alpha.powf(2.0)/(std::f64::consts::PI * 7.0)) + 0.05;
        let drag = 0.5*1.225 * self.state.velocity.magnitude().powf(2.0) * self.area * CD;
        let drag_vectors = Vec3::new(-drag * alpha.cos(), 0.0, drag * alpha.sin());
        //let drag_vectors = Vec3::new(-drag, 0.0, 0.0);

        let resultant = thrust_vectors + &drag_vectors + &lift_vectors;
        resultant
    }

    pub fn calculate_thrust(&self) -> f64 {
        let power = self.max_power * self.throttle_percent;
        power / (self.state.velocity.magnitude())
    }

    pub fn throttle_by(&mut self, amount: f64) {
        self.throttle_percent += amount;
        if self.throttle_percent > 1.0 {
            self.throttle_percent = 1.0;  
        }
        if self.throttle_percent < 0.0 {
            self.throttle_percent = 0.0;  
        }
    }


    pub fn pitch_by(&mut self, amount: f64) {
        let delta_pitch = amount * deg_to_rad(self.state.pointing_global.roll).cos();
        let delta_yaw = amount * deg_to_rad(self.state.pointing_global.roll).sin();

        self.state.pointing_global.altitude += delta_pitch;
        self.state.pointing_global.azimouth += delta_yaw;
    }

    pub fn yaw_by(&mut self, amount: f64 ){
        let delta_pitch = amount * deg_to_rad(self.state.pointing_global.roll).sin();
        let delta_yaw = amount * deg_to_rad(self.state.pointing_global.roll).cos();

        self.state.pointing_global.altitude += delta_pitch;
        self.state.pointing_global.azimouth += delta_yaw;
    }

    pub fn get_alpha(&self) -> f64 {
        let climb_rate = self.state.velocity.angle_with_horizon();
        deg_to_rad(self.state.pointing_global.altitude) - climb_rate 
    }

    pub fn get_sideslip(&self) -> f64 {
        0.0
    }

}

#[cfg(test)]
mod test {
    use state::State;
    use crate::common_math::{rad_to_deg, Angles};

    use super::*;
    use std::io::Write;
    use std::os::macos::raw::stat;
    use std::process::Command;
    
    #[test]
    fn pitching_up() {
        // when pitching up, there should be a strictly decreasing velocity
        let mut results_string = String::new();

        let mut state = State::new();
        state.position.z = 1e10;
        state.velocity.x = 50.0;
        let mut plane = Aircraft {
            state,
            throttle_percent: 0.0,
            mass: 1156.0,
            max_power: 120e3,
            area: 16.17,
        };
        for a in 0..90 {

            plane.state.pointing_global.altitude = a as f64;
            plane.do_step(0.01);
            results_string.push_str(plane.state.log().as_str());
            results_string.push_str(",");
        }
    
        std::fs::remove_file("pitch_up.json").unwrap();
        let mut results_file = std::fs::OpenOptions::new().write(true).create(true).open("pitch_up.json").unwrap();
        results_file.write("{\"data\": [".as_bytes()).unwrap();
        results_string.pop();
        results_string.push_str("]}");
        results_file.write(results_string.as_bytes()).unwrap();

        // run the python script and make sure its right
        let mut command = Command::new("python");
        command.arg("python_tests/pitch_up.py");
        assert!(command.status().unwrap().success());
    }

    #[test]
    fn test_alpha() {
        let state = State {
            pointing_global: Angles::new(0.0, 0.0, 0.0),
            position: Vec3::new(0.0, 0.0, 0.0),
            angular_rate: Angles::new(0.0, 0.0, 0.0),
            velocity: Vec3::new(10.0, 0.0, 1.0),
            acceleration: Vec3::new(0.0, 0.0, 0.0),
        };
        let mut plane = Aircraft {
            state,
            throttle_percent: 0.0,
            mass: 10.0,
            max_power: 1.0,
            area: 1.0,
        };
        let target = -0.1_f64.atan();
        let answer = plane.get_alpha();
        assert!((target - answer).abs() < 1e-6);

        plane.state.velocity = Vec3::new(0.0, 10.0, 1.0);
        let target = -0.1_f64.atan();
        let answer = plane.get_alpha();
        assert!((target - answer).abs() < 1e-6);
    }
}