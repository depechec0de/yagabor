use crate::{gameboy::{mmu::Address, cpu::cpu::ClockCycles, gameboy::GameBoy, ppu::{PPU, BGMAP0_ADDRESS, BGMAP1_ADDRESS}}};

use super::interrupts::{Interrupts, Interruption};

const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;

pub(crate) type Frame = [[Pixel; SCREEN_WIDTH]; SCREEN_HEIGHT];

pub(crate) const BLACK_FRAME: Frame = [[Pixel::DarkGray; SCREEN_WIDTH]; SCREEN_HEIGHT];

pub(crate) const LCD_CONTROL_ADDRESS: Address = 0xFF40;
pub(crate) const LCD_STATUS_ADDRESS: Address = 0xFF41;
pub(crate) const LCD_SCY_ADDRESS: Address = 0xFF42;
pub(crate) const LCD_SCX_ADDRESS: Address = 0xFF43;
pub(crate) const LCD_LY_ADDRESS: Address = 0xFF44;
pub(crate) const LCD_LYC_ADDRESS: Address = 0xFF45;
pub(crate) const LCD_OAMDMA_ADDRESS: Address = 0xFF46;
pub(crate) const LCD_BGPALETTE_ADDRESS: Address = 0xFF47;
pub(crate) const LCD_OBP0_ADDRESS: Address = 0xFF48;
pub(crate) const LCD_OBP1_ADDRESS: Address = 0xFF49;
pub(crate) const LCD_WY_ADDRESS: Address = 0xFF4A;
pub(crate) const LCD_WX_ADDRESS: Address = 0xFF4B;

pub(crate) const CLOCKS_SEARCHING_OAM: u16 = 80;
pub(crate) const CLOCKS_TRANSFERING: u16 = 172;
pub(crate) const CLOCKS_HBLANK: u16 = 204;
pub(crate) const CLOCKS_VBLANK: u16 = 456;

#[derive(Clone, Copy)]
pub(crate) enum Pixel {
    White, DarkGray, LightGray, Black
}

#[derive(Clone, Debug)]
pub(crate) enum LCDMode {
    SearchingOAM, Transfering, HBlank, VBlank 
}

pub(crate) enum LCDControl {
    Power, WindowTileMap, WindowEnable, BGandWindowTileSet, BGTileMap, SpriteSize,
    SpritesEnabled, BGEnabled 
}

pub(crate) struct LCD {
    control: u8,
    clock: u16,
    mode: LCDMode,
    scanline: u8,
    scy: u8,
    scx: u8,
    bgpalette: u8,
    framebuffer: Frame
}

impl LCD {
    pub(crate) fn new() -> Self {
        LCD { control:0, clock: 0, mode: LCDMode::SearchingOAM , scanline: 0, scy: 0, scx: 0, bgpalette: 0, framebuffer: BLACK_FRAME }
    }

    // https://gbdev.io/pandocs/STAT.html#stat-modes
    pub(crate) fn tick(gb: &mut GameBoy, cycles: ClockCycles) {
        gb.io.lcd.clock = gb.io.lcd.clock.wrapping_add(cycles);

        match LCD::mode(gb) {
            LCDMode::SearchingOAM => {
                if LCD::clock(gb) >= CLOCKS_SEARCHING_OAM {
                    LCD::reset_clock(gb);
                    LCD::start_mode(gb, LCDMode::Transfering);
                }
            },
            LCDMode::Transfering => {
                if LCD::clock(gb) >= CLOCKS_TRANSFERING {
                    LCD::reset_clock(gb);
                    LCD::start_mode(gb, LCDMode::HBlank);
                    LCD::render_scanline(gb);
                }
            },
            LCDMode::HBlank => {
                if LCD::clock(gb) >= CLOCKS_HBLANK {
                    LCD::reset_clock(gb);
                    LCD::next_scanline(gb);

                    if LCD::read_scanline(gb) == 143 {
                        Interrupts::turnon(gb, Interruption::VBlank);
                        LCD::start_mode(gb, LCDMode::VBlank);
                    }else{
                        LCD::start_mode(gb, LCDMode::SearchingOAM);
                    }
                }
            },
            LCDMode::VBlank => {
                if LCD::clock(gb) >= CLOCKS_VBLANK {
                    LCD::reset_clock(gb);
                    LCD::next_scanline(gb);

                    if LCD::read_scanline(gb) > 153 {
                        LCD::start_mode(gb, LCDMode::SearchingOAM);
                        LCD::reset_scanline(gb);
                    }
                }
            },
        }
    }

