use crate::common_math;
use crate::common_math::{Vec3, deg_to_rad};
use crate::state;

pub struct Aircraft {
    pub state: state::State,
    pub throttle_percent: f64,
    mass: f64,
    max_thrust: f64,
}

impl Aircraft {
    pub fn new() -> Aircraft {
        Aircraft {
            state: state::State::runway(),
            throttle_percent: 0.0,
            mass: 1156.0,
            max_thrust: 600.0,
        }
    }

    pub fn flying() -> Aircraft {
        Aircraft {
            state: state::State::flying(),
            throttle_percent: 0.7,
            mass: 1000.0,
            max_thrust: 1000.0,
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
            if self.state.velocity.z < -0.5 {
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
        let thrust = self.max_thrust * self.throttle_percent;
        let thrust_vectors = Vec3::new(thrust, 0.0, 0.0);

        let horizontal_forwards = Vec3::new(self.state.velocity.x, self.state.velocity.y, 0.0);
        let mut climb_angle = self.state.velocity.angle_with(&horizontal_forwards);
        if self.state.velocity.z < 0.0 {
            climb_angle *= -1.0;
        }


        //need a way to transform the lift/drag to account for sideslip

        let velocity_scaled;

        if let Some(unit) = self.state.velocity.unit_vector() {
            velocity_scaled = unit.transform_coordinates(&self.state.pointing_global);
        } else {
            return thrust_vectors;
        }

        let alpha = (deg_to_rad(self.state.pointing_global.altitude) - climb_angle) * 6.0;
        let CL = 1.2_f64.min(alpha).max(-0.8);
        let lift = 0.5 * 1.225 * self.state.velocity.magnitude().powf(2.0) * 16.17 * CL;
        let lift_vectors = Vec3::new(0.0, lift * velocity_scaled.y, lift * velocity_scaled.x);

        let CD = (alpha.powf(2.0)/(std::f64::consts::PI * 7.0)) + 0.035;
        let drag = 0.5*1.225 * self.state.velocity.magnitude().powf(2.0) * 16.17 * CD * 0.3;
        let drag_vectors = Vec3::new(-drag * velocity_scaled.x, -drag * velocity_scaled.y, -drag * velocity_scaled.z);

        
        let resultant = thrust_vectors + &drag_vectors + &lift_vectors;
        resultant
    }

    pub fn increase_throttle(&mut self) {
        self.throttle_percent += 0.05;
        if self.throttle_percent > 1.0 {
            self.throttle_percent = 1.0;  
        }
    }

    pub fn decrease_throttle(&mut self) {
        self.throttle_percent -= 0.05;
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

}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn testit() {
        let mut state = state::State::new();
        state.position.z = 1e5;
        let mut plane = Aircraft {
            state,
            throttle_percent: 0.0,
            mass: 1000.0,
            max_thrust: 1000.0,
        };

        plane.state.velocity.z = 1.0;

        for t in 0..10 {
            println!("{:?}, {}", plane.state.position.z - 1e5, t as f64 * 0.1);
            println!("{:?}, {}", plane.state.acceleration, t as f64 * 0.1);
            println!();
            plane.do_step(0.1);
        }

        assert!(plane.state.acceleration.z > -9.81);
    }
}