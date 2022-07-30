use std::fs::{ReadDir, read, read_dir};
use std::path::Path;
pub struct Cartridge {
    pub game_paths: Vec<String>,
    pub rom: Vec<u8>,
    choosen_game: String,
}

impl Cartridge {
    pub fn new() -> Cartridge {
        let mut cartridge = Cartridge { 
            game_paths: Vec::<String>::new(),
            rom: Vec::<u8>::new(),
            choosen_game: String::new(), 
        };
        let path = read_dir("assets/").unwrap();
        cartridge.load_game_paths(path);
        cartridge.choosen_game = cartridge.game_paths[0].clone();
        cartridge.game_to_rom();
        cartridge
    }
    fn load_game_paths(&mut self, dir: ReadDir){
        for path in dir {
            let dir_entry = path.unwrap();
            let file_path = dir_entry.path();
            let file_path = file_path.as_path();
            let file_extension = file_path.extension().unwrap();
            if file_extension == "ch8"{
                let game_path_str = file_path.to_str().unwrap();
                let game_path = game_path_str.to_string();
                self.game_paths.push(game_path);
            }
        }
    }
    pub fn game_to_rom(&mut self){
        let buffer = read(self.choosen_game.clone()).expect("Faild to read a file.");
        self.rom = buffer;
    }
    pub fn get_game_name(&self) -> &str{
        let game = self.choosen_game.as_str();
        let game = game.trim_end_matches(".ch8");
        let game = game.trim_start_matches("assets/");
        game
    }
}
