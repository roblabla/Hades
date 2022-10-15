#![allow(dead_code)]

mod core;
mod emu;
mod gba;
mod scheduler;
mod memory;
mod ppu;

pub use emu::Emulator;
pub use ppu::{SCREEN_WIDTH, SCREEN_HEIGHT};