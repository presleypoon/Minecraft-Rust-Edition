use macroquad::math::{Vec3, vec3};

pub struct Player {
    pub position: Vec3,
}
impl Player {
    pub fn new() -> Self {
        Player {
            position: vec3(0.0, 72.0, 0.0),
        }
    }
}
