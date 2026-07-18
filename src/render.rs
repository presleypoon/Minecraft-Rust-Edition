use crate::Player;
use crate::world::*;
use macroquad::prelude::*;

pub fn render(player: &Player, world: &World, look_angle: Vec2) {
	clear_background(Color::from_hex(0x7FCCFFFF));

	let position: Vec3 = player.pos;

	let yaw_rad: f32 = look_angle.x.to_radians();
	let pitch_rad: f32 = look_angle.y.to_radians();
	let front: Vec3 = vec3(
		yaw_rad.cos() * pitch_rad.cos(),
		pitch_rad.sin(),
		yaw_rad.sin() * pitch_rad.cos(),
	);
	let target: Vec3 = position + front;

	set_camera(&Camera3D {
		position,
		target,
		up: vec3(0.0, 1.0, 0.0),
		..Default::default()
	});

	for ((cx, cy, cz), chunk) in &world.data {
		for i in 0..=4095 {
			let x: usize = i % 16;
			let y: usize = (i / 16) % 16;
			let z: usize = i / 256;

			if chunk.data[z][y][x] == Block::Air || !neighbour_air(chunk, (x, y, z)) {
				continue;
			}

			let draw_pos: Vec3 = vec3(*cx as f32 * 16.0, *cy as f32 * 16.0, *cz as f32 * 16.0)
				+ vec3(x as f32, 15.0 - y as f32, z as f32);

			render_one_block(chunk.data[z][y][x], draw_pos);
		}
	}
}

fn neighbour_air(chunk: &Chunk, pos: (usize, usize, usize)) -> bool {
	pos.0 == 0
		|| pos.0 == 15
		|| pos.1 == 0
		|| pos.1 == 15
		|| pos.2 == 0
		|| pos.2 == 15
		|| is_air(chunk, add_tuples(pos, (0, 0, 1)))
		|| is_air(chunk, sub_tuples(pos, (0, 0, 1)))
		|| is_air(chunk, add_tuples(pos, (0, 1, 0)))
		|| is_air(chunk, sub_tuples(pos, (0, 1, 0)))
		|| is_air(chunk, add_tuples(pos, (1, 0, 0)))
		|| is_air(chunk, sub_tuples(pos, (1, 0, 0)))
}

fn add_tuples(op1: (usize, usize, usize), op2: (usize, usize, usize)) -> (usize, usize, usize) {
	(op1.0 + op2.0, op1.1 + op2.1, op1.2 + op2.2)
}
fn sub_tuples(op1: (usize, usize, usize), op2: (usize, usize, usize)) -> (usize, usize, usize) {
	(op1.0 - op2.0, op1.1 - op2.1, op1.2 - op2.2)
}

fn is_air(chunk: &Chunk, pos: (usize, usize, usize)) -> bool {
	chunk.data[pos.2][pos.1][pos.0] == Block::Air
}

fn render_one_block(block: Block, draw_pos: Vec3) {
	match block {
		Block::Grass => {
			draw_cube(draw_pos, vec3(1.0, 1.0, 1.0), None, GREEN);
			draw_cube_wires(draw_pos, vec3(1.0, 1.0, 1.0), BLACK);
		}
		Block::Cobblestone => {
			draw_cube(draw_pos, vec3(1.0, 1.0, 1.0), None, GRAY);
			// draw_cube_wires(draw_pos, vec3(1.0, 1.0, 1.0), BLACK);
		}
		_ => unreachable!("Invalid Block: {:?}", block),
	}
}
