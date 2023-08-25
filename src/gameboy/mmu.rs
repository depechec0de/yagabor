use std::fmt;

use pretty_hex::*;

use crate::gameboy::{ppu::*, rom::*, cartridge::Cartridge};

use super::io::io::{IOEvent, IO};

pub(crate) type Address = u16;

const GAMEROM_0_BEGIN: Address = 0x0000;
const GAMEROM_0_END: Address = 0x3FFF;
const GAMEROM_0_SIZE: usize = (GAMEROM_0_END - GAMEROM_0_BEGIN + 1) as usize;

const GAMEROM_N_BEGIN: Address = 0x4000;
const GAMEROM_N_END: Address = 0x7FFF;
const GAMEROM_N_SIZE: usize = (GAMEROM_N_END - GAMEROM_N_BEGIN + 1) as usize;

const EXTRAM_BEGIN: Address = 0xA000;
const EXTRAM_END: Address = 0xBFFF;
const EXTRAM_SIZE: usize = (EXTRAM_END - EXTRAM_BEGIN + 1) as usize;

const WRAM_BEGIN: Address = 0xC000;
const WRAM_END: Address = 0xDFFF;
const WRAM_SIZE: usize = (WRAM_END - WRAM_BEGIN + 1) as usize;

const ERAM_BEGIN: Address = 0xE000;
const ERAM_END: Address = 0xFDFF;

const OAM_BEGIN: Address = 0xFE00;
const OAM_END: Address = 0xFE9F;
const OAM_SIZE: usize = (OAM_END - OAM_BEGIN + 1) as usize;

const NOTUSABLE_BEGIN: Address = 0xFEA0;
const NOTUSABLE_END: Address = 0xFEFF;

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
    cartridge: Cartridge,
    pub(crate) io: IO,
    ppu: PPU,
    eram: [u8; EXTRAM_SIZE],
    oam: [u8; OAM_SIZE],
    wram: [u8; WRAM_SIZE],
    hram: [u8; HRAM_SIZE],
}

impl MMU {
    pub fn new(cartridge: Cartridge, io: IO) -> Self {
        let bootrom = ROM::dmg();
        let ppu = PPU::new();
        MMU { 
            is_boot_rom_mapped: true, 
            cartridge,
            bootrom,
            io,
            ppu,
            eram: [0; EXTRAM_SIZE], 
            oam: [0; OAM_SIZE],
            wram: [0; WRAM_SIZE], 
            hram: [0; HRAM_SIZE],
        }
    }

    pub(super) fn read_byte(&self, address: Address) -> u8 {
        match address {
            GAMEROM_0_BEGIN ..= GAMEROM_0_END => {
                match address {
                    BOOT_BEGIN ..= BOOT_END => {
                        if self.is_boot_rom_mapped {
                            self.bootrom.read_byte(address)
                        }else{
                            self.cartridge.read_byte(address)
                        }
                    },
                    _ => self.cartridge.read_byte(address)
                }
            },
            GAMEROM_N_BEGIN ..= GAMEROM_N_END => self.cartridge.read_byte(address),
            VRAM_BEGIN ..= VRAM_END => self.read_vram(address),
            EXTRAM_BEGIN ..= EXTRAM_END => self.read_eram(address),
            WRAM_BEGIN ..= WRAM_END => self.read_wram(address),
            ERAM_BEGIN ..= ERAM_END => panic!("prohibited read 0x{:x} to echo ram", address),
            OAM_BEGIN ..= OAM_END => self.read_oam(address),
            NOTUSABLE_BEGIN ..= NOTUSABLE_END => panic!("prohibited read 0x{:x}", address),
            IO_BEGIN ..= IO_END => self.read_io(address),
            HRAM_BEGIN ..= HRAM_END => self.read_hram(address),
            INTERRUPT_ENABLE_ADDRESS => self.io.interrupts.read_enable(),
            _ => panic!("unmapped read {:x}", address),
        }
    }

