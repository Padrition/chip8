#[derive(Debug)]
pub struct Cpu {
    memory: [u8; 4096],
    register: [u16; 16],
    program_counter: usize,
    stack: [u16;16],
    stack_pointer: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            memory: [0; 4096],
            register: [0; 16],
            program_counter: 0x200,
            stack: [0;16],
            stack_pointer: 0,
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
                (0x1, _, _, _,) => self.jump_to_subroutine(nnn),
                (0x2, _, _, _)  => self.call_subroutine(nnn),
                (0x3, _, _, _)  => self.skip_if_x(x, kk),
                (0x4, _, _, _)  => self.skip_if_not_x(x, kk),
                (0x5, _, _, 0x0) => self.skip_if_x_eq_y(x,y),
                (0x6, _, _, _) => self.store_to_x(x, kk),
                (0x7, _, _, _) => self.add_to_x(x, kk),
                (0x8, _, _, 0x0) => self.store_y_to_x(x,y),
                (0x8, _, _, 0x1) => self.set_x_xory(x,y),
                (0x8, _, _, 0x2) => self.set_x_xandy(x,y),
                (0x8, _, _, 0x3) => self.set_x_xxory(x,y),
                (0x8, _, _, 0x4) => self.add_y_to_x(x,y),
                _ => todo!("TODO: {:0x}", opcode),
            }

            self.program_counter_increase();
        }
    }
    fn add_y_to_x(&mut self, x: u8, y: u8){
        let vx = self.register[x as usize];
        let vy = self.register[y as usize];

        let (vx, carry) = vx.overflowing_add(vy);
        self.register[x as usize] = vx;
        if carry {
            self.register[0xF] = 1;
        }else{
            self.register[0xF] = 0;
        } 
    }

    fn set_x_xxory(&mut self, x:u8, y:u8){
        let vx = self.register[x as usize];
        let vy = self.register[y as usize];

        self.register[x as usize] = vx ^ vy;
    }

    fn set_x_xandy(&mut self, x: u8, y:u8) {
        let vx = self.register[x as usize];
        let vy = self.register[y as usize];

        self.register[x as usize] = vx & vy;
    }

    fn set_x_xory(&mut self, x: u8, y:u8) {
        let vx = self.register[x as usize];
        let vy = self.register[y as usize];

        self.register[x as usize] = vx | vy;
    }

    fn store_y_to_x(&mut self, x: u8, y: u8){
        self.register[x as usize] = self.register[y as usize];
    }

    fn add_to_x(&mut self, x: u8, kk: u8){
        self.register[x as usize] += kk as u16;
    }

    fn store_to_x(&mut self, x: u8, kk: u8){
        self.register[x as usize] = kk as u16;
    }

    fn skip_if_x_eq_y(&mut self, x: u8, y: u8){
        if self.register[x as usize] == self.register[y as usize]{
            self.program_counter_increase();
        }
    }

    fn skip_if_not_x(&mut self, x: u8, kk: u8){
        if self.register[x as usize] as u8 != kk{
            self.program_counter_increase();
        }
    }

    fn skip_if_x(&mut self, x: u8, kk: u8){
        if self.register[x as usize] as u8 == kk{
            self.program_counter_increase();
        }
    }

    fn return_from_subroutine(&mut self){
        if self.stack_pointer == 0 {
            panic!("Stack underflow!")
        }
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer as usize] as usize;
    }

    fn call_subroutine(&mut self, nnn: u16){
        if self.stack_pointer as usize > self.stack.len(){
            panic!("Stack overflow!")
        }

        self.stack[self.stack_pointer as usize] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.jump_to_subroutine(nnn);

    }

    fn screen_clear(&mut self){
        todo!("clear screen");
    }

    fn jump_to_subroutine(&mut self, nnn: u16){
        self.program_counter = nnn as usize;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn add_y_to_x_test() {
        let mut cpu = Cpu::new();
        cpu.register[0] = u16::MAX;
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
        let x: u8= 5;
        let y: u8 = 5;
        cpu.register[x as usize] = x as u16;
        cpu.register[y as usize] = y as u16;
        cpu.skip_if_x_eq_y(x, y);
        assert_eq!(cpu.program_counter, 0x202);

        let mut cpu = Cpu::new();
        let x: u8= 5;
        let y: u8 = 6;
        cpu.register[x as usize] = x as u16;
        cpu.register[y as usize] = y as u16;
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
