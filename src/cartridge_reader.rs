use std::fs::read;
pub struct Cartridge {
    pub rom: Vec<u8>,
}

impl Cartridge {
    pub fn new(filename: &str) -> Cartridge {
        let buffer = read(filename).expect("Faild to read a file.");

        Cartridge { rom: buffer }
    }
}