    pub(super) fn write_byte(&mut self, address: Address, value: u8) {
        match address {
            GAMEROM_0_BEGIN ..= GAMEROM_0_END => {
                //panic!("Writing in ROM {:x} is not possible", address);
            },
            GAMEROM_N_BEGIN ..= GAMEROM_N_END => {
                //panic!("Writing in ROM {:x} is not possible", address);
            },
            VRAM_BEGIN ..= VRAM_END => self.write_vram(address, value),
            EXTRAM_BEGIN ..= EXTRAM_END => self.write_eram(address, value),
            WRAM_BEGIN ..= WRAM_END => self.write_wram(address, value),
            ERAM_BEGIN ..= ERAM_END => panic!("prohibited write 0x{:x} to echo ram", address),
            OAM_BEGIN ..= OAM_END => self.write_oam(address, value),
            NOTUSABLE_BEGIN ..= NOTUSABLE_END => panic!("prohibited write 0x{:x}", address),
            IO_BEGIN ..= IO_END => self.write_io(address, value),
            HRAM_BEGIN ..= HRAM_END => self.write_hram(address, value),
            INTERRUPT_ENABLE_ADDRESS => self.io.interrupts.write_enable(value),
            _ => panic!("unmapped write {:x}", address),
        };
    }

    fn read_vram(&self, address: Address) -> u8 {
        self.ppu.read_vram(address - VRAM_BEGIN)
    }

    fn read_wram(&self, address: Address) -> u8 {
        self.wram[address as usize - WRAM_BEGIN as usize]
    }

    fn read_eram(&self, address: Address) -> u8 {
        self.eram[address as usize - EXTRAM_BEGIN as usize]
    }

    fn read_oam(&self, address: Address) -> u8 {
        self.oam[address as usize - OAM_BEGIN as usize]
    }

    fn read_hram(&self, address: Address) -> u8 {
        self.hram[address as usize - HRAM_BEGIN as usize]
    }

    fn read_io(&self, address: Address) -> u8 {
        self.io.read_byte(address)     
    }

    fn write_vram(&mut self, address: Address, value: u8) {
        self.ppu.write_vram(address - VRAM_BEGIN, value);
    }

    fn write_wram(&mut self, address: Address, value: u8) {
        self.wram[address as usize - WRAM_BEGIN as usize] = value;
    }

    fn write_eram(&mut self, address: Address, value: u8) {
        self.eram[address as usize - EXTRAM_BEGIN as usize] = value;
    }

    fn write_oam(&mut self, address: Address, value: u8) {
        self.oam[address as usize - OAM_BEGIN as usize] = value;
    }

    fn write_hram(&mut self, address: Address, value: u8) {
        self.hram[address as usize - HRAM_BEGIN as usize] = value;
    }

    fn write_io(&mut self, address: Address, value: u8) {
        let result: Option<IOEvent> = self.io.write_byte(address, value);
        match result {
            Some(IOEvent::BootSwitched(new_val)) => {
                self.is_boot_rom_mapped = new_val;
            },
            _ => {}
        }
    }

}

impl fmt::Display for MMU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //write!(f, "{}", self.bootrom)?;
        //write!(f, "{}", "\n")?;
        write!(f, "{} {:x}-{:x}\n", "VRAM", VRAM_BEGIN, VRAM_END)?;
        write!(f, "{}", self.ppu)?;
        write!(f, "{}", "\n\n")?;

        write!(f, "{} {:x}-{:x}\n", "WRAM", WRAM_BEGIN, WRAM_END)?;
        write!(f, "{}", pretty_hex(&self.wram))?;
        write!(f, "{}", "\n\n")?;

        write!(f, "{} {:x}-{:x}\n", "HRAM", HRAM_BEGIN, HRAM_END)?;
        write!(f, "{}", pretty_hex(&self.hram))?;
        write!(f, "{}", "\n\n")
    }
}