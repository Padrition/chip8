use std::fs::read;
pub struct Cartridge {
    games: Vec<String>,
    pub rom: Vec<u8>,
}

impl Cartridge {
    pub fn new() -> Cartridge {
        Cartridge { 
            games: Vec::<String>::new(),
            rom: Vec::<u8>::new()
        }
    }
}
