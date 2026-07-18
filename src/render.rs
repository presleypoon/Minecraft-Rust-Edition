use crate::Player;
use crate::world::*;
use macroquad::prelude::*;

pub struct Texture {
	pub grass_top: Texture2D,
	pub grass_side: Texture2D,
	pub dirt: Texture2D,
	pub cobblestone: Texture2D,
}
impl Texture {
	pub async fn new() -> Self {
		let textures = [
			load_texture("assets/textures/blocks/grass_top.png")
				.await
				.unwrap(),
			load_texture("assets/textures/blocks/grass_side.png")
				.await
				.unwrap(),
			load_texture("assets/textures/blocks/dirt.png")
				.await
				.unwrap(),
			load_texture("assets/textures/blocks/cobblestone.png")
				.await
				.unwrap(),
		];

		// for texture in textures.iter() {
		// 	texture.set_filter(FilterMode::Nearest);
		// }

		Texture {
			grass_top: textures[0].clone(),
			grass_side: textures[1].clone(),
			dirt: textures[2].clone(),
			cobblestone: textures[3].clone(),
		}
	}
}

pub fn render(player: &Player, world: &mut World, look_angle: Vec2) {
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

	for chunk in world.data.values_mut() {
		for mesh in chunk.meshes.iter_mut() {
			draw_mesh(mesh);
		}
	}
}

fn create_vertex(pos: Vec3, uv: Vec2) -> Vertex {
	Vertex {
		position: pos,
		uv,
		normal: Vec4::ZERO,
		color: WHITE.into(),
	}
}

pub fn build_chunk(
	data: &[[[Block; 16]; 16]; 16],
	x: i32,
	y: i32,
	z: i32,
	textures: &Texture,
) -> Vec<Mesh> {
	let mut meshes: Vec<Mesh> = Vec::new();

	let mut vertices: [Vec<Vertex>; 4] = [const { Vec::new() }; 4];
	let mut indices: [Vec<u16>; 4] = [const { Vec::new() }; 4];
	let mut counter: [u16; 4] = [0; 4];

	let chunk_offset = vec3(x as f32 * 16.0, y as f32 * 16.0, z as f32 * 16.0);

	for (z, y, x) in (0..16)
		.flat_map(|z: usize| (0..16).flat_map(move |y: usize| (0..16).map(move |x: usize| (z, y, x))))
	{
		let block = data[z][y][x];
		if block == Block::Air {
			continue;
		}

		for i in 0..4 {
			if counter[i] > 9900 || indices[i].len() > 4900 {
				meshes.push(Mesh {
					vertices: std::mem::take(&mut vertices[i]),
					indices: std::mem::take(&mut indices[i]),
					texture: None,
				});
				counter[i] = 0;
			}
		}

		let bx: f32 = x as f32;
		let by: f32 = 15.0 - y as f32;
		let bz: f32 = z as f32;

		let p: [Vec3; 8] = [
			chunk_offset + vec3(bx, by, bz),
			chunk_offset + vec3(bx + 1.0, by, bz),
			chunk_offset + vec3(bx + 1.0, by + 1.0, bz),
			chunk_offset + vec3(bx, by + 1.0, bz),
			chunk_offset + vec3(bx, by, bz + 1.0),
			chunk_offset + vec3(bx + 1.0, by, bz + 1.0),
			chunk_offset + vec3(bx + 1.0, by + 1.0, bz + 1.0),
			chunk_offset + vec3(bx, by + 1.0, bz + 1.0),
		];

		let i_top: usize = match block {
			Block::Grass => 0,
			Block::Cobblestone => 3,
			_ => unreachable!(),
		};
		build_up(
			(x, y, z),
			p,
			data,
			&mut indices[i_top],
			&mut vertices[i_top],
			&mut counter[i_top],
		);

		let i_bottom: usize = match block {
			Block::Grass => 2,
			Block::Cobblestone => 3,
			_ => unreachable!(),
		};
		build_down(
			(x, y, z),
			p,
			data,
			&mut indices[i_bottom],
			&mut vertices[i_bottom],
			&mut counter[i_bottom],
		);

		let i_left: usize = match block {
			Block::Grass => 1,
			Block::Cobblestone => 3,
			_ => unreachable!(),
		};
		build_left(
			(x, y, z),
			p,
			data,
			&mut indices[i_left],
			&mut vertices[i_left],
			&mut counter[i_left],
		);

		let i_right: usize = match block {
			Block::Grass => 1,
			Block::Cobblestone => 3,
			_ => unreachable!(),
		};
		build_right(
			(x, y, z),
			p,
			data,
			&mut indices[i_right],
			&mut vertices[i_right],
			&mut counter[i_right],
		);

		let i_front: usize = match block {
			Block::Grass => 1,
			Block::Cobblestone => 3,
			_ => unreachable!(),
		};
		build_front(
			(x, y, z),
			p,
			data,
			&mut indices[i_front],
			&mut vertices[i_front],
			&mut counter[i_front],
		);

		let i_back: usize = match block {
			Block::Grass => 1,
			Block::Cobblestone => 3,
			_ => unreachable!(),
		};
		build_back(
			(x, y, z),
			p,
			data,
			&mut indices[i_back],
			&mut vertices[i_back],
			&mut counter[i_back],
		);
	}

	for i in 0..4 {
		if !vertices[i].is_empty() {
			let tex_handle = match i {
				0 => Some(textures.grass_top.clone()),
				1 => Some(textures.grass_side.clone()),
				2 => Some(textures.dirt.clone()),
				3 => Some(textures.cobblestone.clone()),
				_ => None,
			};

			meshes.push(Mesh {
				vertices: std::mem::take(&mut vertices[i]),
				indices: std::mem::take(&mut indices[i]),
				texture: tex_handle,
			});
		}
	}

	meshes
}

