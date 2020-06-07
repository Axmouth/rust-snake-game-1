extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

use std::collections::LinkedList;
use std::iter::FromIterator;

use rand::prelude::*;

static GRID_X: i32 = 10;
static GRID_Y: i32 = 10;
static BLOCK_SIZE: i32 = 20;

#[derive(Clone, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

pub struct Game {
    gl: GlGraphics, // OpenGL drawing backend.
    snake: Snake,
    apple: Apple,
}

struct Apple {
    loc: (i32, i32),
}

impl Apple {
    fn render(&self, gl: &mut GlGraphics, args: RenderArgs) {
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = graphics::rectangle::square(
            (self.loc.0 * BLOCK_SIZE) as f64,
            (self.loc.1 * BLOCK_SIZE) as f64,
            20_f64,
        );
        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::rectangle(RED, square, transform, gl);
        });
    }
}

struct Snake {
    body: LinkedList<(i32, i32)>,
    direction: Direction,
    _eating_apple: bool,
}
impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: RenderArgs) {
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        let squares: Vec<graphics::types::Rectangle> = self
            .body
            .iter()
            .map(|&(x, y)| {
                let square = graphics::rectangle::square(
                    (x * BLOCK_SIZE) as f64,
                    (y * BLOCK_SIZE) as f64,
                    20_f64,
                );
                return square;
            })
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            squares
                .into_iter()
                .for_each(|square| graphics::rectangle(BLUE, square, transform, gl));
        });
    }

    fn update(&mut self) {
        let mut new_head = (*self.body.front().expect("Snake has no body")).clone();

        match self.direction {
            Direction::Down => new_head.1 += 1,
            Direction::Up => new_head.1 -= 1,
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
        };

        self.body.push_front(new_head);

        if !self._eating_apple {
            self.body.pop_back().unwrap();
        } else {
            self._eating_apple = false;
        }
    }

    fn check_collision(&self, loc: (i32, i32)) -> bool {
        let mut result = false;
        self.body.iter().for_each(|&(x, y)| {
            if x == loc.0 && y == loc.1 {
                result = true;
            }
        });
        return result;
    }

    fn check_head_self_collision(&self) -> bool {
        let loc = self.body.front().unwrap();
        let mut result = false;
        let mut crossed_front = false;
        self.body.iter().for_each(|&(x, y)| {
            if !crossed_front {
                crossed_front = true;
            } else if x == loc.0 && y == loc.1 {
                result = true;
            }
        });
        return result;
    }

    fn check_apple_in_front(&mut self, apple_loc: (i32, i32)) -> bool {
        let mut future_head = self.body.front().unwrap().clone();

        match self.direction {
            Direction::Down => future_head.1 += 1,
            Direction::Up => future_head.1 -= 1,
            Direction::Left => future_head.0 -= 1,
            Direction::Right => future_head.0 += 1,
        };

        if apple_loc.0 == future_head.0 && apple_loc.1 == future_head.1 {
            self._eating_apple = true;
            return true;
        }
        return false;
    }
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);
        });

        self.snake.render(&mut self.gl, *args);
        self.apple.render(&mut self.gl, *args);
    }

    fn update(&mut self, args: &UpdateArgs) {
        let eating_apple = self.snake.check_apple_in_front(self.apple.loc);
        self.snake.update();

        if eating_apple {
            self.reset_apple();
        }

        if self.snake.body.front().unwrap().0 > GRID_Y
            || self.snake.body.front().unwrap().1 > GRID_Y
            || self.snake.body.front().unwrap().0 < 0
            || self.snake.body.front().unwrap().1 < 0
            || self.snake.check_head_self_collision()
        {
            self.reset_game_state();
        }
    }

    fn pressed(&mut self, btn: &Button) {
        let last_dir = self.snake.direction.clone();

        self.snake.direction = match btn {
            &Button::Keyboard(Key::Up) if last_dir != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down) if last_dir != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left) if last_dir != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right) if last_dir != Direction::Left => Direction::Right,
            _ => last_dir,
        };
    }

    fn reset_game_state(&mut self) {
        self.snake = Snake {
            body: LinkedList::from_iter((vec![(0, 0), (0, 1)]).into_iter()),
            direction: Direction::Right,
            _eating_apple: false,
        };
        self.reset_apple();
    }

    fn reset_apple(&mut self) {
        let mut rng = rand::thread_rng();
        self.apple.loc.0 = rng.gen_range(0.0, GRID_X as f64) as i32;
        self.apple.loc.1 = rng.gen_range(0.0, GRID_Y as f64) as i32;
        while self.snake.check_collision(self.apple.loc) {
            self.apple.loc.0 = rng.gen_range(0.0, GRID_X as f64) as i32;
            self.apple.loc.1 = rng.gen_range(0.0, GRID_Y as f64) as i32;
        }
    }
}

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

    let mut rng = rand::thread_rng();

    // Create a new game and run it.
    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            body: LinkedList::from_iter((vec![(0, 0), (0, 1)]).into_iter()),
            direction: Direction::Right,
            _eating_apple: false,
        },
        apple: Apple {
            loc: (
                rng.gen_range(0.0, GRID_X as f64) as i32,
                rng.gen_range(0.0, GRID_Y as f64) as i32,
            ),
        },
    };

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
