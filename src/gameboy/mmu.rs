use std::fmt;

use pretty_hex::*;

use crate::gameboy::{ppu::*, rom::*, cartridge::Cartridge};

use super::{io::{io::IO, interrupts::Interrupts}, gameboy::GameBoy};

pub(crate) type Address = u16;

pub(crate) const GAMEROM_0_BEGIN: Address = 0x0000;
pub(crate) const GAMEROM_0_END: Address = 0x3FFF;
pub(crate) const GAMEROM_0_SIZE: usize = (GAMEROM_0_END - GAMEROM_0_BEGIN + 1) as usize;

pub(crate) const GAMEROM_N_BEGIN: Address = 0x4000;
pub(crate) const GAMEROM_N_END: Address = 0x7FFF;
pub(crate) const GAMEROM_N_SIZE: usize = (GAMEROM_N_END - GAMEROM_N_BEGIN + 1) as usize;

pub(crate) const VRAM_BEGIN: Address = 0x8000;
pub(crate) const VRAM_END: Address = 0x9FFF;
pub(crate) const VRAM_SIZE: usize = (VRAM_END - VRAM_BEGIN + 1) as usize;

pub(crate) const EXTRAM_BEGIN: Address = 0xA000;
pub(crate) const EXTRAM_END: Address = 0xBFFF;
pub(crate) const EXTRAM_SIZE: usize = (EXTRAM_END - EXTRAM_BEGIN + 1) as usize;

pub(crate) const WRAM_BEGIN: Address = 0xC000;
pub(crate) const WRAM_END: Address = 0xDFFF;
pub(crate) const WRAM_SIZE: usize = (WRAM_END - WRAM_BEGIN + 1) as usize;

pub(crate) const ERAM_BEGIN: Address = 0xE000;
pub(crate) const ERAM_END: Address = 0xFDFF;

pub(crate) const OAM_BEGIN: Address = 0xFE00;
pub(crate) const OAM_END: Address = 0xFE9F;
pub(crate) const OAM_SIZE: usize = (OAM_END - OAM_BEGIN + 1) as usize;

pub(crate) const NOTUSABLE_BEGIN: Address = 0xFEA0;
pub(crate) const NOTUSABLE_END: Address = 0xFEFF;

pub(crate) const IO_BEGIN: Address = 0xFF00;
pub(crate) const IO_END: Address = 0xFF7F;
pub(crate) const IO_SIZE: usize = (IO_END - IO_BEGIN + 1) as usize;

const HRAM_BEGIN: Address = 0xFF80;
const HRAM_END: Address = 0xFFFE;
const HRAM_SIZE: usize = (HRAM_END - HRAM_BEGIN + 1) as usize;

pub(crate) const INTERRUPT_ENABLE_ADDRESS: Address = 0xFFFF;

pub(crate) struct MMU {
    is_boot_rom_mapped: bool,
    bootrom: ROM,
    eram: [u8; EXTRAM_SIZE],
    wram: [u8; WRAM_SIZE],
    hram: [u8; HRAM_SIZE],
}

impl MMU {
    pub fn new() -> Self {
        let bootrom = ROM::dmg();
        MMU { 
            is_boot_rom_mapped: true, 
            bootrom,
            eram: [0; EXTRAM_SIZE], 
            wram: [0; WRAM_SIZE], 
            hram: [0; HRAM_SIZE],
        }
    }

