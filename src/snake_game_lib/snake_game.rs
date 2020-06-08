extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::*;
use rand::*;
use std::collections::LinkedList;
use std::iter::FromIterator;

use super::apple::*;
use super::constants::*;
use super::direction::*;
use super::snake::*;

pub struct SnakeGame {
    gl: GlGraphics, // OpenGL drawing backend.
    snake: Snake,
    apple: Apple,
    direction_pressed: bool,
}

impl SnakeGame {
    pub fn new(opengl: OpenGL) -> Self {
        let mut rng = rand::thread_rng();
        let game = SnakeGame {
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
            direction_pressed: false,
        };

        return game;
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            // Clear the screen.
            clear(GREEN, gl);
        });

        self.snake.render(&mut self.gl, *args);
        self.apple.render(&mut self.gl, *args);
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
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
        self.direction_pressed = false;
    }

    pub fn pressed(&mut self, btn: &Button) {
        if self.direction_pressed {
            return;
        }
        self.direction_pressed = true;
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
