#[derive(PartialEq)]
pub struct Emulator{
    pub emulator_state: EmulatorState,
}

#[derive(PartialEq)]
pub enum EmulatorState{
    InGame,
    InRomLoader,
}


impl Emulator{
    pub fn new() -> Emulator{
        Emulator{
            emulator_state: EmulatorState::InRomLoader,
        }
    }
    pub fn switch_state(&mut self){
        self.emulator_state = if self.emulator_state == EmulatorState::InRomLoader{
            EmulatorState::InGame
        }else {
            EmulatorState::InRomLoader
        }
    }
}