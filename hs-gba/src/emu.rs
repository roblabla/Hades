use crate::gba::GBA;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum EmulatorState {
    Pause,
    Run,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Emulator {
    state: EmulatorState,
    gba: GBA,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            state: EmulatorState::Pause,
            gba: GBA::new()
        }
    }

    pub fn run(&mut self) -> ! {
        self.state = EmulatorState::Run;

        loop {
            match self.state {
                EmulatorState::Pause => (),
                EmulatorState::Run => {
                    self.gba.step();
                },
            }
        }
    }
}
