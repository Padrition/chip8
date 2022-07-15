use rand::Rng;

use crate::cartridge_reader::Cartridge;
use super::HEIGHT;
use super::PROGRAM_START;
use super::RAM;
use super::WIDTH;

const TIMER_RATE: u64 = 1667; //60Hz - the speed at wich timers count down

const CHIP8_FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

#[derive(Debug)]
pub struct Cpu {
    memory: [u8; RAM],
    register: [u8; 16],
    program_counter: usize,
    stack: [u16; 16],
    stack_pointer: u8,
    i: u16,
    delay_timer: u8,
    sound_timer: u8,
    pixels: [[u8; WIDTH]; HEIGHT],
    keypad: [bool; 16],
    last_tick: std::time::Instant,
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut ram = [0; RAM];
        ram[..CHIP8_FONT.len()].clone_from_slice(&CHIP8_FONT);
        Cpu {
            memory: ram,
            register: [0; 16],
            program_counter: PROGRAM_START,
            stack: [0; 16],
            stack_pointer: 0,
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            pixels: [[0; WIDTH]; HEIGHT],
            keypad: [false;16],
            last_tick: std::time::Instant::now(),
        }
    }

    pub fn load_rom(&mut self, rom: Cartridge){
        self.memory[PROGRAM_START..PROGRAM_START + rom.rom.len()].clone_from_slice(rom.rom.as_slice());
    }

    pub fn read_pixels(&self) -> [[u8;WIDTH]; HEIGHT]{
        self.pixels
    }

    fn what_key_is_pressed(&self) -> Option<u8>{
        for (key, pressed) in self.keypad.iter().enumerate(){
            if *pressed{
                return Some(key as u8);
            }
        }
        None
    }

    pub fn load_key_map(&mut self, key_map: &[bool;16]){
        for (key, state) in key_map.iter().enumerate(){
            self.keypad[key] = *state;
        }
    }

    fn program_counter_decrease(&mut self){
        self.program_counter -= 2;
    }

    fn program_counter_increase(&mut self) {
        self.program_counter += 2;
    }

    fn next_opcode(&self) -> u16 {
        let op1 = self.memory[self.program_counter];
        let op2 = self.memory[self.program_counter + 1];

        ((op1 as u16) << 8) | op2 as u16
    }

    pub fn run_next_instruction(&mut self) {
        let opcode = self.next_opcode();

        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = (opcode & 0x000F) as u8;

        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;

        match (c, x, y, d) {
            (0x0, 0x0, 0xE, 0x0) => self.screen_clear(),
            (0x0, 0x0, 0xE, 0xE) => self.return_from_subroutine(),
            (0x1, _, _, _) => self.jump_to_subroutine(nnn),
            (0x2, _, _, _) => self.call_subroutine(nnn),
            (0x3, _, _, _) => self.skip_if_x(x, kk),
            (0x4, _, _, _) => self.skip_if_not_x(x, kk),
            (0x5, _, _, 0x0) => self.skip_if_x_eq_y(x, y),
            (0x6, _, _, _) => self.store_to_x(x, kk),
            (0x7, _, _, _) => self.add_to_x(x, kk),
            (0x8, _, _, 0x0) => self.store_y_to_x(x, y),
            (0x8, _, _, 0x1) => self.set_x_xory(x, y),
            (0x8, _, _, 0x2) => self.set_x_xandy(x, y),
            (0x8, _, _, 0x3) => self.set_x_xxory(x, y),
            (0x8, _, _, 0x4) => self.add_y_to_x(x, y),
            (0x8, _, _, 0x5) => self.sub_y_from_x(x, y),
            (0x8, _, _, 0x6) => self.right_shift_x(x),
            (0x8, _, _, 0x7) => self.sub_x_from_y(x, y),
            (0x8, _, _, 0xE) => self.left_shift_x(x),
            (0x9, _, _, 0x0) => self.comparte_x_y(x, y),
            (0xA, _, _, _) => self.store_addres(nnn),
            (0xB, _, _, _) => self.jump_to_addr_and_v0(nnn),
            (0xC, _, _, _) => self.store_rand_to_x(x, kk),
            (0xD, _, _, _) => self.draw_a_sprite(x, y, d),
            (0xE, _, 0x9, 0xE) => self.skip_if_pressed(x),
            (0xE, _, 0xA, 0x1) => self.skip_if_not_pressed(x),
            (0xF, _, 0x0, 0x7) => self.store_delayt_to_x(x),
            (0xF, _, 0x0, 0xA) => self.wait_for_press(x),
            (0xF, _, 0x1, 0x5) => self.set_delayt(x),
            (0xF, _, 0x1, 0x8) => self.set_soundt(x),
            (0xF, _, 0x1, 0xE) => self.add_x_to_i(x),
            (0xF, _, 0x2, 0x9) => self.set_i_to_sprite_addr(x),
            (0xF, _, 0x3, 0x3) => self.bcd_from_x_to_i(x),
            (0xF, _, 0x5, 0x5) => self.store_registers_to_memory(x),
            (0xF, _, 0x6, 0x5) => self.read_memory_to_registers(x),
            _ => println!("Emulator was unable to match the following opcode: {:0x}", opcode),
        }

        self.program_counter_increase();

        if self.last_tick.elapsed() >= std::time::Duration::from_millis(TIMER_RATE){
            if self.delay_timer > 0 { self.delay_timer -= 1};
            if self.sound_timer > 0 { self.sound_timer -= 1};

            self.last_tick = std::time::Instant::now();
        }
    }
    fn read_memory_to_registers(&mut self, x: u8) {
        for j in 0..x {
            self.register[j as usize] = self.memory[(self.i + j as u16) as usize];
        }
    }
    fn store_registers_to_memory(&mut self, x: u8) {
        for i in 0..x {
            self.memory[(self.i + i as u16) as usize] = self.register[i as usize];
        }
    }
    fn bcd_from_x_to_i(&mut self, x: u8) {
        let decimal = self.register[x as usize];
        let i1 = decimal / 100; //maybe there is a better way of converting to bcd, but I came up with this my self. Idk if I shloud be proud or ashamed.
        let i2 = (decimal - i1 * 100) / 10;
        let i3 = (decimal - i1 * 100) - i2 * 10;

        self.memory[self.i as usize] = i1;
        self.memory[(self.i + 1) as usize] = i2;
        self.memory[(self.i + 2) as usize] = i3;
    }
    fn set_i_to_sprite_addr(&mut self, x: u8) {
        let vx = self.register[x as usize];
        let sprite_length = 5;
        self.i = vx as u16 * sprite_length;
    }
    fn add_x_to_i(&mut self, x: u8) {
        self.i += x as u16;
    }
    fn set_soundt(&mut self, x: u8) {
        let vx = self.register[x as usize];
        self.sound_timer = vx;
    }
    fn set_delayt(&mut self, x: u8) {
        let vx = self.register[x as usize];
        self.delay_timer = vx;
    }
    // FX0A:
    // if there is a pressed key, write it's value to the register VX
    // Otherwise decrease program_counter to wait for a key press
    fn wait_for_press(&mut self, x: u8) {
        match self.what_key_is_pressed(){
            Some(key) => self.register[x as usize] = key,
            None => self.program_counter_decrease(),
        }
    }
    fn store_delayt_to_x(&mut self, x: u8) {
        self.register[x as usize] = self.delay_timer;
    }
    fn skip_if_not_pressed(&mut self, x: u8) {
        let key_wanted = self.register[x as usize] as usize;
        if !self.keypad[key_wanted]{
            self.program_counter_increase();
        }
    }
    fn skip_if_pressed(&mut self, x: u8) {
        let key_wanted = self.register[x as usize] as usize;
        if self.keypad[key_wanted]{
            self.program_counter_increase();
        }
    }
    fn draw_a_sprite(&mut self, x: u8, y: u8, n: u8) {
        for sprite_step in 0..n {
            let y = (self.register[y as usize] + sprite_step) % HEIGHT as u8;
            for bit in 0..8 {
                let sprite_bit =
                    (self.memory[(self.i + sprite_step as u16) as usize] >> (7 - bit)) & 1;
                let x = (self.register[x as usize] + bit) % WIDTH as u8;
                let old_pixel = self.pixels[y as usize][x as usize];
                self.pixels[y as usize][x as usize] ^= sprite_bit;
                if old_pixel == 1 {
                    self.register[0xF] = if old_pixel == self.pixels[y as usize][x as usize] {
                        0
                    } else {
                        1
                    };
                }
            }
        }
    }
    fn store_rand_to_x(&mut self, x: u8, kk: u8) {
        let mut rng = rand::thread_rng();
        let mut rand_num: u8 = rng.gen_range(0..255);
        rand_num &= kk;
        self.register[x as usize] = rand_num;
    }
    fn jump_to_addr_and_v0(&mut self, nnn: u16) {
        let v0 = self.register[0] as u16;
        self.jump_to_subroutine(nnn + v0);
    }
    fn store_addres(&mut self, nnn: u16) {
        self.i = nnn;
    }
    fn comparte_x_y(&mut self, x: u8, y: u8) {
        let vx = self.register[x as usize];
        let vy = self.register[y as usize];

        if vx != vy {
            self.program_counter_increase();
        }
    }
    fn left_shift_x(&mut self, x: u8) {
        self.register[0xF] = self.register[x as usize] & 0b0000_0001;
        self.register[x as usize] <<= 1;
    }
    fn sub_x_from_y(&mut self, x: u8, y: u8) {
        let vx = self.register[x as usize];
        let vy = self.register[y as usize];

        self.register[0xF] = if vy > vx { 1 } else { 0 };

        self.register[x as usize] = vy.wrapping_sub(vx);
    }
    fn right_shift_x(&mut self, x: u8) {
        self.register[0xF] = self.register[x as usize] & 0b0000_0001;
        self.register[x as usize] >>= 1;
    }
    fn sub_y_from_x(&mut self, x: u8, y: u8) {
        let vx = self.register[x as usize];
        let vy = self.register[y as usize];

        self.register[0xF] = if vx > vy { 1 } else { 0 };

        self.register[x as usize] = vx.wrapping_sub(vy);
    }
    fn add_y_to_x(&mut self, x: u8, y: u8) {
        let vx = self.register[x as usize];
        let vy = self.register[y as usize];

        let (vx, carry) = vx.overflowing_add(vy);
        self.register[x as usize] = vx;

        self.register[0xF] = if carry { 1 } else { 0 };
    }

    fn set_x_xxory(&mut self, x: u8, y: u8) {
        let vx = self.register[x as usize];
        let vy = self.register[y as usize];

        self.register[x as usize] = vx ^ vy;
    }

    fn set_x_xandy(&mut self, x: u8, y: u8) {
        let vx = self.register[x as usize];
        let vy = self.register[y as usize];

        self.register[x as usize] = vx & vy;
    }

    fn set_x_xory(&mut self, x: u8, y: u8) {
        let vx = self.register[x as usize];
        let vy = self.register[y as usize];

        self.register[x as usize] = vx | vy;
    }

    fn store_y_to_x(&mut self, x: u8, y: u8) {
        self.register[x as usize] = self.register[y as usize];
    }

    fn add_to_x(&mut self, x: u8, kk: u8) {
        let vx = self.register[x as usize];
        let (vx,_) = vx.overflowing_add(kk);
        self.register[x as usize] = vx;
    }

    fn store_to_x(&mut self, x: u8, kk: u8) {
        self.register[x as usize] = kk;
    }

    fn skip_if_x_eq_y(&mut self, x: u8, y: u8) {
        if self.register[x as usize] == self.register[y as usize] {
            self.program_counter_increase();
        }
    }

    fn skip_if_not_x(&mut self, x: u8, kk: u8) {
        if self.register[x as usize] as u8 != kk {
            self.program_counter_increase();
        }
    }

    fn skip_if_x(&mut self, x: u8, kk: u8) {
        if self.register[x as usize] as u8 == kk {
            self.program_counter_increase();
        }
    }

    fn return_from_subroutine(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow!")
        }
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer as usize] as usize;
    }

    fn call_subroutine(&mut self, nnn: u16) {
        if self.stack_pointer as usize > self.stack.len() {
            panic!("Stack overflow!")
        }

        self.stack[self.stack_pointer as usize] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.jump_to_subroutine(nnn);
    }

    fn screen_clear(&mut self) {
        self.pixels = [[0;WIDTH];HEIGHT];
    }

    fn jump_to_subroutine(&mut self, nnn: u16) {
        self.program_counter = nnn as usize;
    }
}

#[cfg(test)]
#[path = "./processor_test.rs"]
mod test;
