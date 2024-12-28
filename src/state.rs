use crate::common_math;

pub struct State {
    pub position: common_math::Vec3,
    pub pointing: common_math::Angles,
    pub angular_rate: common_math::Angles,
    pub velocity: common_math::Vec3,
    pub acceleration: common_math::Vec3
}

impl State {
    pub fn new() -> State {
        State {
            pointing: common_math::Angles::new(0.0,0.0,0.0),
            angular_rate: common_math::Angles::new(0.0,0.0,0.0),
            position: common_math::Vec3::new(0.0,0.0,0.0),
            velocity: common_math::Vec3::new(0.0,0.0,0.0),
            acceleration: common_math::Vec3::new(0.0,0.0,0.0),
        }
    }

    pub fn log(&self) -> String {
        format!("{{\"position\": {}, \"pointing\": {}, \"angular_rate\": {}, \"velocity\": {}, \"acceleration\": {}}}", self.position.jsonify(), self.pointing.jsonify(), self.angular_rate.jsonify(), self.velocity.jsonify(), self.acceleration.jsonify())
    }
}