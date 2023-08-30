use std::io::{Error, ErrorKind};
use std::fmt;

use crate::debug::TileDataFrame;

use super::cartridge::Cartridge;
use super::cpu::cpu::{CPU, ClockCycles};
use super::io::interrupts::Interrupts;
use super::io::io::IO;
use super::io::lcd::{LCD, Frame};
use super::mmu::MMU;
use super::ppu::PPU;

pub(crate) struct GameBoy {
    pub(crate) cpu: CPU,
    pub(crate) mmu: MMU,
    pub(crate) ppu: PPU,
    pub(crate) io: IO,
    pub(crate) cartridge: Cartridge
}

pub(crate) struct GBStep {
    pub(crate) cycles: ClockCycles,
    pub(crate) output: GBOutput
}

pub(crate) struct GBOutput {
    pub(crate) serial: Option<u8>
}

impl GameBoy {
    pub(crate) fn new(cartridge: Cartridge) -> Self {
        let io = IO::new();
        let mmu = MMU::new();
        let cpu = CPU::new();
        let ppu = PPU::new();

        GameBoy { cpu, mmu, ppu, io, cartridge }
    }
    
    pub(crate) fn tick(&mut self) -> Result<GBStep, Error> {
        let mut output = GBOutput{ serial: None };
        let cycles = CPU::step(self)? as ClockCycles;

        if let Some(data) = CPU::send_serial(self){
            output.serial = Some(data);
            IO::ack_sent_serial(self);
        }

        LCD::tick(self, cycles);

        Ok(GBStep{cycles,output})
    }

    pub(crate) fn frame(&mut self) -> Frame {
        LCD::read_framebuffer(self)
    }

    pub(crate) fn tiledata(&mut self) -> TileDataFrame {
        LCD::read_tiledata(self)
    }

    pub(crate) fn joypad_down(&mut self) {
        
    }    
}

impl fmt::Display for GameBoy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A:{:02X} \
            F:{:02X} \
            B:{:02X} \
            C:{:02X} \
            D:{:02X} \
            E:{:02X} \
            H:{:02X} \
            L:{:02X} \
            SP:{:04X} \
            PC:{:04X} \
            PCMEM:{:02X},{:02X},{:02X},{:02X}",  
            self.cpu.regs.a, 
            u8::from(self.cpu.regs.flags.clone()), 
            self.cpu.regs.b, 
            self.cpu.regs.c, 
            self.cpu.regs.d, 
            self.cpu.regs.e, 
            self.cpu.regs.h, 
            self.cpu.regs.l, 
            self.cpu.sp, 
            self.cpu.pc,
            MMU::read_byte(self, self.cpu.pc), 
            MMU::read_byte(self,self.cpu.pc+1), 
            MMU::read_byte(self,self.cpu.pc+2), 
            MMU::read_byte(self,self.cpu.pc+3)
            )
    }
}