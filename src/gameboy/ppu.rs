use std::fmt;

use pretty_hex::*;

use super::{mmu::*, gameboy::GameBoy, io::lcd::ColoredPixel};

pub(crate) const BGMAP0_ADDRESS: Address = 0x9800;
pub(crate) const BGMAP1_ADDRESS: Address = 0x9C00;

#[derive(Copy,Clone)]
pub(crate) enum TilePixelValue {
    Zero,
    One,
    Two,
    Three,
}

impl Default for TilePixelValue {
    fn default() -> Self {
        TilePixelValue::Zero
    }
}

type Tile = [[TilePixelValue; 8]; 8];

fn empty_tile() -> Tile {
    [[TilePixelValue::Zero; 8]; 8]
}

pub(crate) struct PPU{
    pub(crate) vram: [u8; VRAM_SIZE],
    pub(crate) oam: [u8; OAM_SIZE],
    pub(crate) tile_set: [Tile; 384],
}

impl PPU {
    pub(super) fn new() -> PPU {
        PPU { 
            vram: [0x0; VRAM_SIZE], 
            oam: [0; OAM_SIZE],
            tile_set: [[[TilePixelValue::Zero; 8]; 8]; 384] 
        }
    }

    pub(crate) fn tile_set(gb: &GameBoy) -> [Tile; 384] {
        gb.ppu.tile_set
    }
    

    pub(super) fn read_byte(gb: &GameBoy, address: Address) -> u8 {
        match address {
            OAM_BEGIN ..= OAM_END => PPU::read_oam(gb, address),
            VRAM_BEGIN ..= VRAM_END => PPU::read_vram(gb, address),
            _ => panic!("Invalid read PPU")
        }
    }

    pub(super) fn write_byte(gb: &mut GameBoy, address: Address, value: u8) {
        match address {
            OAM_BEGIN ..= OAM_END => PPU::write_oam(gb, address, value),
            VRAM_BEGIN ..= VRAM_END => PPU::write_vram(gb, address, value),
            _ => panic!("Invalid write PPU")
        }
    }

    pub(super) fn read_oam(gb: &GameBoy, address: Address) -> u8 {
        gb.ppu.vram[(address - OAM_BEGIN) as usize]
    }  

    pub(super) fn write_oam(gb: &mut GameBoy, address: Address, value: u8) {
        gb.ppu.oam[(address - OAM_BEGIN) as usize] = value;
    }  

    pub(super) fn read_vram(gb: &GameBoy, address: Address) -> u8 {
        gb.ppu.vram[(address - VRAM_BEGIN) as usize]
    }

    pub(crate) fn write_vram(gb: &mut GameBoy, address: Address, value: u8) {
        let index = (address - VRAM_BEGIN) as usize;
        gb.ppu.vram[index] = value;
        // If our index is greater than 0x1800, we're not writing to the tile set storage
        // so we can just return.
        if index >= 0x1800 { return }

        // Tiles rows are encoded in two bytes with the first byte always
        // on an even address. Bitwise ANDing the address with 0xffe
        // gives us the address of the first byte.
        // For example: `12 & 0xFFFE == 12` and `13 & 0xFFFE == 12`
        let normalized_index = index & 0xFFFE;

        // First we need to get the two bytes that encode the tile row.
        let byte1 = gb.ppu.vram[normalized_index];
        let byte2 = gb.ppu.vram[normalized_index + 1];

        // A tiles is 8 rows tall. Since each row is encoded with two bytes a tile
        // is therefore 16 bytes in total.
        let tile_index = index / 16;
        // Every two bytes is a new row
        let row_index = (index % 16) / 2;

        // Now we're going to loop 8 times to get the 8 pixels that make up a given row.
        for pixel_index in 0..8 {
            // To determine a pixel's value we must first find the corresponding bit that encodes
            // that pixels value:
            // 1111_1111
            // 0123 4567
            //
            // As you can see the bit that corresponds to the nth pixel is the bit in the nth
            // position *from the left*. Bits are normally indexed from the right.
            //
            // To find the first pixel (a.k.a pixel 0) we find the left most bit (a.k.a bit 7). For
            // the second pixel (a.k.a pixel 1) we first the second most left bit (a.k.a bit 6) and
            // so on.
            //
            // We then create a mask with a 1 at that position and 0s everywhere else.
            //
            // Bitwise ANDing this mask with our bytes will leave that particular bit with its
            // original value and every other bit with a 0.
            let mask = 1 << (7 - pixel_index);
            let lsb = byte1 & mask;
            let msb = byte2 & mask;

            // If the masked values are not 0 the masked bit must be 1. If they are 0, the masked
            // bit must be 0.
            //
            // Finally we can tell which of the four tile values the pixel is. For example, if the least
            // significant byte's bit is 1 and the most significant byte's bit is also 1, then we
            // have tile value `Three`.
            let value = match (lsb != 0, msb != 0) {
                (true, true) => TilePixelValue::Three,
                (false, true) => TilePixelValue::Two,
                (true, false) => TilePixelValue::One,
                (false, false) => TilePixelValue::Zero,
            };

            gb.ppu.tile_set[tile_index][row_index][pixel_index] = value;
        }

    }
}

impl fmt::Display for PPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:x}-{:x}\n", "PPU", VRAM_BEGIN, VRAM_END)?;
        write!(f, "{}", pretty_hex(&self.vram))
    }
}