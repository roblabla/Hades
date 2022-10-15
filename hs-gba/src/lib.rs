#![allow(dead_code)]

mod core;
mod emulator;
mod gba;
mod scheduler;
mod memory;
mod ppu;

pub use emulator::Emulator;
pub use ppu::{SCREEN_WIDTH, SCREEN_HEIGHT};