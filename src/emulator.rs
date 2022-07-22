pub struct Emulator{
    pub emulator_state: EmulatorState,
    pub emulator_choice: EmulatorChoice,
}

pub enum EmulatorState{
    InGame,
    InMenu,
}

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
}