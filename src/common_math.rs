use core::f64;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct Angles {
    pub azimouth: f64,
    pub altitude: f64,
    pub roll: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3{
            x,
            y,
            z
        }
    }

    #[allow(dead_code)]
    pub fn magnitude(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).powf(0.5)
    }

    #[allow(dead_code)]
    pub fn unit_vector(&self) -> Option<Vec3> {
        let mag = self.magnitude();
        if mag == 0_f64{
            return None;
        }
        Some(Vec3 {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        })
    }

    pub fn transform_coordinates(&self, pointing: &Angles) ->  Vec3 {
        /*
        takes in a azimouth altitude roll of a plane and returns the relevant scaling factors
        for thrust, this would take in the thrust in body coords and convert it into thrust in world coords
         */
        Vec3{
            x:  self.x * deg_to_rad(pointing.altitude).cos() * deg_to_rad(pointing.azimouth).cos() +
                self.z * deg_to_rad(pointing.roll).sin() * deg_to_rad(pointing.azimouth).sin() - 
                self.y * deg_to_rad(pointing.azimouth).sin() * deg_to_rad(pointing.roll).cos(),

            y:  self.x * deg_to_rad(pointing.altitude).cos() * deg_to_rad(pointing.azimouth).sin() + 
                self.z * deg_to_rad(pointing.roll).sin() + 
                self.y * deg_to_rad(pointing.azimouth).sin() * deg_to_rad(pointing.roll),
                
            z:  self.x * deg_to_rad(pointing.altitude).sin() +
                self.y * deg_to_rad(pointing.roll).sin() + 
                self.z * deg_to_rad(pointing.altitude).cos() * deg_to_rad(pointing.roll).cos()
        }
    }

    pub fn dot_product(&self, other: &Vec3) -> f64 {
        return (self.x * other.x) + (self.y * other.y) + (self.z * other.z);
    }

    pub fn angle_with(&self, other: &Vec3) -> f64 {
        let fraction = self.dot_product(other) / (self.magnitude() * other.magnitude());
        if fraction.is_nan(){
            return f64::consts::PI/2.0;
        }
        return fraction.acos();
    }

    pub fn jsonify(&self) -> String {
        format!("{{\"x\": {}, \"y\": {}, \"z\": {}}}", self.x, self.y, self.z)
    }
}

impl Angles {
    pub fn new(azimouth: f64, altitude: f64, roll: f64) -> Angles {
        Angles {
            azimouth: azimouth,
            altitude: altitude, 
            roll,
        }
    }
    
    pub fn as_vec3(&self) -> Vec3 {
        let x_azimouth = deg_to_rad(self.azimouth).cos();
        let x_altitude = x_azimouth * deg_to_rad(self.altitude).cos();

        let y_azimouth = deg_to_rad(self.azimouth).sin();
        let y_altitude = y_azimouth * deg_to_rad(self.altitude).cos();
        
        let z_altitude = deg_to_rad(self.altitude).sin();

        let x = x_altitude; 
        let y = y_altitude;
        let z = z_altitude;
        Vec3 {x, y, z}
    }
    
    pub fn jsonify(&self) -> String {
        format!("{{\"alt\": {}, \"az\": {}, \"roll\": {}}}", self.altitude, self.azimouth, self.roll)
    }
}

impl std::ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f64) -> Vec3 {
        Vec3{
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl std::ops::Add<&Angles> for Angles {
    type Output = Angles;
    fn add(self, other: &Angles) -> Angles {
        Angles {
            azimouth: self.azimouth + other.azimouth,
            altitude: self.altitude + other.altitude,
            roll: self.roll + other.roll,
        }
    }
}

impl std::ops::Mul<f64> for Angles {
    type Output = Angles;
    fn mul(self, other: f64) -> Angles {
        Angles {
            altitude: self.altitude * other,
            azimouth: self.azimouth * other,
            roll: self.roll * other,
        }
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < 1e-10 && 
        (self.y - other.y).abs() < 1e-10 && 
        (self.z - other.z).abs() < 1e-10 
    }
}

pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0  
}

#[cfg(test)]
mod test{
    use core::f64;

    use super::*;
    #[test]
    fn deg_to_rad_correct() {
        assert_eq!(deg_to_rad(180.0), std::f64::consts::PI);
    }

    #[test]
    fn magnitude_correct() {
        let thing = Vec3::new(2.0, 3.0, 4.0);
        assert_eq!(thing.magnitude(), 29_f64.sqrt())
    }

    #[test]
    fn unit_vector_correct() {
        //TODO regression
    }

