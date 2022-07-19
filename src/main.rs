mod audio;
mod cartridge_reader;
mod display;
mod keypad;
mod processor;
use audio::*;
use cartridge_reader::*;
use display::*;
use keypad::*;
use processor::*;

use std::time::{Duration, Instant};

use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::{EventSettings, Events};
use piston::input::*;
use piston::window::WindowSettings;

use rodio::OutputStream;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const RAM: usize = 4096;
const PROGRAM_START: usize = 0x200;
const SIZE_SCALLER: u32 = 10;
const TIMER_RATE: u64 = 16666; // 60 Hz
const EMULATOR_RATE: u64 = 1851; //540 Hz

fn main() {
    let opengl = OpenGL::V3_2;

    let window_width: u32 = WIDTH as u32 * SIZE_SCALLER;
    let window_height: u32 = HEIGHT as u32 * SIZE_SCALLER;

    let mut window: Window = WindowSettings::new("CHIP8", [window_width, window_height])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut cpu = Cpu::new();
    let cartridge = Cartridge::new("/home/padrition/Downloads/chip8-test-rom-with-audio.ch8");
    let mut game_graphics = GameGraphics::new();
    let mut keypad = Keypad::new();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut audio = Audio::new(440.0, stream_handle);

    cpu.load_rom(cartridge);

    let mut last_tick = Instant::now();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        audio.play(&cpu);

        if let Some(Button::Keyboard(key)) = e.press_args() {
            keypad.presse_key(key);
        }
        if last_tick.elapsed() >= Duration::from_micros(EMULATOR_RATE) {
            cpu.load_key_map(keypad.map_keys());

            cpu.run_next_instruction();

            last_tick = Instant::now();
        }
        if let Some(args) = e.render_args() {
            game_graphics.render(&args, &cpu);
        }
        if let Some(Button::Keyboard(key)) = e.release_args() {
            keypad.release_key(key);
        }
    }
}
