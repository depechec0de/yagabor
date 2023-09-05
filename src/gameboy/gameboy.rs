use std::io::Error;
use std::fmt;

use super::cartridge::Cartridge;
use super::cpu::cpu::{CPU, ClockCycles};
use super::io::interrupts::{Interrupts, Interruption};
use super::io::io::IO;
use super::io::joypad::{Button, Joypad};
use super::io::lcd::{LCD, Frame};
use super::mmu::MMU;
use super::ppu::PPU;

pub(crate) struct GameBoy {
    pub(crate) cpu: CPU,
    pub(crate) mmu: MMU,
    pub(crate) ppu: PPU,
    pub(crate) io: IO,
    pub(crate) cartridge: Option<Cartridge>,
    pub(crate) serial: Option<u8>
}

impl GameBoy {
    pub(crate) fn new(cartridge: Option<Cartridge>) -> Self {
        let io = IO::new();
        let mmu = MMU::new();
        let cpu = CPU::new();
        let ppu = PPU::new();

        GameBoy { cpu, mmu, ppu, io, cartridge, serial: None }
    }
    
    pub(crate) fn tick(&mut self) -> Result<ClockCycles, Error> {
        let cycles = CPU::step(self)? as ClockCycles;

        // if self.cpu.pc == 0x100 {
        //     return Err(Error::new(ErrorKind::Other, "test"));
        // }
        
        if let Some(data) = CPU::send_serial(self){
            self.serial = Some(data);
            IO::ack_sent_serial(self);
        }else{
            self.serial = None;
        }

        LCD::tick(self, cycles);

        Ok(cycles)
    }

    pub(crate) fn read_serial(&self) -> Option<u8> {
        self.serial
    }

    pub(crate) fn frame(&self) -> Frame {
        LCD::screen_buffer(self)
    }

    pub(crate) fn tiledata(&self) -> Frame {
        LCD::tiledata_buffer(self)
    }

    pub(crate) fn background(&self) -> Frame {
        LCD::background_buffer(self)
    }

    pub(crate) fn button_pressed(&mut self, b: Button) {
        Joypad::button_pressed(self, b);
        Interrupts::turnon(self, Interruption::Joypad);
    } 

    pub(crate) fn button_released(&mut self, b: Button) {
        Joypad::button_released(self, b);
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