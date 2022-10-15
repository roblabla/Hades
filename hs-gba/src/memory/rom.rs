use crate::{
    memory::{
        Address,
        bus::Bus,
    },
    core::{
        RegisterSize,
        Word,
    }, gba::GBA
};

const ROM_SIZE: usize = 4096;//0x02000_0000;
const ROM_MASK: usize = ROM_SIZE - 1;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Rom {
    pub data: [u8; ROM_SIZE]
}

impl Rom {
    pub fn new() -> Self {
        Self {
            data: [0; ROM_SIZE],
        }
    }
}

impl Bus for Rom {
    fn read<T: RegisterSize>(gba: &mut GBA, mut address: Address) -> T {
        address.mask(ROM_MASK as Word);

        gba.scheduler.idle(1);

        unsafe {
            *((gba.memory.rom.data.as_ptr().add(address.value())) as *const T)
        }
    }

    fn write<T: RegisterSize>(gba: &mut GBA, mut address: Address, value: T) {
        address.mask(ROM_MASK as Word);

        gba.scheduler.idle(1);

        unsafe {
            *((gba.memory.rom.data.as_ptr().add(address.value())) as *mut T) = value;
        }
    }
}