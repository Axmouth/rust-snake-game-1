extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use opengl_graphics::GlGraphics;
use piston::input::*;

use super::constants::*;

pub struct Apple {
    pub loc: (i32, i32),
}

impl Apple {
    pub fn render(&self, gl: &mut GlGraphics, args: RenderArgs) {
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
