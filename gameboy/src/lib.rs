pub mod emulation;
pub mod cartridge;
pub mod io;
pub(crate) mod gameboy;
mod ppu;
mod rom;
mod cpu;
mod mmu;

pub enum Button {
    Up, Down, Left, Right, Start, Select, A, B
}

pub const SCREEN_WIDTH: u32 = 160;
pub const SCREEN_HEIGHT: u32 = 144;

pub const BACKGROUND_WIDTH: u32 = 256;
pub const BACKGROUND_HEIGHT: u32 = 256;

pub const TILEDATA_WIDTH: u32 = 128;
pub const TILEDATA_HEIGHT: u32 = 192;