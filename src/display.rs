use crate::processor::Cpu;
use crate::SIZE_SCALLER;
use crate::WIDTH;
use crate::HEIGHT;
use crate::emulator::*;
use crate::cartridge_reader::*;
use graphics::*;
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache};
use piston::input::RenderArgs;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const PIXEL_DIMENTION: f64 = SIZE_SCALLER as f64;
const FONT_SIZE: u32 = 32;
const CHOOSE_GAME: &str = "CHOOSE GAME";

pub struct GameGraphics {
    gl: GlGraphics,
    pub draw: bool,
}

impl GameGraphics {
    pub fn new() -> GameGraphics {
        GameGraphics {
            gl: GlGraphics::new(OpenGL::V3_2),
            draw: true,
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

    pub fn draw_ui(&mut self, args: &RenderArgs, glyph: &mut GlyphCache, cartridge: &Cartridge){
        if self.draw{
            let x_choose = self.center_text_x(CHOOSE_GAME);
            let y_choose = self.text_y(1);
            let rec_len = self.text_len_in_pixels(CHOOSE_GAME) + 2.0 * SIZE_SCALLER as f64;
            let rec_x = self.center_text_x(CHOOSE_GAME) - 2.0 * SIZE_SCALLER as f64;
            let rec_y = self.text_y(2) - (FONT_SIZE + SIZE_SCALLER) as f64;
            let rec_with_border = Rectangle::new_round_border(WHITE, 10.0, 1.0);
            let rom_name = cartridge.get_game_name();
            let rom_x = self.center_text_x(rom_name);
            let rom_y = self.text_y(2);
            let left_arrow_x = rec_x - FONT_SIZE as f64; 
            let right_arrow_x = rec_x + rec_len;
            self.gl.draw(args.viewport(), |c, gl|{
                clear(BLACK, gl);
                text(WHITE, FONT_SIZE, CHOOSE_GAME, glyph, c.transform.trans(x_choose, y_choose), gl).unwrap();
                rec_with_border.draw([rec_x,rec_y,rec_len, 50.0], &DrawState::default(), c.transform, gl);
                text(WHITE, FONT_SIZE, rom_name, glyph, c.transform.trans(rom_x, rom_y), gl).unwrap();
                text(WHITE, FONT_SIZE, "<", glyph, c.transform.trans(left_arrow_x, rom_y), gl).unwrap();
                text(WHITE, FONT_SIZE, ">", glyph, c.transform.trans(right_arrow_x, rom_y), gl).unwrap();
            });
        }
    }

    fn center_text_x(&self, text: &str) -> math::Scalar{
        let screen_width = WIDTH * SIZE_SCALLER as usize;
        let text_len = self.text_len_in_pixels(text);
        let x_pos = (screen_width as f64 / 2.0) - (text_len as f64 / 2.0);
        x_pos
    }

    fn text_len_in_pixels(&self, text: &str) -> f64{
        text.len() as f64 * FONT_SIZE as f64
    }
    
    fn text_y(&self, n: usize) -> math::Scalar{
        ((HEIGHT * SIZE_SCALLER as usize) / 3 * n) as f64
    }

}
