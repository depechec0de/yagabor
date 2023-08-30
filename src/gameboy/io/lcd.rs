use crate::{gameboy::{mmu::{Address, VRAM_BEGIN}, cpu::cpu::ClockCycles, gameboy::GameBoy, ppu::{PPU, BGMAP0_ADDRESS, BGMAP1_ADDRESS, TilePixelValue}}, screen::Screen, debug::{TileDataFrame, TILEDATA_ROWS, TILEDATA_COLS}};

use super::interrupts::{Interrupts, Interruption};

pub(crate) const SCREEN_WIDTH: usize = 160;
pub(crate) const SCREEN_HEIGHT: usize = 144;

pub(crate) type Frame = [ColoredPixel; SCREEN_WIDTH * SCREEN_HEIGHT];

pub(crate) const BLACK_FRAME: Frame = [ColoredPixel::DarkGray; SCREEN_WIDTH * SCREEN_HEIGHT];

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
pub(crate) enum ColoredPixel {
    White, DarkGray, LightGray, Black
}

impl std::convert::From<ColoredPixel> for u8 {
    fn from(cp: ColoredPixel) -> Self {
        match cp {
            ColoredPixel::White => 0,
            ColoredPixel::LightGray => 1,
            ColoredPixel::DarkGray => 2,
            ColoredPixel::Black => 3,
        }
    }
}

