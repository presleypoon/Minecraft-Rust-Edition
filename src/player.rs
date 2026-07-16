use macroquad::prelude::*;

pub struct Player {
    pub position: Vec3,
    pub speed: Vec3,
}
impl Player {
    pub fn new() -> Self {
        Player {
            position: vec3(0.0, 72.0, 0.0),
            speed: vec3(0.0, 0.0, 0.0),
        }
    }

    pub fn change_speed(&mut self, look_angle: Vec2) {
        if !is_any_key_down() {
            return;
        }

        let yaw_rad = look_angle.x.to_radians();

        if is_key_down(KeyCode::W) {
            self.speed += vec3(yaw_rad.cos(), 0.0, yaw_rad.sin()).normalize_or_zero();
        }
        if is_key_down(KeyCode::A) {
            self.speed -= vec3(yaw_rad.cos(), 0.0, yaw_rad.sin()).normalize_or_zero();
        }
        if is_key_down(KeyCode::S) {
            self.speed += vec3(-yaw_rad.sin(), 0.0, yaw_rad.cos()).normalize_or_zero();
        }
        if is_key_down(KeyCode::D) {
            self.speed += vec3(-yaw_rad.sin(), 0.0, yaw_rad.cos()).normalize_or_zero();
        }
    }
}