    pub(crate) fn render_scanline(gb: &mut GameBoy) {
        // let lcd = &gb.io.lcd;
        // let tiles = PPU::tile_set(gb);
        // let bgmaparea = LCD::read_control(gb, LCDControl::BGTileMap);

        // let mut bgmapoff = if bgmaparea { BGMAP1_ADDRESS } else { BGMAP0_ADDRESS };
        // bgmapoff += (((lcd.scanline + lcd.scy) & 0xFF) >> 3) as u16;

        // let linesoff = (lcd.scx >> 3) as u16;

        // let y = (lcd.scanline + lcd.scy) & 0b00000111;
        // let x = lcd.scx & 0b00000111;

        // let canvasoffs = lcd.scanline * 160 * 4;

        // let tile = PPU::read_byte(gb, bgmapoff + linesoff);

        // for i in 0..160 {
            
        // }
    }

    pub(crate) fn read_framebuffer(gb: &GameBoy) -> Frame {
        gb.io.lcd.framebuffer
    }

    pub(crate) fn mode(gb: &GameBoy) -> LCDMode {
        gb.io.lcd.mode.clone()
    }

    pub(crate) fn clock(gb: &GameBoy) -> u16 {
        gb.io.lcd.clock
    }

    pub(crate) fn reset_clock(gb: &mut GameBoy) {
        gb.io.lcd.clock = 0;
    }

    pub(crate) fn start_mode(gb: &mut GameBoy, mode: LCDMode) {
        gb.io.lcd.mode = mode;
    }

    pub(crate) fn next_scanline(gb: &mut GameBoy) {
        gb.io.lcd.scanline += 1;
    }

    pub(crate) fn read_scanline(gb: &GameBoy) -> u8 {
        gb.io.lcd.scanline
    }

    pub(crate) fn reset_scanline(gb: &mut GameBoy) {
        gb.io.lcd.scanline = 0;
    }

    pub(crate) fn read_control(gb: &GameBoy, parameter: LCDControl) -> bool {
        match parameter {
            LCDControl::Power               => (gb.io.lcd.control & 0b10000000) > 0, 
            LCDControl::WindowTileMap       => (gb.io.lcd.control & 0b01000000) > 0, 
            LCDControl::WindowEnable        => (gb.io.lcd.control & 0b00100000) > 0,  
            LCDControl::BGandWindowTileSet  => (gb.io.lcd.control & 0b00010000) > 0,  
            LCDControl::BGTileMap           => (gb.io.lcd.control & 0b00001000) > 0,  
            LCDControl::SpriteSize          => (gb.io.lcd.control & 0b00000100) > 0, 
            LCDControl::SpritesEnabled      => (gb.io.lcd.control & 0b00000010) > 0,  
            LCDControl::BGEnabled           => (gb.io.lcd.control & 0b00000001) > 0,  
        }
    }

    pub(crate) fn set_control(gb: &mut GameBoy, parameter: LCDControl) {
        match parameter {
            LCDControl::Power               => gb.io.lcd.control |= 0b10000000, 
            LCDControl::WindowTileMap       => gb.io.lcd.control |= 0b01000000, 
            LCDControl::WindowEnable        => gb.io.lcd.control |= 0b00100000,  
            LCDControl::BGandWindowTileSet  => gb.io.lcd.control |= 0b00010000,  
            LCDControl::BGTileMap           => gb.io.lcd.control |= 0b00001000,  
            LCDControl::SpriteSize          => gb.io.lcd.control |= 0b00000100, 
            LCDControl::SpritesEnabled      => gb.io.lcd.control |= 0b00000010,  
            LCDControl::BGEnabled           => gb.io.lcd.control |= 0b00000001,  
        }
    }

    pub(crate) fn read_byte(gb: &GameBoy, address: Address) -> u8 {
        match address {
            LCD_LY_ADDRESS => { gb.io.lcd.scanline },
            LCD_SCY_ADDRESS => { gb.io.lcd.scy },
            LCD_SCX_ADDRESS => { gb.io.lcd.scx },
            LCD_CONTROL_ADDRESS => { gb.io.lcd.control },
            LCD_BGPALETTE_ADDRESS => { gb.io.lcd.bgpalette },
            _ => 0
        }
    }

    pub(crate) fn write_byte(gb: &mut GameBoy, address: Address, value: u8) {
        match address {
            LCD_LY_ADDRESS => { gb.io.lcd.scanline = value },
            LCD_SCY_ADDRESS => { gb.io.lcd.scy = value },
            LCD_SCX_ADDRESS => { gb.io.lcd.scx = value },
            LCD_CONTROL_ADDRESS => { gb.io.lcd.control = value },
            LCD_BGPALETTE_ADDRESS => { gb.io.lcd.bgpalette = value },
            _ => {}
        }
    }
    
    
}