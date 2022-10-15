use crate::{
    gba::Gba,
    protocol::{
        EmulatorChannels,
        Message,
    }
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum EmulatorState {
    Pause,
    Run,
}

#[derive(Debug, Clone)]
pub struct Emulator {
    channels: EmulatorChannels,
    state: EmulatorState,
    gba: Gba,
}

impl Emulator {
    pub fn new(channels: EmulatorChannels) -> Self {
        Self {
            channels,
            state: EmulatorState::Pause,
            gba: Gba::new()
        }
    }

    /// Process a single message
    fn process_message(&mut self, message: &Message) {
        match message {
            Message::Run => self.state = EmulatorState::Run,
            Message::Pause => self.state = EmulatorState::Pause,
            _ => unimplemented!(),
        }
    }

    pub fn run(&mut self) -> ! {
        loop {
            // Process all available messages
            while let Some(message) = self.channels.try_receive() {
                self.process_message(&message);
            }

            // Take the appropriate action depending on the current state
            match self.state {
                EmulatorState::Pause => {
                    if let Some(message) = self.channels.wait() {
                        self.process_message(&message);
                    }
                },
                EmulatorState::Run => {
                    self.gba.step();
                },
            }
        }
    }
}