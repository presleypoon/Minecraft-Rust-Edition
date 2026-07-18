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

	for chunk in world.data.values() {
		for mesh in &chunk.meshes {
			draw_mesh(mesh);
			// gl_use_default_material();
			// unsafe { get_internal_gl().flush() };
		}
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

	for z in 0..16 {
		for y in 0..16 {
			for x in 0..16 {
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

				let block_coord: Vec3 = vec3(x as f32, 15.0 - y as f32, z as f32);

				let p: [Vec3; 8] = [
					chunk_offset + vec3(block_coord.x, block_coord.y, block_coord.z),
					chunk_offset + vec3(block_coord.x + 1.0, block_coord.y, block_coord.z),
					chunk_offset + vec3(block_coord.x + 1.0, block_coord.y + 1.0, block_coord.z),
					chunk_offset + vec3(block_coord.x, block_coord.y + 1.0, block_coord.z),
					chunk_offset + vec3(block_coord.x, block_coord.y, block_coord.z + 1.0),
					chunk_offset + vec3(block_coord.x + 1.0, block_coord.y, block_coord.z + 1.0),
					chunk_offset
						+ vec3(
							block_coord.x + 1.0,
							block_coord.y + 1.0,
							block_coord.z + 1.0,
						),
					chunk_offset + vec3(block_coord.x, block_coord.y + 1.0, block_coord.z + 1.0),
				];

				if y == 0 || data[z][y - 1][x] == Block::Air {
					vertices.push(create_vertex(p[3], colour));
					vertices.push(create_vertex(p[2], colour));
					vertices.push(create_vertex(p[6], colour));
					vertices.push(create_vertex(p[7], colour));

					indices.extend_from_slice(&[
						vertex_counter,
						vertex_counter + 1,
						vertex_counter + 2,
						vertex_counter,
						vertex_counter + 2,
						vertex_counter + 3,
					]);
					vertex_counter += 4;
				}

				if y == 15 || data[z][y + 1][x] == Block::Air {
					vertices.push(create_vertex(p[0], colour));
					vertices.push(create_vertex(p[1], colour));
					vertices.push(create_vertex(p[5], colour));
					vertices.push(create_vertex(p[4], colour));

					indices.extend_from_slice(&[
						vertex_counter,
						vertex_counter + 2,
						vertex_counter + 1,
						vertex_counter,
						vertex_counter + 3,
						vertex_counter + 2,
					]);
					vertex_counter += 4;
				}

				if z == 0 || data[z - 1][y][x] == Block::Air {
					vertices.push(create_vertex(p[0], colour));
					vertices.push(create_vertex(p[1], colour));
					vertices.push(create_vertex(p[2], colour));
					vertices.push(create_vertex(p[3], colour));

					indices.extend_from_slice(&[
						vertex_counter,
						vertex_counter + 2,
						vertex_counter + 1,
						vertex_counter,
						vertex_counter + 3,
						vertex_counter + 2,
					]);
					vertex_counter += 4;
				}

				if z == 15 || data[z + 1][y][x] == Block::Air {
					vertices.push(create_vertex(p[4], colour));
					vertices.push(create_vertex(p[5], colour));
					vertices.push(create_vertex(p[6], colour));
					vertices.push(create_vertex(p[7], colour));

					indices.extend_from_slice(&[
						vertex_counter,
						vertex_counter + 1,
						vertex_counter + 2,
						vertex_counter,
						vertex_counter + 2,
						vertex_counter + 3,
					]);
					vertex_counter += 4;
				}

				if x == 0 || data[z][y][x - 1] == Block::Air {
					vertices.push(create_vertex(p[0], colour));
					vertices.push(create_vertex(p[3], colour));
					vertices.push(create_vertex(p[7], colour));
					vertices.push(create_vertex(p[4], colour));

					indices.extend_from_slice(&[
						vertex_counter,
						vertex_counter + 1,
						vertex_counter + 2,
						vertex_counter,
						vertex_counter + 2,
						vertex_counter + 3,
					]);
					vertex_counter += 4;
				}

				if x == 15 || data[z][y][x + 1] == Block::Air {
					vertices.push(create_vertex(p[1], colour));
					vertices.push(create_vertex(p[2], colour));
					vertices.push(create_vertex(p[6], colour));
					vertices.push(create_vertex(p[5], colour));

					indices.extend_from_slice(&[
						vertex_counter,
						vertex_counter + 2,
						vertex_counter + 1,
						vertex_counter,
						vertex_counter + 3,
						vertex_counter + 2,
					]);
					vertex_counter += 4;
				}
			}
		}
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
