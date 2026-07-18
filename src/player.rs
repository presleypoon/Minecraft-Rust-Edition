use macroquad::prelude::*;

const DRAG: Vec3 = vec3(0.91, 0.98, 0.91);
const GRAVITY: Vec3 = vec3(0.0, -0.08, 0.0);

/// `pos` is position
/// `vel` is velocity
pub struct Player {
	pub pos: Vec3,
	pub vel: Vec3,
}
impl Player {
	pub fn new() -> Self {
		Player {
			pos: vec3(0.0, 72.0, 0.0),
			vel: vec3(0.0, 0.0, 0.0),
		}
	}

	pub fn move_player(&mut self, look_angle: Vec2) {
		let accel: Vec3 = Player::find_accel(look_angle);
		self.change_pos();
		self.change_vel(accel);
		println!("{}, {}", self.pos, self.vel);
	}

	fn find_accel(look_angle: Vec2) -> Vec3 {
		let yaw_rad: f32 = look_angle.x.to_radians();
		let mut move_dir: Vec3 = vec3(0.0, 0.0, 0.0);

		if is_key_down(KeyCode::W) {
			move_dir += vec3(yaw_rad.cos(), 0.0, yaw_rad.sin());
		}
		if is_key_down(KeyCode::A) {
			move_dir -= vec3(-yaw_rad.sin(), 0.0, yaw_rad.cos());
		}
		if is_key_down(KeyCode::S) {
			move_dir -= vec3(yaw_rad.cos(), 0.0, yaw_rad.sin());
		}
		if is_key_down(KeyCode::D) {
			move_dir += vec3(-yaw_rad.sin(), 0.0, yaw_rad.cos());
		}
		move_dir.normalize_or_zero() + GRAVITY
	}

	fn change_pos(&mut self) {
		self.pos += self.vel;
	}

	fn change_vel(&mut self, accel: Vec3) {
		self.vel = DRAG * (self.vel + accel);
	}
}