fn build_up(
	coord: (usize, usize, usize),
	p: [Vec3; 8],
	data: &[[[Block; 16]; 16]; 16],
	indices: &mut Vec<u16>,
	vertices: &mut Vec<Vertex>,
	counter: &mut u16,
) {
	if coord.1 != 0 && data[coord.2][coord.1 - 1][coord.0] != Block::Air {
		return;
	}
	vertices.push(create_vertex(p[3], vec2(0.0, 0.0)));
	vertices.push(create_vertex(p[2], vec2(1.0, 0.0)));
	vertices.push(create_vertex(p[6], vec2(1.0, 1.0)));
	vertices.push(create_vertex(p[7], vec2(0.0, 1.0)));

	indices.extend_from_slice(&[
		*counter,
		*counter + 1,
		*counter + 2,
		*counter,
		*counter + 2,
		*counter + 3,
	]);
	*counter += 4;
}

fn build_down(
	coord: (usize, usize, usize),
	p: [Vec3; 8],
	data: &[[[Block; 16]; 16]; 16],
	indices: &mut Vec<u16>,
	vertices: &mut Vec<Vertex>,
	counter: &mut u16,
) {
	if coord.1 != 15 && data[coord.2][coord.1 + 1][coord.0] != Block::Air {
		return;
	}
	vertices.push(create_vertex(p[0], vec2(0.0, 0.0)));
	vertices.push(create_vertex(p[1], vec2(1.0, 0.0)));
	vertices.push(create_vertex(p[5], vec2(1.0, 1.0)));
	vertices.push(create_vertex(p[4], vec2(0.0, 1.0)));

	indices.extend_from_slice(&[
		*counter,
		*counter + 2,
		*counter + 1,
		*counter,
		*counter + 3,
		*counter + 2,
	]);
	*counter += 4;
}

fn build_left(
	coord: (usize, usize, usize),
	p: [Vec3; 8],
	data: &[[[Block; 16]; 16]; 16],
	indices: &mut Vec<u16>,
	vertices: &mut Vec<Vertex>,
	counter: &mut u16,
) {
	if coord.2 != 0 && data[coord.2 - 1][coord.1][coord.0] != Block::Air {
		return;
	}
	vertices.push(create_vertex(p[0], vec2(0.0, 0.0)));
	vertices.push(create_vertex(p[1], vec2(1.0, 0.0)));
	vertices.push(create_vertex(p[2], vec2(1.0, 1.0)));
	vertices.push(create_vertex(p[3], vec2(0.0, 1.0)));

	indices.extend_from_slice(&[
		*counter,
		*counter + 2,
		*counter + 1,
		*counter,
		*counter + 3,
		*counter + 2,
	]);
	*counter += 4;
}

fn build_right(
	coord: (usize, usize, usize),
	p: [Vec3; 8],
	data: &[[[Block; 16]; 16]; 16],
	indices: &mut Vec<u16>,
	vertices: &mut Vec<Vertex>,
	counter: &mut u16,
) {
	if coord.2 != 15 && data[coord.2 + 1][coord.1][coord.0] != Block::Air {
		return;
	}
	vertices.push(create_vertex(p[4], vec2(0.0, 0.0)));
	vertices.push(create_vertex(p[5], vec2(1.0, 0.0)));
	vertices.push(create_vertex(p[6], vec2(1.0, 1.0)));
	vertices.push(create_vertex(p[7], vec2(0.0, 1.0)));

	indices.extend_from_slice(&[
		*counter,
		*counter + 1,
		*counter + 2,
		*counter,
		*counter + 2,
		*counter + 3,
	]);
	*counter += 4;
}

fn build_front(
	coord: (usize, usize, usize),
	p: [Vec3; 8],
	data: &[[[Block; 16]; 16]; 16],
	indices: &mut Vec<u16>,
	vertices: &mut Vec<Vertex>,
	counter: &mut u16,
) {
	if coord.0 != 0 && data[coord.2][coord.1][coord.0 - 1] != Block::Air {
		return;
	}
	vertices.push(create_vertex(p[0], vec2(0.0, 0.0)));
	vertices.push(create_vertex(p[3], vec2(1.0, 0.0)));
	vertices.push(create_vertex(p[7], vec2(1.0, 1.0)));
	vertices.push(create_vertex(p[4], vec2(0.0, 1.0)));

	indices.extend_from_slice(&[
		*counter,
		*counter + 1,
		*counter + 2,
		*counter,
		*counter + 2,
		*counter + 3,
	]);
	*counter += 4;
}

fn build_back(
	coord: (usize, usize, usize),
	p: [Vec3; 8],
	data: &[[[Block; 16]; 16]; 16],
	indices: &mut Vec<u16>,
	vertices: &mut Vec<Vertex>,
	counter: &mut u16,
) {
	if coord.0 != 15 && data[coord.2][coord.1][coord.0 + 1] != Block::Air {
		return;
	}
	vertices.push(create_vertex(p[1], vec2(0.0, 0.0)));
	vertices.push(create_vertex(p[2], vec2(1.0, 0.0)));
	vertices.push(create_vertex(p[6], vec2(1.0, 1.0)));
	vertices.push(create_vertex(p[5], vec2(0.0, 1.0)));

	indices.extend_from_slice(&[
		*counter,
		*counter + 2,
		*counter + 1,
		*counter,
		*counter + 3,
		*counter + 2,
	]);
	*counter += 4;
}
