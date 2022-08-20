use crate::cartridge_reader::*;
use crate::processor::Cpu;
use crate::SIZE_SCALLER;
use graphics::*;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL};
use piston::input::RenderArgs;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const PIXEL_DIMENTION: f64 = SIZE_SCALLER as f64;
const FONT_SIZE: u32 = 32;
const FONT_SIZE_SMALL: u32 = 20;
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

    pub fn draw_ui(&mut self, args: &RenderArgs, glyph: &mut GlyphCache, cartridge: &Cartridge) {
        if self.draw {
            let title_x = 144.0;
            let title_y = 50.0;
            let rec_width = 50.0;
            let rec_len = 320.0;
            let rec_y = 130.0;
            let rec_x =  160.0;
            let rec_with_border = Rectangle::new_round_border(WHITE, 10.0, 1.0);
            let left_arrow_x = 96.0;
            let left_arrow_y = 180.0;
            let right_arrow_x = 512.0;
            let right_arrow_y = left_arrow_y;
            let rom_name = cartridge.get_game_name();
            let half_name = (rom_name.chars().count() as f64 / 2.0) * FONT_SIZE_SMALL as f64;
            let rom_x = 320.0 - half_name;
            let rom_y = left_arrow_y;
            self.gl.draw(args.viewport(), |c, gl| {
                clear(BLACK, gl);
                text(
                    WHITE,
                    FONT_SIZE,
                    CHOOSE_GAME,
                    glyph,
                    c.transform.trans(title_x, title_y),
                    gl,
                )
                .unwrap();
                rec_with_border.draw(
                    [rec_x, rec_y, rec_len, rec_width],
                    &DrawState::default(),
                    c.transform,
                    gl,
                );
                text(
                    WHITE,
                    FONT_SIZE_SMALL,
                    rom_name,
                    glyph,
                    c.transform.trans(rom_x, rom_y),
                    gl,
                )
                .unwrap();
                text(
                    WHITE,
                    FONT_SIZE,
                    "<",
                    glyph,
                    c.transform.trans(left_arrow_x, left_arrow_y),
                    gl,
                )
                .unwrap();
                text(
                    WHITE,
                    FONT_SIZE,
                    ">",
                    glyph,
                    c.transform.trans(right_arrow_x, right_arrow_y),
                    gl,
                )
                .unwrap();
            });
        }
    }
}
