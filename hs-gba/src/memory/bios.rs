use crate::{
    memory::{
        Address,
        bus::Bus,
    },
    core::{
        RegisterSize,
        Word,
    }, gba::Gba
};

const BIOS_SIZE: usize = 0x0000_4000;
const BIOS_MASK: usize = BIOS_SIZE - 1;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Bios {
    pub data: [u8; BIOS_SIZE]
}

impl Bios {
    pub fn new() -> Self {
        //let data =
        Self {
            data: *include_bytes!("../../../games/bios/bios.bin"),
        }
    }
}

impl Bus for Bios {
    fn read<T: RegisterSize>(gba: &mut Gba, mut address: Address) -> T {
        address.mask(BIOS_MASK as Word);

        gba.idle(1);

        unsafe {
            *((gba.memory.bios.data.as_ptr().add(address.value())) as *const T)
        }
    }

    fn write<T: RegisterSize>(gba: &mut Gba, mut address: Address, value: T) {
        address.mask(BIOS_MASK as Word);

        gba.idle(1);

        unsafe {
            *((gba.memory.bios.data.as_ptr().add(address.value())) as *mut T) = value;
        }
    }
}