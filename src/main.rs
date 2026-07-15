mod player;
mod world;

use player::*;
use world::*;

use enigo::{Coordinate, Enigo, Mouse, Settings};
use macroquad::prelude::*;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

const TPS: f32 = 20.0;
static mut WINDOW_WIDTH: f32 = -1.0;
static mut WINDOW_HEIGHT: f32 = -1.0;
static mut CENTRE_X: f32 = -1.0;
static mut CENTRE_Y: f32 = -1.0;

// macro_rules! elapsed {
//     ($name:expr, $block:block) => {
//         let start = std::time::Instant::now();
//         $block;
//         let duration = start.elapsed();
//         println!("{} took {:?}", $name, duration);
//     };
// }

fn window() -> Conf {
    Conf {
        window_title: "Minecraft: Rust Edition".to_string(),
        fullscreen: true,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window)]
async fn main() {
    let tick_rate: Duration = Duration::from_secs_f32(1.0 / TPS);
    let mut last_tick: Instant = Instant::now();
    let mut accumlator: Duration = Duration::ZERO;
    let mut running: bool = false;
    let mut enigo: Enigo = Enigo::new(&Settings::default()).unwrap();

    let player: Player = Player::new();
    let mut world: World = World::new();
    let mut look_angle: Vec2 = Vec2::ZERO;

    world.new_chunk(0, 0, 0);
    set_cursor_grab(true);
    show_mouse(false);

    unsafe {
        let (window_width, window_height) = enigo.main_display().unwrap_or((1920, 1080));
        WINDOW_WIDTH = window_width as f32;
        WINDOW_HEIGHT = window_height as f32;

        CENTRE_X = WINDOW_WIDTH / 2.0;
        CENTRE_Y = WINDOW_HEIGHT / 2.0;
    }

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

        camera_move(&mut look_angle, &mut enigo);
        render(&player, &world, look_angle);

        next_frame().await;
    }
}

fn camera_move(look_angle: &mut Vec2, enigo: &mut Enigo) {
    let mouse_loc: (i32, i32) = enigo.location().unwrap();
    let mouse_pos: Vec2 = vec2(mouse_loc.0 as f32, mouse_loc.1 as f32);
    let mut mouse_rel_pos: Vec2 = mouse_pos - vec2(unsafe { CENTRE_X }, unsafe { CENTRE_Y });
    mouse_rel_pos.y *= -1.0;
    mouse_rel_pos *= vec2(0.25, 0.25);
    *look_angle += mouse_rel_pos;
    look_angle.x = (look_angle.x + 180.0) % 360.0 - 180.0;
    look_angle.y = look_angle.y.clamp(-90.0, 90.0);

    enigo
        .move_mouse(
            unsafe { CENTRE_X } as i32,
            unsafe { CENTRE_Y } as i32,
            Coordinate::Abs,
        )
        .ok();
}

fn render(player: &Player, world: &World, look_angle: Vec2) {
    clear_background(Color::from_hex(0x7FCCFFFF));

    let position: Vec3 = player.position;

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
        for i in 0..4095 {
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
