use crate::common_math::*;

#[allow(non_snake_case)]
pub struct Wing {
    position: Vec3,
    area: f64,
    aspect_ratio: f64,
    CDo: f64,
    pointing: Angles,
}

impl Wing {
    #[allow(non_snake_case)]
    fn aero_forces(&self, freestream: &Vec3) -> Vec3 {
        let U_inf = freestream.magnitude();
        let alpha = rad_to_deg(freestream.angle_with_horizon()) + self.pointing.altitude;
        let CL = match alpha {
            12.0..std::f64::MAX => 14.0 / alpha,
            std::f64::MIN..-8.0 => 0.8,
            _ => alpha / 10.0,
        };

        let lift = Vec3::new(0.0, 0.0, 0.5 * 1.225 * U_inf.powi(2) * self.area * CL);

        let CDi = CL.powi(2) / (std::f64::consts::PI * self.aspect_ratio);
        let CD = CDi + self.CDo;
        let drag = Vec3::new(-0.5 * 1.225 * U_inf.powi(2) * self.area * CD, 0.0, 0.0);

        (lift + drag).transform_coordinates(&self.pointing)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn alpha_of_6(){
        let test_wing = Wing {
            position: Vec3::new(1.0, 0.0, 0.0),
            area: 1.0,
            aspect_ratio: 7.0,
            pointing: Angles::new(0.0, 0.0, 0.0),
            CDo: 0.01,
        };

        let test_velocity = Vec3::new(10.0 * deg_to_rad(6.0).cos(), 0.0, 10.0 * deg_to_rad(6.0).sin());
        let test_forces = test_wing.aero_forces(&test_velocity);
        let lift_result = 0.5 * 1.225 * 100.0 * 1.0 * 0.6;
        assert!((test_forces.z - lift_result).abs() < 1e-6);

        let test_velocity_down = Vec3 {
            z: -10.0 * deg_to_rad(6.0).sin(),
            ..test_velocity
        };
        let test_forces_down = test_wing.aero_forces(&test_velocity_down);
        let downforce_result = 0.5 * 1.225 * 100.0 * 1.0 * -0.6;
        assert!((test_forces_down.z - downforce_result).abs() < 1e-6);

        let test_setting = Wing {
            pointing: Angles::new(0.0, 3.0, 0.0),
            ..test_wing
        };
        let test_forces2 = test_setting.aero_forces(&test_velocity);
        let lift_result2 = 0.5 * 1.225 * 100.0 * 0.9;
        let CD = 0.01 + (0.9 * 0.9) / (std::f64::consts::PI * 7.0);
        let drag_result2 = 0.5 * 1.225 * 100.0 * CD;
        let z_force = lift_result2 * deg_to_rad(3.0).cos() - drag_result2 * deg_to_rad(3.0).sin();
        assert!((test_forces2.z - z_force).abs() < 1e-6);
    
        let test_setting_forces2 = test_setting.aero_forces(&test_velocity_down);
        let downforce_result_setting = (0.5 * 1.225 * 100.0 * 1.0 * -0.3);
        let CD2 = 0.01 + (-0.3 * -0.3) / (std::f64::consts::PI * 7.0);
        let drag_result2 = 0.5 * 1.225 * 100.0 * CD2;
        let z_force2 = downforce_result_setting* deg_to_rad(3.0).cos() - drag_result2 * deg_to_rad(3.0).sin();
        assert!((test_setting_forces2.z - z_force2).abs() < 1e-6)

    }
}