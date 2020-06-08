extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

mod snake_game_lib;
use snake_game_lib::*;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
        "Snake Game",
        [(BLOCK_SIZE * GRID_X) as f64, (BLOCK_SIZE * GRID_Y) as f64],
    )
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();
    // Create a new game and run it.
    let mut game = SnakeGame::new(opengl);

    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }
        if let Some(args) = e.update_args() {
            game.update(&args);
        }
        if let Some(args) = e.button_args() {
            if args.state == ButtonState::Press {
                game.pressed(&args.button);
            }
        }
    }
}
