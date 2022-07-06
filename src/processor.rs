use rand::Rng;

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
    memory: [u8; 4096],
    register: [u8; 16],
    program_counter: usize,
    stack: [u16; 16],
    stack_pointer: u8,
    i: u16,
    delay_timer: u8,
    sound_timer: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut ram = [0; 4096];
        ram[..CHIP8_FONT.len()].clone_from_slice(&CHIP8_FONT);
        Cpu {
            memory: ram,
            register: [0; 16],
            program_counter: 0x200,
            stack: [0; 16],
            stack_pointer: 0,
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    fn program_counter_increase(&mut self) {
        self.program_counter += 2;
    }

    fn next_opcode(&self) -> u16 {
        let op1 = self.memory[self.program_counter];
        let op2 = self.memory[self.program_counter + 1];

        ((op1 as u16) << 8) | op2 as u16
    }

    pub fn run(&mut self) {
        loop {
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
                _ => todo!("TODO: {:0x}", opcode),
            }

            self.program_counter_increase();
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
    fn wait_for_press(&mut self, x: u8) {
        todo!("Wait for press");
    }
    fn store_delayt_to_x(&mut self, x: u8) {
        self.register[x as usize] = self.delay_timer;
    }
    fn skip_if_not_pressed(&mut self, x: u8) {
        todo!("SKIP not");
    }
    fn skip_if_pressed(&mut self, x: u8) {
        todo!("SKIP");
    }
    fn draw_a_sprite(&mut self, x: u8, y: u8, n: u8) {
        todo!("DRAW!");
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
        self.register[x as usize] += kk;
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
        todo!("clear screen");
    }

    fn jump_to_subroutine(&mut self, nnn: u16) {
        self.program_counter = nnn as usize;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn read_memory_to_registers_test() {
        let mut cpu = Cpu::new();
        cpu.i = 0x300;
        for j in 0..cpu.register.len() {
            cpu.memory[0x300 + j] = j as u8;
        }
        cpu.read_memory_to_registers(16);
        for j in 0..cpu.register.len() {
            assert_eq!(cpu.register[j], j as u8);
        }
    }
    #[test]
    fn store_registers_to_memory_test() {
        let mut cpu = Cpu::new();
        cpu.i = 0x300;
        for j in 0..cpu.register.len() {
            cpu.register[j] = j as u8;
        }
        cpu.store_registers_to_memory(16);
        for j in 0..cpu.register.len() {
            assert_eq!(cpu.register[j], cpu.memory[0x300 + j]);
        }
    }
    #[test]
    fn bcd_from_x_to_i_test() {
        let mut cpu = Cpu::new();
        cpu.i = 0x300;
        cpu.register[0] = 159;
        cpu.bcd_from_x_to_i(0);
        assert_eq!(cpu.memory[0x300], 1);
        assert_eq!(cpu.memory[0x301], 5);
        assert_eq!(cpu.memory[0x302], 9);
    }
    #[test]
    fn set_i_to_sprite_addr_test() {
        let mut cpu = Cpu::new();
        cpu.register[0] = 0xD;
        cpu.set_i_to_sprite_addr(0);
        assert_eq!(cpu.i, 65);
    }
    #[test]
    fn store_rand_to_x_test() {
        let mut cpu = Cpu::new();
        for i in 0..255 {
            cpu.store_rand_to_x(0, i);
            let range = 0..255;
            assert!(range.contains(&cpu.register[0]));
        }
    }
    #[test]
    fn left_shift_x_test() {
        let mut cpu = Cpu::new();
        cpu.register[0] = 5;
        cpu.left_shift_x(0);
        assert_eq!(cpu.register[0xF], 1);
        assert_eq!(cpu.register[0], 10);
    }
    #[test]
    fn sub_x_from_y_test() {
        let mut cpu = Cpu::new();
        cpu.register[0] = 5;
        cpu.register[1] = 7;
        cpu.sub_x_from_y(0, 1);
        assert_eq!(cpu.register[0], 2);
        assert_eq!(cpu.register[0xF], 1);

        let mut cpu = Cpu::new();
        cpu.register[0] = 5;
        cpu.register[1] = 4;
        cpu.sub_x_from_y(0, 1);
        assert_eq!(cpu.register[0], 255);
        assert_eq!(cpu.register[0xF], 0);
    }
    #[test]
    fn right_shift_x_test() {
        let mut cpu = Cpu::new();
        cpu.register[0] = 5;
        cpu.right_shift_x(0);
        assert_eq!(cpu.register[0], 2);
        assert_eq!(cpu.register[0xF], 1);

        let mut cpu = Cpu::new();
        cpu.register[0] = 4;
        cpu.right_shift_x(0);
        assert_eq!(cpu.register[0], 2);
        assert_eq!(cpu.register[0xF], 0);
    }
    #[test]
    fn sub_y_from_x_test() {
        let mut cpu = Cpu::new();
        cpu.register[0] = 7;
        cpu.register[1] = 5;
        cpu.sub_y_from_x(0, 1);
        assert_eq!(cpu.register[0], 2);
        assert_eq!(cpu.register[0xF], 1);

        let mut cpu = Cpu::new();
        cpu.register[0] = 4;
        cpu.register[1] = 5;
        cpu.sub_y_from_x(0, 1);
        assert_eq!(cpu.register[0], 255);
        assert_eq!(cpu.register[0xF], 0);
    }
    #[test]
    fn add_y_to_x_test() {
        let mut cpu = Cpu::new();
        cpu.register[0] = u8::MAX;
        cpu.register[1] = 2;
        cpu.add_y_to_x(0, 1);
        assert_eq!(cpu.register[0], 1);
        assert_eq!(cpu.register[0xF], 1);

        let mut cpu = Cpu::new();
        cpu.register[0] = 2;
        cpu.register[1] = 2;
        cpu.add_y_to_x(0, 1);
        assert_eq!(cpu.register[0], 4);
        assert_eq!(cpu.register[0xF], 0);
    }
    #[test]
    fn set_x_xxory() {
        let mut cpu = Cpu::new();
        let x = 3;
        let y = 7;
        cpu.register[x as usize] = x;
        cpu.register[y as usize] = y;
        cpu.set_x_xxory(x as u8, y as u8);
        assert_eq!(cpu.register[x as usize], 0b100);
    }
    #[test]
    fn set_x_xandy() {
        let mut cpu = Cpu::new();
        let x = 3;
        let y = 5;
        cpu.register[x as usize] = x;
        cpu.register[y as usize] = y;
        cpu.set_x_xandy(x as u8, y as u8);
        assert_eq!(cpu.register[x as usize], 0b001);
    }
    #[test]
    fn set_x_xory() {
        let mut cpu = Cpu::new();
        let x = 2;
        let y = 5;
        cpu.register[x as usize] = x;
        cpu.register[y as usize] = y;
        cpu.set_x_xory(x as u8, y as u8);
        assert_eq!(cpu.register[x as usize], 0b111);
    }
    #[test]
    fn skip_if_x_eq_y() {
        let mut cpu = Cpu::new();
        let x: u8 = 5;
        let y: u8 = 5;
        cpu.register[x as usize] = x;
        cpu.register[y as usize] = y;
        cpu.skip_if_x_eq_y(x, y);
        assert_eq!(cpu.program_counter, 0x202);

        let mut cpu = Cpu::new();
        let x: u8 = 5;
        let y: u8 = 6;
        cpu.register[x as usize] = x;
        cpu.register[y as usize] = y;
        cpu.skip_if_x_eq_y(x, y);
        assert_eq!(cpu.program_counter, 0x200);
    }
    #[test]
    fn skip_if_not_x() {
        let mut cpu = Cpu::new();
        let x = 5;
        cpu.register[x] = 5;
        cpu.skip_if_not_x(x as u8, 5);
        assert_eq!(cpu.program_counter, 0x200);

        let mut cpu = Cpu::new();
        let x = 5;
        cpu.register[x] = 5;
        cpu.skip_if_not_x(x as u8, 6);
        assert_eq!(cpu.program_counter, 0x202);
    }
    #[test]
    fn skip_if_x_test() {
        let mut cpu = Cpu::new();
        let x = 5;
        cpu.register[x] = 5;
        cpu.skip_if_x(x as u8, 5);
        assert_eq!(cpu.program_counter, 0x202);

        let mut cpu = Cpu::new();
        let x = 5;
        cpu.register[x] = 5;
        cpu.skip_if_x(x as u8, 6);
        assert_eq!(cpu.program_counter, 0x200);
    }

    #[test]
    fn call_and_return_subroutine_test() {
        let mut cpu = Cpu::new();
        cpu.call_subroutine(0x400);
        assert_eq!(cpu.program_counter, 0x400);
        assert_eq!(cpu.stack_pointer, 1);
        assert_eq!(cpu.stack[0], 0x200);
        cpu.return_from_subroutine();
        assert_eq!(cpu.program_counter, 0x200);
        assert_eq!(cpu.stack_pointer, 0);
    }

    #[test]
    #[should_panic]
    fn return_from_subroutine_test_panic() {
        let mut cpu = Cpu::new();
        cpu.return_from_subroutine();
    }

    #[test]
    fn return_from_subroutine_test() {
        let mut cpu = Cpu::new();
        cpu.stack_pointer = 15;
        cpu.stack[14] = 0x300;
        cpu.return_from_subroutine();
        assert_eq!(cpu.program_counter, 0x300);
        assert_eq!(cpu.stack_pointer, 14);
    }
    #[test]
    #[should_panic]
    fn call_subroutine_test_panic_case() {
        let mut cpu = Cpu::new();
        cpu.stack_pointer = 16;
        cpu.call_subroutine(0x300);
    }
    #[test]
    fn call_subroutine_test() {
        let mut cpu = Cpu::new();
        cpu.call_subroutine(0x300);
        assert_eq!(cpu.program_counter, 0x300);
        assert_eq!(cpu.stack[0], 0x200);
        assert_eq!(cpu.stack_pointer, 1);
    }
    #[test]
    fn jump_to_subroutine_test() {
        let mut cpu = Cpu::new();
        cpu.jump_to_subroutine(0x300);
        assert_eq!(cpu.program_counter, 0x300);
    }
    #[test]
    fn next_opcode_test() {
        let mut cpu = Cpu::new();
        cpu.memory[0x200] = 0x15;
        cpu.memory[0x201] = 0xFC;
        assert_eq!(cpu.next_opcode(), 0x15FC);
    }
}
