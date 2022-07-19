use crate::processor::Cpu;
use crate::SIZE_SCALLER;
use graphics::*;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::RenderArgs;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const PIXEL_DIMENTION: f64 = SIZE_SCALLER as f64;

pub struct GameGraphics {
    gl: GlGraphics,
}

impl GameGraphics {
    pub fn new() -> GameGraphics {
        GameGraphics {
            gl: GlGraphics::new(OpenGL::V3_2),
        }
    }
    pub fn render(&mut self, args: &RenderArgs, cpu: &Cpu) {
        let pixels = cpu.read_pixels();
        let square = Rectangle::new(WHITE);
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            for (y, row) in pixels.into_iter().enumerate() {
                for (x, pixel) in row.into_iter().enumerate() {
                    let x = x as f64 * SIZE_SCALLER as f64;
                    let y = y as f64 * SIZE_SCALLER as f64;
                    if pixel != 0 {
                        square.draw(
                            [x, y, PIXEL_DIMENTION, PIXEL_DIMENTION],
                            &DrawState::default(),
                            c.transform,
                            gl,
                        );
                    }
                }
            }
        });
    }
}
