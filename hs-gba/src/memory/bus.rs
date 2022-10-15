use crate::{
    gba::Gba,
    core::RegisterSize,
    memory::Address,
};

pub trait Bus {
    fn read<T: RegisterSize>(gba: &mut Gba, addr: Address) -> T;
    fn write<T: RegisterSize>(gba: &mut Gba, addr: Address, value: T);
}