    #[test]
    fn coordinate_transform_correct() {
        // case 1: azimouthed 90 degrees to right, thrust pointing forwards
        let test_transform_1 = Vec3::new(1.0, 0.0, 0.0);
        let pointing_1 = Angles::new(90.0, 0.0, 0.0);
        let answer_1 = Vec3::new(0.0, 1.0, 0.0); 
        let check_1 = test_transform_1.transform_coordinates(&pointing_1);
        assert!((answer_1.x - check_1.x).abs() < 1e-10);
        assert!((answer_1.y - check_1.y).abs() < 1e-10);
        assert!((answer_1.z - check_1.z).abs() < 1e-10);
        assert!((answer_1.magnitude() - 1.0).abs() < 1e-10);

        // case 2: azimouthed 90 degrees to left, thrust pointing forwards
        let test_transform_2 = Vec3::new(1.0, 0.0, 0.0);
        let pointing_2 = Angles::new(-90.0, 0.0, 0.0);
        let answer_2 = Vec3::new(0.0, -1.0, 0.0); 
        let check_2 = test_transform_2.transform_coordinates(&pointing_2);
        assert!((answer_2.x - check_2.x).abs() < 1e-10);
        assert!((answer_2.y - check_2.y).abs() < 1e-10);
        assert!((answer_2.z - check_2.z).abs() < 1e-10);
        assert!((answer_2.magnitude() - 1.0).abs() < 1e-10);
        
        // case 3: azimouthed 90 degrees to right, thrust pointing right
        let test_transform_3 = Vec3::new(0.0, 1.0, 0.0);
        let pointing_3 = Angles::new(90.0, 0.0, 0.0);
        let answer_3 = Vec3::new(-1.0, 0.0, 0.0); 
        let check_3 = test_transform_3.transform_coordinates(&pointing_3);
        assert!((answer_3.x - check_3.x).abs() < 1e-10);
        assert!((answer_3.y - check_3.y).abs() < 1e-10);
        assert!((answer_3.z - check_3.z).abs() < 1e-10);
        assert!((answer_3.magnitude() - 1.0).abs() < 1e-10);

        // case 4: azimouthed 90 degrees to the left, thrust pointing right
        let test_transform_4 = Vec3::new(0.0, 1.0, 0.0);
        let pointing_4 = Angles::new(-90.0, 0.0, 0.0);
        let answer_4 = Vec3::new(1.0, 0.0, 0.0); 
        let check_4 = test_transform_4.transform_coordinates(&pointing_4);
        assert!((answer_4.x - check_4.x).abs() < 1e-10);
        assert!((answer_4.y - check_4.y).abs() < 1e-10);
        assert!((answer_4.z - check_4.z).abs() < 1e-10);
        assert!((answer_4.magnitude() - 1.0).abs() < 1e-10);

        // case 5: altitude 45 degrees up and azimouth 45 degrees to the right, thrust pointing forwards
        let test_transform_5 = Vec3::new(1.0, 0.0, 0.0);
        let pointing_5 = Angles::new(45.0, 45.0, 0.0);
        let answer_5 = Vec3::new(1.0 / 2.0_f64.sqrt() * 1.0 / 2.0_f64.sqrt(), 1.0 / 2.0_f64.sqrt() * 1.0 / 2.0_f64.sqrt(), 1.0 / 2.0_f64.sqrt()); 
        let check_5 = test_transform_5.transform_coordinates(&pointing_5);
        assert!((answer_5.x - check_5.x).abs() < 1e-10);
        assert!((answer_5.y - check_5.y).abs() < 1e-10);
        assert!((answer_5.z - check_5.z).abs() < 1e-10);
        assert!((answer_5.magnitude() - 1.0).abs() < 1e-10);
        
        // case 6: altitude 10 degrees up and roll 10 degrees left
        let test_transform_6 = Vec3::new(1.0, 0.0, 0.0);
        let pointing_6 = Angles::new(0.0, 10.0, 10.0);
        let answer_6 = Vec3::new(deg_to_rad(10.0).cos(), 0.0, deg_to_rad(10.0).sin()); 
        let check_6 = test_transform_6.transform_coordinates(&pointing_6);
        assert!((answer_6.x - check_6.x).abs() < 1e-10);
        assert!((answer_6.y - check_6.y).abs() < 1e-10);
        assert!((answer_6.z - check_6.z).abs() < 1e-10);
        assert!((answer_6.magnitude() - 1.0).abs() < 1e-10);

        // case 7
        let test_transform_7 = Vec3::new(1.0, 0.0, 0.0);
        let pointing_7 = Angles::new(0.0, -10.0, 0.0);
        let answer_7 = Vec3::new(deg_to_rad(-10.0).cos(), 0.0, deg_to_rad(-10.0).sin()); 
        let check_7 = test_transform_7.transform_coordinates(&pointing_7);
        assert!((answer_7.x - check_7.x).abs() < 1e-10);
        assert!((answer_7.y - check_7.y).abs() < 1e-10);
        assert!((answer_7.z - check_7.z).abs() < 1e-10);
        assert!((answer_7.magnitude() - 1.0).abs() < 1e-10);
        //TODO extend test suite
    }

    #[test]
    fn test_angle_with() {
        let vec1_1 = Vec3::new(0.0, 1.0, 0.0);
        let vec1_2 = Vec3::new(0.0, -1.0, 0.0);
        println!("{}", vec1_1.angle_with(&vec1_2)/f64::consts::PI);
        let vec2_1 = Vec3::new(1.0, 0.0, 0.0);
        println!("{}", vec1_1.angle_with(&vec2_1)/f64::consts::PI);
        let vec2_2 = Vec3::new(1.0, 0.0, 1.0).unit_vector().unwrap();
        println!("{}, {:?}", vec2_1.angle_with(&vec2_2)/f64::consts::PI, vec2_2)
    }

    #[test]
    fn test_angle_to_vec3() {
        assert_eq!(Angles::new(0.0, 0.0, 0.0).as_vec3(), 
            Vec3::new(1.0, 0.0, 0.0)
        );

        assert_eq!(Angles::new(0.0, 90.0, 0.0).as_vec3(), 
            Vec3::new(0.0, 0.0, 1.0)
        );

        assert_eq!(Angles::new(90.0, 0.0, 0.0).as_vec3(), 
            Vec3::new(0.0, 1.0, 0.0)
        );
    }
}