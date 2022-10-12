pub type Register = u32;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct Registers {
    pub general: [Register; 16],
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct ARM7TDMI {
    pub registers: Registers,
}