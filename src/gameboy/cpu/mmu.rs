use crate::gameboy::{ppu::*, rom::*, cartridge::Cartridge};

use super::{io::*, Address};

const MEM_SIZE: usize = 0xFFFF;

const ROM_BEGIN: u16 = 0x0000;
const ROM_END: u16 = 0x7FFF;

const GAMEROM0_BEGIN: u16 = 0x0000;
const GAMEROM0_END: u16 = 0x3FFF;
const GAMEROM0_SIZE: usize = (GAMEROM0_END - GAMEROM0_BEGIN + 1) as usize;

const VRAM_BEGIN: u16 = 0x8000;
const VRAM_END: u16 = 0x9FFF;
const VRAM_SIZE: usize = (VRAM_END - VRAM_BEGIN + 1) as usize;

const WRAM_BEGIN: u16 = 0xC000;
const WRAM_END: u16 = 0xDFFF;
const WRAM_SIZE: usize = (WRAM_END - WRAM_BEGIN + 1) as usize;

const ERAM_BEGIN: u16 = 0xE000;
const ERAM_END: u16 = 0xFDFF;

const NOTUSABLE_BEGIN: u16 = 0xFEA0;
const NOTUSABLE_END: u16 = 0xFEFF;

const HRAM_BEGIN: u16 = 0xFF80;
const HRAM_END: u16 = 0xFFFE;
const HRAM_SIZE: usize = (HRAM_END - HRAM_BEGIN + 1) as usize;

pub(crate) struct MMU {
    is_boot_rom_mapped: bool,
    bootrom: ROM,
    cartridge: Cartridge,
    io: IO,
    vram: [u8; VRAM_SIZE],
    wram: [u8; WRAM_SIZE],
    hram: [u8; HRAM_SIZE],
}

impl MMU {
    pub fn new(bootrom: ROM, cartridge: Cartridge, io: IO) -> MMU {
        MMU { 
            is_boot_rom_mapped: true, 
            cartridge,
            bootrom,
            io,
            vram: [0; VRAM_SIZE], 
            wram: [0; WRAM_SIZE], 
            hram: [0; HRAM_SIZE],
        }
    }

    pub(super) fn read_byte(&self, address: Address) -> u8 {
        match address {
            GAMEROM0_BEGIN ..= GAMEROM0_END => {
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
            VRAM_BEGIN ..= VRAM_END => self.read_vram(address),
            WRAM_BEGIN ..= WRAM_END => self.read_wram(address),
            ERAM_BEGIN ..= ERAM_END => panic!("prohibited read 0x{:x} to echo ram", address),
            NOTUSABLE_BEGIN ..= NOTUSABLE_END => panic!("prohibited read 0x{:x}", address),
            IO_BEGIN ..= IO_END => self.read_io(address),
            HRAM_BEGIN ..= HRAM_END => self.read_hram(address),
            _ => panic!("unmapped read {:x}", address),
        }
    }

    pub(super) fn write_byte(&mut self, address: Address, value: u8) {
        match address {
            GAMEROM0_BEGIN ..= GAMEROM0_END => panic!("Writing in ROM is not possible"),
            VRAM_BEGIN ..= VRAM_END => self.write_vram(address, value),
            WRAM_BEGIN ..= WRAM_END => self.write_wram(address, value),
            ERAM_BEGIN ..= ERAM_END => panic!("prohibited write 0x{:x} to echo ram", address),
            NOTUSABLE_BEGIN ..= NOTUSABLE_END => panic!("prohibited write 0x{:x}", address),
            IO_BEGIN ..= IO_END => self.write_io(address, value),
            HRAM_BEGIN ..= HRAM_END => self.write_hram(address, value),
            _ => panic!("unmapped write {:x}", address),
        }
    }

    fn read_vram(&self, address: Address) -> u8 {
        self.vram[address as usize - VRAM_BEGIN as usize]
    }

    fn read_wram(&self, address: Address) -> u8 {
        self.wram[address as usize - WRAM_BEGIN as usize]
    }

    fn read_io(&self, address: Address) -> u8 {
        self.io.read_byte(address)     
    }

    fn read_hram(&self, address: Address) -> u8 {
        self.hram[address as usize - HRAM_BEGIN as usize]
    }

    fn write_vram(&mut self, address: Address, value: u8) {
        self.vram[address as usize - VRAM_BEGIN as usize] = value;
    }

    fn write_wram(&mut self, address: Address, value: u8) {
        self.wram[address as usize - WRAM_BEGIN as usize] = value;
    }

    fn write_io(&mut self, address: Address, value: u8) {
        let result: Option<IOEvent> = self.io.write_byte(address, value);
        
        match result {
            Some(IOEvent::BootSwitched(new_val)) => self.is_boot_rom_mapped = new_val,
            None => {}
        }
    }

    fn write_hram(&mut self, address: Address, value: u8) {
        self.hram[address as usize - HRAM_BEGIN as usize] = value;
    }

}