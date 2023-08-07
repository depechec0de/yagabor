use crate::gameboy::{ppu::*, rom::*, cartridge::Cartridge};

const MEM_SIZE: usize = 0xFFFF;

pub(crate) struct MMU {
    memory: [u8; MEM_SIZE],
    is_boot_rom_mapped: bool,
    bootrom: ROM,
    cartridge: Cartridge
}

impl MMU {
    pub fn new(bootrom: ROM, cartridge: Cartridge) -> MMU {
        let data = [0; MEM_SIZE];

        MMU { 
            memory: data, 
            is_boot_rom_mapped: true, 
            cartridge,
            bootrom
        }
    }

    pub(super) fn read_byte(&self, address: u16) -> u8 {
        let address = address as usize;
        match address {
            BOOT_BEGIN ..= BOOT_END => {
                if self.is_boot_rom_mapped {
                    self.bootrom.read_byte(address)
                }else{
                    self.cartridge.read_byte(address)
                }
            },
            // VRAM_BEGIN ..= VRAM_END => {
            //     self.ppu.read_vram(address - VRAM_BEGIN)
            // },
            _ => self.cartridge.read_byte(address)
        }
    }

    pub(super) fn write_byte(&mut self, address: u16, value: u8) {
        let address = address as usize;
        match address {
            VRAM_BEGIN ..= VRAM_END => {
                //self.ppu.write_vram(address - VRAM_BEGIN, value)
            },
            addr => self.memory[addr as usize] = value
        }
    }
}