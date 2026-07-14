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

        render(&world);

        next_frame().await;
    }
}

fn render(world: &World) {
    clear_background(Color::from_hex(0x7FCCFFFF));

    set_camera(&Camera3D {
        position: vec3(8.0, 10.0, 25.0),
        target: vec3(8.0, 0.0, 8.0),
        up: vec3(0.0, 1.0, 0.0),
        ..Default::default()
    });

    for ((cx, cz), chunk) in &world.data {
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let draw_pos = vec3(*cx as f32 * 16.0, 0.0, *cz as f32 * 16.0)
                        + vec3(x as f32, y as f32, z as f32);

                    match chunk.data[z][y][x] {
                        Block::Grass => {
                            draw_cube(draw_pos, vec3(1.0, 1.0, 1.0), None, GREEN);
                            draw_cube_wires(draw_pos, vec3(1.0, 1.0, 1.0), BLACK);
                        }
                        Block::Cobblestone => {
                            draw_cube(draw_pos, vec3(1.0, 1.0, 1.0), None, GRAY);
                            draw_cube_wires(draw_pos, vec3(1.0, 1.0, 1.0), BLACK);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
