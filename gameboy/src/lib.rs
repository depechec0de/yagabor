pub mod emulation;
pub mod cartridge;
pub(crate) mod io;
pub(crate) mod gameboy;
mod ppu;
mod rom;
mod cpu;
mod mmu;

pub const SCREEN_WIDTH: u32 = 160;
pub const SCREEN_HEIGHT: u32 = 144;

pub const BACKGROUND_WIDTH: u32 = 256;
pub const BACKGROUND_HEIGHT: u32 = 256;

pub const TILEDATA_WIDTH: u32 = 128;
pub const TILEDATA_HEIGHT: u32 = 192;

pub enum Button {
    Up, Down, Left, Right, Start, Select, A, B
}

#[derive(Clone, Copy, Debug)]
pub enum ColoredPixel {
    White, LightGray, DarkGray, Black
}

#[derive(Clone, Debug)]
pub struct GameBoyFrame {
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<ColoredPixel>
}