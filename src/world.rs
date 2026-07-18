use crate::render::{Texture, build_chunk};
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

	pub fn new_chunk(&mut self, x: i32, y: i32, z: i32, textures: &Texture, chunk_type: ChunkType) {
		let data: Box<[[[Block; 16]; 16]; 16]> = match chunk_type {
			ChunkType::OnGround => Box::new(gen_on_ground()),
			ChunkType::BelowGround => Box::new([[[Block::Cobblestone; 16]; 16]; 16]),
		};

		let meshes: Vec<Mesh> = build_chunk(&data, x, y, z, textures);

		self.data.insert((x, y, z), Chunk { data, meshes });
	}
}

fn gen_on_ground() -> [[[Block; 16]; 16]; 16] {
	let mut data = [[[Block::Cobblestone; 16]; 16]; 16];
	for data_z in &mut data {
		data_z[0] = [Block::Grass; 16];
	}
	data
}
