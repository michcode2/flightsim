use crate::common_math::{self, *};

//everything here is relative to the plane

#[derive(Clone, Copy, Debug)]
struct Wing {
    aspect_ratio: f64,
    area: f64,
    pointing: common_math::Angles,
    position: common_math::Vec3,
    control_area: f64,
    CDo: f64,
}

impl Wing {
    fn test() -> Wing {
        Wing {
            aspect_ratio: 7.0,
            area: 10.0,
            pointing: Angles::new(0.0, 0.0, 0.0),
            position: Vec3::new(0.0, 0.0, 0.0),
            control_area: 2.0,
            CDo: 0.01,
        }
    }

    #[allow(non_snake_case)]
    fn calculate_forces(&self, u_inf: &Vec3) -> Vec3 {
        let u_tot = u_inf.magnitude();
        let alpha = (self.pointing.altitude + rad_to_deg(u_inf.angle_with_horizon()))/10.0;

        let CL = alpha.max(-0.8).min(1.2);

        let lift_force = 0.5 * 1.225 * u_tot.powi(2) * self.area * CL;

        let CDi = CL.powi(2)/(std::f64::consts::PI * self.aspect_ratio); 
        let CD = CDi + self.CDo;
        let drag_force = 0.5 * 1.225 * u_tot.powi(2) * self.area * CD;

        Vec3::new(-drag_force, 0.0, lift_force)
    }
}

#[cfg(test)]
mod test {
    use common_math::deg_to_rad;

    use super::*;
    #[test]
    fn zero_speed() {
        let test_wing = Wing::test();
        let no_speed = Vec3::new(0.0, 0.0, 0.0);
        assert_eq!(test_wing.calculate_forces(&no_speed).magnitude(), 0.0);
    }

    #[test]
    fn alpha_of_6() {
        let test_wing = Wing::test();
        let test_velocity = Vec3::new(10.0 * deg_to_rad(6.0).cos(), 0.0, 10.0 * deg_to_rad(6.0).sin());
        let forces = test_wing.calculate_forces(&test_velocity).z;
        assert!((forces - 367.5).abs() < 1e-6);

        let wing_2 = test_wing.clone();
        let mut velocity_2 = test_velocity.clone();
        velocity_2.z *= -1.0;
        let forces_2 = wing_2.calculate_forces(&velocity_2).z;
        assert!((forces_2 - -367.5).abs() < 1e-6);

        let mut wing_cambered = test_wing.clone();
        wing_cambered.pointing.altitude = 3.0;
        let forces_cambered = wing_cambered.calculate_forces(&test_velocity).z;
        assert!((forces_cambered - 551.25).abs() < 1e-6);
    }

    #[test]
    fn alpha_of_14() {
        let test_wing = Wing::test();
        let test_velocity = Vec3::new(10.0 * deg_to_rad(14.0).cos(), 0.0, 10.0 * deg_to_rad(14.0).sin());
        let forces = test_wing.calculate_forces(&test_velocity).z;
        assert!((forces - 735.0).abs() < 1e-6);

        let wing_2 = test_wing.clone();
        let mut velocity_2 = test_velocity.clone();
        velocity_2.z *= -1.0;
        let forces_2 = wing_2.calculate_forces(&velocity_2).z;
        assert!((forces_2 - -490.0).abs() < 1e-6);

        let mut wing_cambered = test_wing.clone();
        wing_cambered.pointing.altitude = 3.0;
        let forces_cambered = wing_cambered.calculate_forces(&test_velocity).z;
        assert!((forces_cambered - 735.0).abs() < 1e-6);
    }
}