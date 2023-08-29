use crate::gameboy::{mmu::Address, cpu::cpu::ClockCycles, gameboy::GameBoy};

use super::interrupts::{Interrupts, Interruption};

pub(crate) const LCD_CONTROL_ADDRESS: Address = 0xFF40;
pub(crate) const LCD_STATUS_ADDRESS: Address = 0xFF41;
pub(crate) const LCD_SCY_ADDRESS: Address = 0xFF42;
pub(crate) const LCD_SCX_ADDRESS: Address = 0xFF43;
pub(crate) const LCD_LY_ADDRESS: Address = 0xFF44;
pub(crate) const LCD_LYC_ADDRESS: Address = 0xFF45;

pub(crate) const LCD_WY_ADDRESS: Address = 0xFF4A;
pub(crate) const LCD_WX_ADDRESS: Address = 0xFF4B;

pub(crate) const CLOCKS_SEARCHING_OAM: u16 = 80;
pub(crate) const CLOCKS_TRANSFERING: u16 = 172;
pub(crate) const CLOCKS_HBLANK: u16 = 204;
pub(crate) const CLOCKS_VBLANK: u16 = 456;

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
    scanline: u8
}

impl LCD {
    pub(crate) fn new() -> Self {
        LCD { control:0, clock: 0, mode: LCDMode::SearchingOAM , scanline: 0 }
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
                    // TODO: write scanline to framebuffer?
                }
            },
            LCDMode::HBlank => {
                if LCD::clock(gb) >= CLOCKS_HBLANK {
                    LCD::reset_clock(gb);
                    LCD::next_scanline(gb);

                    if LCD::read_scanline(gb) == 143 {
                        Interrupts::turnon(gb, Interruption::VBlank);
                        LCD::start_mode(gb, LCDMode::VBlank);
                        // TODO: render?
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

    pub(crate) fn read_control(gb: &mut GameBoy, parameter: LCDControl) -> bool {
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
            _ => 0
        }
    }

    pub(crate) fn write_byte(gb: &mut GameBoy, address: Address, value: u8) {
        match address {
            LCD_LY_ADDRESS => { gb.io.lcd.scanline = value },
            LCD_CONTROL_ADDRESS => { gb.io.lcd.control = value },
            _ => {}
        }
    }
    
    
}