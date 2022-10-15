use std::num::Wrapping;

use num::{
    Num,
    traits::{
        WrappingAdd,
        WrappingSub,
        WrappingMul,
    },
};

pub type Byte = u8;
pub type HalfWord = u16;
pub type Word = u32;

pub trait RegisterSize: Copy + Num + WrappingAdd + WrappingSub + WrappingMul {}
impl RegisterSize for Byte {}
impl RegisterSize for HalfWord {}
impl RegisterSize for Word {}

pub type Register = Wrapping<Word>;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum RegisterIndex {
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
    R8 = 8,
    R9 = 9,
    R10 = 10,
    FP = 11,
    IP = 12,
    SP = 13,
    LR = 14,
    PC = 15,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct Registers {
    pub general: [Register; 16],
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct ARM7TDMI {
    pub registers: Registers,
}

impl ARM7TDMI {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pc(&self) -> Register {
        self.registers.general[RegisterIndex::PC as usize]
    }

    pub fn pc_mut(&mut self) -> &mut Register {
        &mut self.registers.general[RegisterIndex::PC as usize]
    }
}