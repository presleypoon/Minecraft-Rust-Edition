use macroquad::prelude::*;

const DRAG: Vec3 = vec3(0.91, 0.98, 0.91);
const ONE_SUB_DRAG: Vec3 = vec3(0.09, 0.02, 0.09);
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
		self.change_pos(accel);
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

	fn change_pos(&mut self, accel: Vec3) {
		self.pos += self.vel - (vec3(1.0, 1.0, 1.0) + DRAG) / pow_vec3_vec3(ONE_SUB_DRAG, accel);
	}

	fn change_vel(&mut self, accel: Vec3) {
		self.vel = DRAG * (self.vel - DRAG * accel / ONE_SUB_DRAG) + DRAG * accel / ONE_SUB_DRAG;
	}
}

fn pow_vec3_vec3(op1: Vec3, op2: Vec3) -> Vec3 {
	vec3(op1.x.powf(op2.x), op1.y.powf(op2.y), op1.z.powf(op2.z))
}
