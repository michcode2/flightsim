use crate::common_math;
use crate::common_math::Vec3;
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
            state: state::State::new(),
            throttle_percent: 0.0,
            mass: 1000.0,
            max_thrust: 1750.0,
        }
    }

    pub fn do_step(&mut self, dt: f64) {
        let mut next_acceleration = self.free_body_diagram().transform_coordinates(&self.state.pointing) * (1.0/self.mass);
        let mut next_velocity = self.state.velocity + &(next_acceleration * dt);
        let mut next_position = self.state.position + &(next_velocity * dt);
        let next_pointing = self.state.pointing + &(self.state.angular_rate * dt);

        if next_position.z <= 0.0 {
            next_position.z = self.state.position.z.max(0.0);
            next_acceleration.z = self.state.acceleration.z.max(0.0);
            next_velocity.z = self.state.velocity.z.max(0.0);
        }
        
        self.state = state::State {
            position: next_position,
            velocity: next_velocity,
            pointing: next_pointing,
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


        let alpha = (common_math::deg_to_rad(self.state.pointing.altitude) - climb_angle) * 10.0;

        let CL = 1.2_f64.min(alpha).max(-0.8);
        let lift = 0.5 * 1.225 * self.state.velocity.magnitude().powf(2.0) * 10.0 * CL;
        let lift_vectors = Vec3::new(0.0, 0.0, lift);

        let CD = CL.powf(2.0)/20.0 + 0.02;
        let drag = 0.5*1.225 * self.state.velocity.magnitude().powf(2.0) * 10.0 * CD;
        let drag_vectors = Vec3::new(-drag, 0.0, 0.0);


        let weight = self.mass * 9.81;
        
        let weight_vector = Vec3::new(0.0, 0.0, -weight);
        
        thrust_vectors + &drag_vectors + &lift_vectors + &weight_vector
    }

    pub fn increase_throttle(&mut self) {
        self.throttle_percent += 0.1;
        if self.throttle_percent > 1.0 {
            self.throttle_percent = 1.0;  
        }
    }

    pub fn decrease_throttle(&mut self) {
        self.throttle_percent -= 0.1;
        if self.throttle_percent < 0.0 {
            self.throttle_percent = 0.0;  
        }
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