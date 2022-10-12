pub type Timestamp = u64;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct Scheduler {
    cycles: Timestamp,
}

impl Scheduler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn now(&self) -> Timestamp {
        self.cycles
    }
}