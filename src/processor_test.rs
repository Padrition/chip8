use super::*;

#[test]
fn load_rom_test() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::new();
    cartridge.rom = vec![55, 55, 55];

    cpu.load_rom(&cartridge);

    assert_eq!(cpu.memory[0x200], 55);
    assert_eq!(cpu.memory[0x201], 55);
    assert_eq!(cpu.memory[0x202], 55);
}
#[test]
fn read_memory_to_registers_test() {
    let mut cpu = Cpu::new();
    cpu.i = 0x300;
    for j in 0..cpu.register.len() {
        cpu.memory[0x300 + j] = j as u8;
    }
    cpu.read_memory_to_registers(15);
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
    cpu.store_registers_to_memory(15);
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
    cpu.memory[0x200] = 0x80;
    cpu.memory[0x201] = 0x0E;
    cpu.run_next_instruction();
    assert_eq!(cpu.register[0xF], 0);
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
    //call subroutine
    cpu.memory[0x200] = 0x24;
    cpu.memory[0x201] = 0x00;
    //return from subroutine
    cpu.memory[0x400] = 0x00;
    cpu.memory[0x401] = 0xEE;

    cpu.run_next_instruction();
    assert_eq!(cpu.program_counter, 0x400);
    assert_eq!(cpu.stack_pointer, 1);
    assert_eq!(cpu.stack[0], 0x200);

    cpu.run_next_instruction();
    assert_eq!(cpu.program_counter, 0x202);
    assert_eq!(cpu.stack_pointer, 0);
}
#[test]
#[should_panic]
fn return_from_subroutine_test_panic() {
    let mut cpu = Cpu::new();
    cpu.return_from_subroutine();
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
    cpu.memory[0x200] = 0x23;
    cpu.memory[0x202] = 0x00;
    cpu.run_next_instruction();
    assert_eq!(cpu.program_counter, 0x300);
    assert_eq!(cpu.stack[0], 0x200);
    assert_eq!(cpu.stack_pointer, 1);
}
#[test]
fn jump_to_subroutine_test() {
    let mut cpu = Cpu::new();
    cpu.memory[0x200] = 0x13;
    cpu.memory[0x202] = 0x00;
    cpu.run_next_instruction();
    assert_eq!(cpu.program_counter, 0x300);
}
#[test]
fn next_opcode_test() {
    let mut cpu = Cpu::new();
    cpu.memory[0x200] = 0x15;
    cpu.memory[0x201] = 0xFC;
    assert_eq!(cpu.next_opcode(), 0x15FC);
}
