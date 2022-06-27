#[derive(Debug)]
pub struct Cpu {
    memory: [u8; 4096],
    register: [u16; 16],
    program_counter: usize,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            memory: [0; 4096],
            register: [0; 16],
            program_counter: 0x200,
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
                _ => todo!("TODO: {:0x}", opcode),
            }

            self.program_counter_increase();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn next_opcode_test() {
        let mut cpu = Cpu::new();
        cpu.memory[0x200] = 0x15;
        cpu.memory[0x201] = 0xFC;
        assert_eq!(cpu.next_opcode(), 0x15FC);
    }
}
