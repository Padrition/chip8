use piston::input::*;

pub struct Keypad{
    key_map: [bool;16],
}

impl Keypad{
    pub fn new()->Keypad{
        Keypad{
            key_map: [false;16],
        }
    }
    pub fn map_keys(&self) -> &[bool;16]{
        &self.key_map
    }

    pub fn presse_key(&mut self, key: Key){
        match key{
            Key::D1 => self.key_map[0x0] = true,
            Key::D2 => self.key_map[0x1] = true,
            Key::D3 => self.key_map[0x2] = true,
            Key::D4 => self.key_map[0x3] = true,
            Key::Q => self.key_map[0x4] = true,
            Key::W => self.key_map[0x5] = true,
            Key::E => self.key_map[0x6] = true,
            Key::R => self.key_map[0x7] = true,
            Key::A => self.key_map[0x8] = true,
            Key::S => self.key_map[0x9] = true,
            Key::D => self.key_map[0xA] = true,
            Key::F => self.key_map[0xB] = true,
            Key::Z => self.key_map[0xC] = true,
            Key::X => self.key_map[0xD] = true,
            Key::C => self.key_map[0xE] = true,
            Key::V => self.key_map[0xF] = true,
            _ => {},
        }
    }

    pub fn release_key(&mut self, key: Key){
        match key{
            Key::D1 => self.key_map[0x0] = false,
            Key::D2 => self.key_map[0x1] = false,
            Key::D3 => self.key_map[0x2] = false,
            Key::D4 => self.key_map[0x3] = false,
            Key::Q => self.key_map[0x4] = false,
            Key::W => self.key_map[0x5] = false,
            Key::E => self.key_map[0x6] = false,
            Key::R => self.key_map[0x7] = false,
            Key::A => self.key_map[0x8] = false,
            Key::S => self.key_map[0x9] = false,
            Key::D => self.key_map[0xA] = false,
            Key::F => self.key_map[0xB] = false,
            Key::Z => self.key_map[0xC] = false,
            Key::X => self.key_map[0xD] = false,
            Key::C => self.key_map[0xE] = false,
            Key::V => self.key_map[0xF] = false,
            _ => {},
        }
    }
}