#[derive(PartialEq)]
pub struct Emulator{
    pub emulator_state: EmulatorState,
    pub emulator_choice: EmulatorChoice,
}

#[derive(PartialEq)]
pub enum EmulatorState{
    InGame,
    InMenu,
}

#[derive(PartialEq)]
pub enum EmulatorChoice{
    LoadRom,
    Quit,
}

impl Emulator{
    pub fn new() -> Emulator{
        Emulator{
            emulator_state: EmulatorState::InMenu,
            emulator_choice: EmulatorChoice::LoadRom,
        }
    }
    pub fn switch_state(&mut self){
        self.emulator_state = if self.emulator_state == EmulatorState::InGame{
            EmulatorState::InMenu
        }else{
            EmulatorState::InGame
        }
    }
    pub fn switch_choice(&mut self){
        self.emulator_choice = if self.emulator_choice == EmulatorChoice::LoadRom{
            EmulatorChoice::Quit
        }else{
            EmulatorChoice::LoadRom
        }
    }
}