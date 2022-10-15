use crate::{
    core::{
        ARM7TDMI,
        Word
    },
    scheduler::Scheduler,
    memory::{
        bus::Bus,
        Memory,
        Address,
    }
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Gba {
    pub core: ARM7TDMI,
    pub scheduler: Scheduler,
    pub memory: Memory,
}

impl Gba {
    pub fn new() -> Self {
        Self {
            core: ARM7TDMI::new(),
            scheduler: Scheduler::new(),
            memory: Memory::new(),
        }
    }

    pub fn step(&mut self) {
        let _op: Word = Memory::read(self, Address::from(self.core.pc()));

        //println!("OP is {op:#010x}");

        *self.core.pc_mut() += std::mem::size_of::<Word>() as Word;
    }
}
