mod player;
mod render;
mod world;

use player::*;
use render::*;
use world::*;

use enigo::{Coordinate, Enigo, Mouse, Settings};
use macroquad::prelude::*;
use std::time::{Duration, Instant};

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

	let mut player: Player = Player::new();
	let mut world: World = World::new();
	let mut look_angle: Vec2 = Vec2::ZERO;
	
	// for z in -2..2 {
	// 	for y in 0..4 {
	// 		for x in -2..2 {
	for z in -8..8 {
		for y in 0..4 {
			for x in -8..8 {
				world.new_chunk(
					x,
					y,
					z,
					if y == 3 {
						ChunkType::OnGround
					} else {
						ChunkType::BelowGround
					},
				);
			}
		}
	}

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
				game_tick(&mut player, look_angle);
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

fn game_tick(player: &mut Player, look_angle: Vec2) {
	player.move_player(look_angle);
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
