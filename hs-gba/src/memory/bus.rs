use crate::{
    gba::GBA,
    core::RegisterSize,
    memory::Address,
};

pub trait Bus {
    fn read<T: RegisterSize>(gba: &mut GBA, addr: Address) -> T;
    fn write<T: RegisterSize>(gba: &mut GBA, addr: Address, value: T);
}