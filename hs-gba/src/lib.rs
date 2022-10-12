mod core;
mod scheduler;

use crate::{
    core::ARM7TDMI,
    scheduler::Scheduler,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum State {
    PAUSE,
    RUN,
}

impl Default for State {
    fn default() -> Self {
        Self::PAUSE
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct GBA {
    pub state: State,
    pub core: ARM7TDMI,
    pub scheduler: Scheduler,
}

impl GBA {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self) -> ! {
        self.state = State::RUN;

        loop {
            match self.state {
                State::PAUSE => (),
                State::RUN => {
                },
            }
        }
    }
}