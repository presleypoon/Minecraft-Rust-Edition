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

	for mesh in world.data.values().flat_map(|chunk| &chunk.meshes) {
		draw_mesh(mesh);
	}
}

fn create_vertex(pos: Vec3, colour: Color) -> Vertex {
	Vertex {
		position: pos,
		uv: Vec2::ZERO,
		normal: Vec4::ZERO,
		color: colour.into(),
	}
}

pub fn build_chunk(data: &[[[Block; 16]; 16]; 16], x: i32, y: i32, z: i32) -> Vec<Mesh> {
	let mut meshes: Vec<Mesh> = Vec::new();

	let mut vertices: Vec<Vertex> = Vec::new();
	let mut indices: Vec<u16> = Vec::new();
	let mut vertex_counter: u16 = 0;

	let chunk_offset = vec3(x as f32 * 16.0, y as f32 * 16.0, z as f32 * 16.0);

	for (z, y, x) in (0..16)
		.flat_map(|z: usize| (0..16).flat_map(move |y: usize| (0..16).map(move |x: usize| (z, y, x))))
	{
		let block = data[z][y][x];
		if block == Block::Air {
			continue;
		}

		if vertex_counter > 9900 || indices.len() > 4900 {
			meshes.push(Mesh {
				vertices: std::mem::take(&mut vertices),
				indices: std::mem::take(&mut indices),
				texture: None,
			});
			vertex_counter = 0;
		}

		let colour: Color = match block {
			Block::Grass => GREEN,
			Block::Cobblestone => GRAY,
			_ => PINK,
		};

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

		build_up(
			(x, y, z),
			p,
			data,
			&mut indices,
			&mut vertices,
			colour,
			&mut vertex_counter,
		);
		build_down(
			(x, y, z),
			p,
			data,
			&mut indices,
			&mut vertices,
			colour,
			&mut vertex_counter,
		);
		build_left(
			(x, y, z),
			p,
			data,
			&mut indices,
			&mut vertices,
			colour,
			&mut vertex_counter,
		);
		build_right(
			(x, y, z),
			p,
			data,
			&mut indices,
			&mut vertices,
			colour,
			&mut vertex_counter,
		);
		build_front(
			(x, y, z),
			p,
			data,
			&mut indices,
			&mut vertices,
			colour,
			&mut vertex_counter,
		);
		build_back(
			(x, y, z),
			p,
			data,
			&mut indices,
			&mut vertices,
			colour,
			&mut vertex_counter,
		);
	}

	if !vertices.is_empty() {
		meshes.push(Mesh {
			vertices,
			indices,
			texture: None,
		});
	}

	meshes
}

fn build_up(
	coord: (usize, usize, usize),
	p: [Vec3; 8],
	data: &[[[Block; 16]; 16]; 16],
	indices: &mut Vec<u16>,
	vertices: &mut Vec<Vertex>,
	colour: Color,
	vertex_counter: &mut u16,
) {
	if coord.1 != 0 && data[coord.2][coord.1 - 1][coord.0] != Block::Air {
		return;
	}
	vertices.push(create_vertex(p[3], colour));
	vertices.push(create_vertex(p[2], colour));
	vertices.push(create_vertex(p[6], colour));
	vertices.push(create_vertex(p[7], colour));

	indices.extend_from_slice(&[
		*vertex_counter,
		*vertex_counter + 1,
		*vertex_counter + 2,
		*vertex_counter,
		*vertex_counter + 2,
		*vertex_counter + 3,
	]);
	*vertex_counter += 4;
}

fn build_down(
	coord: (usize, usize, usize),
	p: [Vec3; 8],
	data: &[[[Block; 16]; 16]; 16],
	indices: &mut Vec<u16>,
	vertices: &mut Vec<Vertex>,
	colour: Color,
	vertex_counter: &mut u16,
) {
	if coord.1 != 15 || data[coord.2][coord.1 + 1][coord.0] != Block::Air {
		return;
	}
	vertices.push(create_vertex(p[0], colour));
	vertices.push(create_vertex(p[1], colour));
	vertices.push(create_vertex(p[5], colour));
	vertices.push(create_vertex(p[4], colour));

	indices.extend_from_slice(&[
		*vertex_counter,
		*vertex_counter + 2,
		*vertex_counter + 1,
		*vertex_counter,
		*vertex_counter + 3,
		*vertex_counter + 2,
	]);
	*vertex_counter += 4;
}