impl std::convert::From<u8> for ColoredPixel {
    fn from(byte: u8) -> Self {
        if byte & 0b11 == 0b00 {
            ColoredPixel::White
        }else if byte & 0b11 == 0b01 {
            ColoredPixel::LightGray
        } else if byte & 0b11 == 0b10 {
            ColoredPixel::DarkGray
        } else {
            ColoredPixel::Black
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum LCDMode {
    SearchingOAM, Transfering, HBlank, VBlank 
}

pub(crate) enum LCDControl {
    Power, WindowTileMap, WindowEnable, BGandWindowTileSet, BGTileMap, SpriteSize,
    SpritesEnabled, BGEnabled 
}

// Given a Pixel index we map it to a color
#[derive(Clone, Copy)]
pub(crate) struct Palette {
    index0: ColoredPixel, index1: ColoredPixel, index2: ColoredPixel, index3: ColoredPixel
}

impl Palette {
    fn apply(&self, p: TilePixelValue) -> ColoredPixel {
        match p {
            TilePixelValue::Zero => self.index0,
            TilePixelValue::One => self.index1,
            TilePixelValue::Two => self.index2,
            TilePixelValue::Three => self.index3,
        }
    }
}

impl std::convert::From<u8> for Palette {
    fn from(byte: u8) -> Self {
        Palette { 
                index0: ColoredPixel::from(byte), 
                index1: ColoredPixel::from(byte >> 2),
                index2: ColoredPixel::from(byte >> 4), 
                index3: ColoredPixel::from(byte >> 6), 
        }
    }
}

impl std::convert::From<Palette> for u8 {
    fn from(p: Palette) -> Self {
        u8::from(p.index3) << 6 | u8::from(p.index2) << 4 | u8::from(p.index1) << 2 | u8::from(p.index0) 
    }
}

pub(crate) struct LCD {
    control: u8,
    clock: u16,
    mode: LCDMode,
    scanline: u8,
    scy: u8,
    scx: u8,
    bgpalette: Palette,
    framebuffer: Frame
}

impl LCD {
    pub(crate) fn new() -> Self {
        LCD { control:0, clock: 0, mode: LCDMode::SearchingOAM , scanline: 0, scy: 0, scx: 0, bgpalette: Palette::from(0), framebuffer: BLACK_FRAME }
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
        let bgenabled = LCD::read_control(gb, LCDControl::BGEnabled);
        let bgmaparea = LCD::read_control(gb, LCDControl::BGTileMap);
        //let bgaddr = LCD::read_control(gb, LCDControl::BGandWindowTileSet);

        let lcd = &mut gb.io.lcd;
        let ppu = &gb.ppu;

        let mut scan_line: [TilePixelValue; SCREEN_WIDTH] = [Default::default(); SCREEN_WIDTH];
        
        if bgenabled {
            // The x index of the current tile
            let mut tile_x_index = lcd.scx / 8;
            // The current scan line's y-offset in the entire background space is a combination
            // of both the line inside the view port we're currently on and the amount of the view port is scrolled
            let tile_y_index = lcd.scanline.wrapping_add(lcd.scy);
            // The current tile we're on is equal to the total y offset broken up into 8 pixel chunks
            // and multipled by the width of the entire background (i.e. 32 tiles)
            let tile_offset = (tile_y_index as u16 / 8) * 32u16;

            // Where is our tile map defined?
            let background_tile_map = if bgmaparea {
                0x9800
            } else {
                0x9C00
            };
            // Munge this so that the beginning of VRAM is index 0
            let tile_map_begin = background_tile_map - VRAM_BEGIN;
            // Where we are in the tile map is the beginning of the tile map
            // plus the current tile's offset
            let tile_map_offset = (tile_map_begin + tile_offset) as usize;

            // When line and scrollY are zero we just start at the top of the tile
            // If they're non-zero we must index into the tile cycling through 0 - 7
            let row_y_offset = tile_y_index % 8;
            let mut pixel_x_index = lcd.scx % 8;

            let mut canvas_buffer_offset = lcd.scanline as usize * SCREEN_WIDTH;
            // Start at the beginning of the line and go pixel by pixel
            for line_x in 0..SCREEN_WIDTH {
                // Grab the tile index specified in the tile map
                let tile_index = ppu.vram[tile_map_offset + tile_x_index as usize];

                let tile_value = ppu.tile_set[tile_index as usize][row_y_offset as usize]
                    [pixel_x_index as usize];
                let color: ColoredPixel = lcd.bgpalette.apply(tile_value);

                lcd.framebuffer[canvas_buffer_offset] = color;
                canvas_buffer_offset += 1;
                scan_line[line_x] = tile_value;
                // Loop through the 8 pixels within the tile
                pixel_x_index = (pixel_x_index + 1) % 8;

                // Check if we've fully looped through the tile
                if pixel_x_index == 0 {
                    // Now increase the tile x_offset by 1
                    tile_x_index = tile_x_index + 1;
                }

            }
        }
        
    }

    pub(crate) fn read_framebuffer(gb: &GameBoy) -> Frame {
        gb.io.lcd.framebuffer
    }

    pub(crate) fn read_tiledata(gb: &GameBoy) -> TileDataFrame {
        let tiles = PPU::tile_set(gb);
        let mut tdframe : TileDataFrame = [[[[ColoredPixel::White; 8]; 8]; TILEDATA_ROWS]; TILEDATA_COLS];

        for tx in 0..TILEDATA_COLS {
            for ty in 0..TILEDATA_ROWS {
                for px in 0..8{
                    for py in 0..8{
                        tdframe[tx][ty][px][py] = gb.io.lcd.bgpalette.apply(tiles[tx + ty*TILEDATA_COLS][px][py]);
                    }
                }
            }
        }
        
        tdframe
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
            LCD_BGPALETTE_ADDRESS => { u8::from(gb.io.lcd.bgpalette) },
            _ => 0
        }
    }

    pub(crate) fn write_byte(gb: &mut GameBoy, address: Address, value: u8) {
        match address {
            LCD_LY_ADDRESS => { gb.io.lcd.scanline = value },
            LCD_SCY_ADDRESS => { gb.io.lcd.scy = value },
            LCD_SCX_ADDRESS => { gb.io.lcd.scx = value },
            LCD_CONTROL_ADDRESS => { gb.io.lcd.control = value },
            LCD_BGPALETTE_ADDRESS => { gb.io.lcd.bgpalette = Palette::from(value) },
            _ => {}
        }
    }
    
    
}