extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use opengl_graphics::GlGraphics;
use piston::input::*;

use super::constants::*;
use super::direction::*;

use std::collections::LinkedList;

pub struct Snake {
    pub body: LinkedList<(i32, i32)>,
    pub direction: Direction,
    pub _eating_apple: bool,
}

impl Snake {
    pub fn render(&self, gl: &mut GlGraphics, args: RenderArgs) {
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

    pub fn update(&mut self) {
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

    pub fn check_collision(&self, loc: (i32, i32)) -> bool {
        let mut result = false;
        self.body.iter().for_each(|&(x, y)| {
            if x == loc.0 && y == loc.1 {
                result = true;
            }
        });
        return result;
    }

    pub fn check_head_self_collision(&self) -> bool {
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

    pub fn check_apple_in_front(&mut self, apple_loc: (i32, i32)) -> bool {
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
