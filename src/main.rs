#![allow(unused)]

use macroquad::{experimental::scene::clear, prelude::*};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

const TPS: f32 = 20.0;

fn window() -> Conf {
    Conf {
        window_title: "Minecraft: Rust Edition".to_string(),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[derive(Clone)]
enum Block {
    Air,
    Grass,
    Cobblestone,
}

struct Chunk {
    data: [[[Block; 16]; 16]; 16],
}
impl Chunk {
    fn write_to_coord(&mut self, coord: Vec3, block: Block) {
        self.data[coord.z as usize][coord.y as usize][coord.x as usize] = block;
    }

    fn read_from_coord(&self, coord: Vec3) -> Block {
        self.data[coord.z as usize][coord.y as usize][coord.x as usize].clone()
    }
}

struct World {
    data: HashMap<(i32, i32), Chunk>,
}
impl World {
    fn new() -> Self {
        World {
            data: HashMap::new(),
        }
    }
}

#[macroquad::main(window)]
async fn main() {
    let tick_rate: Duration = Duration::from_secs_f32(1.0 / TPS);
    let mut last_tick: Instant = Instant::now();
    let mut accumlator: Duration = Duration::ZERO;
    let mut running: bool = false;

    let world: World = World::new();

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        let elapsed: Duration = last_tick.elapsed();
        last_tick = Instant::now();
        accumlator += elapsed;

        if is_any_key_down() {
            running = true;
        }

        if running {
            while accumlator >= tick_rate {
                /* game logic */
                accumlator -= tick_rate;
            }
        } else {
            accumlator = Duration::ZERO;
        }

        render();

        next_frame().await;
    }
}

fn render() {
    clear_background(Color::from_hex(0x7FCCFFFF));
}