    pub(super) fn read_byte(gb: &GameBoy, address: Address) -> u8 {
        match address {
            GAMEROM_0_BEGIN ..= GAMEROM_0_END => {
                match address {
                    BOOT_BEGIN ..= BOOT_END => {
                        if gb.mmu.is_boot_rom_mapped {
                            gb.mmu.bootrom.read_byte(address)
                        }else{
                            Cartridge::read_byte(&gb, address)
                        }
                    },
                    _ => Cartridge::read_byte(&gb, address)
                }
            },
            GAMEROM_N_BEGIN ..= GAMEROM_N_END => Cartridge::read_byte(&gb, address),
            VRAM_BEGIN ..= VRAM_END => PPU::read_byte(gb, address),
            EXTRAM_BEGIN ..= EXTRAM_END => MMU::read_eram(gb, address),
            WRAM_BEGIN ..= WRAM_END => MMU::read_wram(gb, address),
            ERAM_BEGIN ..= ERAM_END => panic!("prohibited read 0x{:x} to echo ram", address),
            OAM_BEGIN ..= OAM_END => PPU::read_byte(gb, address),
            NOTUSABLE_BEGIN ..= NOTUSABLE_END => panic!("prohibited read 0x{:x}", address),
            IO_BEGIN ..= IO_END => IO::read_byte(gb, address),
            HRAM_BEGIN ..= HRAM_END => MMU::read_hram(gb, address),
            INTERRUPT_ENABLE_ADDRESS => Interrupts::read_enable(gb),
            _ => panic!("unmapped read {:x}", address),
        }
    }

    pub(super) fn write_byte(gb: &mut GameBoy, address: Address, value: u8) {
        match address {
            GAMEROM_0_BEGIN ..= GAMEROM_0_END => {
                //panic!("Writing in ROM {:x} is not possible", address);
            },
            GAMEROM_N_BEGIN ..= GAMEROM_N_END => {
                //panic!("Writing in ROM {:x} is not possible", address);
            },
            VRAM_BEGIN ..= VRAM_END => PPU::write_byte(gb, address, value),
            EXTRAM_BEGIN ..= EXTRAM_END => MMU::write_eram(gb, address, value),
            WRAM_BEGIN ..= WRAM_END => MMU::write_wram(gb, address, value),
            ERAM_BEGIN ..= ERAM_END => panic!("prohibited write 0x{:x} to echo ram", address),
            OAM_BEGIN ..= OAM_END => PPU::write_byte(gb, address, value),
            NOTUSABLE_BEGIN ..= NOTUSABLE_END => panic!("prohibited write 0x{:x}", address),
            IO_BEGIN ..= IO_END => IO::write_byte(gb, address, value),
            HRAM_BEGIN ..= HRAM_END => MMU::write_hram(gb, address, value),
            INTERRUPT_ENABLE_ADDRESS => Interrupts::write_enable(gb, value),
            _ => panic!("unmapped write {:x}", address),
        };
    }

    fn read_wram(gb: &GameBoy, address: Address) -> u8 {
        gb.mmu.wram[address as usize - WRAM_BEGIN as usize]
    }

    fn read_eram(gb: &GameBoy, address: Address) -> u8 {
        gb.mmu.eram[address as usize - EXTRAM_BEGIN as usize]
    }

    fn read_hram(gb: &GameBoy, address: Address) -> u8 {
        gb.mmu.hram[address as usize - HRAM_BEGIN as usize]
    }

    fn write_wram(gb: &mut GameBoy, address: Address, value: u8) {
        gb.mmu.wram[address as usize - WRAM_BEGIN as usize] = value;
    }

    fn write_eram(gb: &mut GameBoy, address: Address, value: u8) {
        gb.mmu.eram[address as usize - EXTRAM_BEGIN as usize] = value;
    }

    fn write_hram(gb: &mut GameBoy, address: Address, value: u8) {
        gb.mmu.hram[address as usize - HRAM_BEGIN as usize] = value;
    }

    pub(crate) fn set_boot_mapping(gb: &mut GameBoy, value: u8) {
        gb.mmu.is_boot_rom_mapped = value == 0;
    }

    pub(super) fn read_next_byte(gb: &GameBoy, address: Address) -> u8 {
        MMU::read_byte(&gb, address+1)
    }
    
    pub(super) fn read_next_word(gb: &GameBoy, address: Address) -> u16 {
        ((MMU::read_byte(&gb, address+2) as u16) << 8) | (MMU::read_byte(&gb, address+1) as u16)
    }
}