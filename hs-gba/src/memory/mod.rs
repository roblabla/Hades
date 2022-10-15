pub mod bios;
pub mod bus;
pub mod rom;

use crate::{
    memory::{
        bios::Bios,
        bus::Bus,
        rom::Rom,
    },
    core::{
        RegisterSize,
        Word,
        Register,
    },
    gba::GBA,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Address(usize);

impl Address {
    pub fn value(self) -> usize {
        self.0
    }

    pub fn align<T: RegisterSize>(&mut self) {
        self.mask((std::mem::size_of::<T>() - 1) as Word);
    }

    pub fn mask(&mut self, mask: Word) {
        self.0 &= mask as usize;
    }
}

impl From<Word> for Address {
    fn from(value: Word) -> Self {
        Address(value as usize)
    }
}

impl From<Register> for Address {
    fn from(value: Register) -> Self {
        Address(value.0 as usize)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Memory {
    pub bios: Bios,
    pub rom: Rom,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            bios: Bios::new(),
            rom: Rom::new(),
        }
    }
}

impl Bus for Memory {
    fn read<T: RegisterSize>(gba: &mut GBA, address: Address) -> T {
        match address.value() >> 24 {
            0x00 => Bios::read(gba, address),
            _ => T::zero(),
        }
    }

    fn write<T: RegisterSize>(gba: &mut GBA, address: Address, value: T) {
        match address.value() >> 24 {
            0x00 => Bios::write(gba, address, value),
            _ => (),
        }
    }
}