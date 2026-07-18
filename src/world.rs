use crate::render::build_chunk;
use macroquad::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Block {
	Air,
	Grass,
	Cobblestone,
}

pub enum ChunkType {
	OnGround,
	BelowGround,
}

pub struct Chunk {
	#[allow(dead_code)]
	pub data: Box<[[[Block; 16]; 16]; 16]>,
	pub meshes: Vec<Mesh>,
}

pub struct World {
	pub data: HashMap<(i32, i32, i32), Chunk>,
}
impl World {
	pub fn new() -> Self {
		World {
			data: HashMap::new(),
		}
	}

	pub fn new_chunk(&mut self, x: i32, y: i32, z: i32, chunk_type: ChunkType) {
		let mut data: Box<[[[Block; 16]; 16]; 16]> = Box::new([[[Block::Air; 16]; 16]; 16]);

		match chunk_type {
			ChunkType::OnGround => {
				for z in &mut data.iter_mut() {
					z[0] = [Block::Grass; 16];

					for y in &mut z[1..16] {
						*y = [Block::Cobblestone; 16];
					}
				}
			}
			ChunkType::BelowGround => data = Box::new([[[Block::Cobblestone; 16]; 16]; 16]),
		}
		
		let meshes: Vec<Mesh> = build_chunk(&data, x, y, z);
		
		self.data.insert((x, y, z), Chunk { data, meshes });
	}
}