fn build_left(
	coord: (usize, usize, usize),
	p: [Vec3; 8],
	data: &[[[Block; 16]; 16]; 16],
	indices: &mut Vec<u16>,
	vertices: &mut Vec<Vertex>,
	colour: Color,
	vertex_counter: &mut u16,
) {
	if coord.2 != 0 || data[coord.2 - 1][coord.1][coord.0] != Block::Air {
		return;
	}
	vertices.push(create_vertex(p[0], colour));
	vertices.push(create_vertex(p[1], colour));
	vertices.push(create_vertex(p[2], colour));
	vertices.push(create_vertex(p[3], colour));

	indices.extend_from_slice(&[
		*vertex_counter,
		*vertex_counter + 2,
		*vertex_counter + 1,
		*vertex_counter,
		*vertex_counter + 3,
		*vertex_counter + 2,
	]);
	*vertex_counter += 4;
}

fn build_right(
	coord: (usize, usize, usize),
	p: [Vec3; 8],
	data: &[[[Block; 16]; 16]; 16],
	indices: &mut Vec<u16>,
	vertices: &mut Vec<Vertex>,
	colour: Color,
	vertex_counter: &mut u16,
) {
	if coord.2 == 15 || data[coord.2 + 1][coord.1][coord.0] == Block::Air {
		vertices.push(create_vertex(p[4], colour));
		vertices.push(create_vertex(p[5], colour));
		vertices.push(create_vertex(p[6], colour));
		vertices.push(create_vertex(p[7], colour));

		indices.extend_from_slice(&[
			*vertex_counter,
			*vertex_counter + 1,
			*vertex_counter + 2,
			*vertex_counter,
			*vertex_counter + 2,
			*vertex_counter + 3,
		]);
		*vertex_counter += 4;
	}
}

fn build_front(
	coord: (usize, usize, usize),
	p: [Vec3; 8],
	data: &[[[Block; 16]; 16]; 16],
	indices: &mut Vec<u16>,
	vertices: &mut Vec<Vertex>,
	colour: Color,
	vertex_counter: &mut u16,
) {
	if coord.0 != 0 || data[coord.2][coord.1][coord.0 - 1] != Block::Air {
		return;
	}
	vertices.push(create_vertex(p[0], colour));
	vertices.push(create_vertex(p[3], colour));
	vertices.push(create_vertex(p[7], colour));
	vertices.push(create_vertex(p[4], colour));

	indices.extend_from_slice(&[
		*vertex_counter,
		*vertex_counter + 1,
		*vertex_counter + 2,
		*vertex_counter,
		*vertex_counter + 2,
		*vertex_counter + 3,
	]);
	*vertex_counter += 4;
}

fn build_back(
	coord: (usize, usize, usize),
	p: [Vec3; 8],
	data: &[[[Block; 16]; 16]; 16],
	indices: &mut Vec<u16>,
	vertices: &mut Vec<Vertex>,
	colour: Color,
	vertex_counter: &mut u16,
) {
	if coord.0 != 15 || data[coord.2][coord.1][coord.0 + 1] != Block::Air {
		return;
	}
	vertices.push(create_vertex(p[1], colour));
	vertices.push(create_vertex(p[2], colour));
	vertices.push(create_vertex(p[6], colour));
	vertices.push(create_vertex(p[5], colour));

	indices.extend_from_slice(&[
		*vertex_counter,
		*vertex_counter + 2,
		*vertex_counter + 1,
		*vertex_counter,
		*vertex_counter + 3,
		*vertex_counter + 2,
	]);
	*vertex_counter += 4;
}
