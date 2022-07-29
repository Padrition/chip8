use std::fs::{ReadDir, read};
pub struct Cartridge {
    pub game_paths: Vec<String>,
    pub rom: Vec<u8>,
}

impl Cartridge {
    pub fn new() -> Cartridge {
        Cartridge { 
            game_paths: Vec::<String>::new(),
            rom: Vec::<u8>::new()
        }
    }
    pub fn load_game_paths(&mut self, dir: ReadDir){
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
}
