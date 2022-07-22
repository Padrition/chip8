use crate::processor::Cpu;
use crate::SIZE_SCALLER;
use crate::WIDTH;
use crate::HEIGHT;
use crate::emulator::*;
use graphics::*;
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache};
use piston::input::RenderArgs;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const PIXEL_DIMENTION: f64 = SIZE_SCALLER as f64;
const FONT_SIZE: u32 = 32;
const HEADER: &str = "CHIP-8 EMULATOR";
const LOAD_ROM: &str = "LOAD GAME";
const QUIT: &str = "QUIT";

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

    pub fn draw_ui(&mut self, args: &RenderArgs, glyph: &mut GlyphCache, emulator: &Emulator){
        let x_header = self.center_text_x(HEADER);
        let y_header = self.text_y(1);
        let x_load = self.center_text_x(LOAD_ROM);
        let y_load = self.text_y(2);
        let x_quit = self.center_text_x(QUIT);
        let y_quit = self.text_y(3);
        let rec_with_border = Rectangle::new_round_border(WHITE, 10.0, 1.0);
        let rect_dimensions = self.rect_dimensions();
        self.gl.draw(args.viewport(), |c, gl|{
            clear(BLACK, gl);
            rec_with_border.draw(rect_dimensions, &DrawState::default(), c.transform, gl);
            text(WHITE, FONT_SIZE, HEADER, glyph,c.transform.trans(x_header, y_header) , gl).unwrap();
            text(WHITE, FONT_SIZE, LOAD_ROM, glyph ,c.transform.trans(x_load, y_load) , gl).unwrap();
            text(WHITE, FONT_SIZE, QUIT, glyph,c.transform.trans(x_quit, y_quit) , gl).unwrap();
        });
        match emulator.emulator_choice{
            EmulatorChoice::LoadRom => {},
            EmulatorChoice::Quit => {},
        }
    }

    fn rect_dimensions(&self) -> [f64;4]{
        let rec_len = ((HEADER.len() * FONT_SIZE as usize) + 2 * SIZE_SCALLER as usize) as f64; 
        let rec_height = (FONT_SIZE + 2 * SIZE_SCALLER) as f64;
        let rec_y = self.text_y(1) - (FONT_SIZE +  SIZE_SCALLER) as f64;
        let rec_x = self.center_text_x(HEADER) - 2.0 * SIZE_SCALLER as f64;
        [rec_x, rec_y, rec_len, rec_height]
    }

    fn center_text_x(&self, text: &str) -> math::Scalar{
        let screen_width = WIDTH * SIZE_SCALLER as usize;
        let text_len = text.len() * FONT_SIZE as usize;
        let x_pos = (screen_width as f64 / 2.0) - (text_len as f64 / 2.0);
        x_pos
    }
    
    fn text_y(&self, n: usize) -> math::Scalar{
        ((HEIGHT * SIZE_SCALLER as usize) / 3 * n) as f64
    }

}
