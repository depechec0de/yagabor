use std::io::Error;

use super::cartridge::Cartridge;
use super::cpu::cpu::{CPU, ClockCycles};
use super::io::io::IO;
use super::mmu::MMU;

pub(crate) struct GameBoy {
    cpu: CPU
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
        let mmu = MMU::new(cartridge, io);
        let cpu = CPU::new(mmu);

        GameBoy { cpu }
    }
    
    pub(crate) fn tick(&mut self) -> Result<GBStep, Error> {
        let mut output = GBOutput{ serial: None };
        let cycles = self.cpu.step()? as ClockCycles;

        if let Some(data) = self.cpu.send_serial(){
            output.serial = Some(data);
            self.cpu.ack_sent_serial();
        }

        Ok(GBStep{cycles,output})
    }

    pub(crate) fn joypad_down(&mut self) {
        
    }

    
}

