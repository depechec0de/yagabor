use std::io::Error;

use super::cartridge::Cartridge;
use super::cpu::cpu::{CPU, ExecResult, MachineCycles};
use super::io::io::IO;
use super::io::timers::Timers;
use super::mmu::MMU;
use super::serial::SerialOutput;

pub(crate) struct GameBoy {
    cpu: CPU,
    timers: Timers,
    serialout: SerialOutput
}

impl GameBoy {
    pub(crate) fn new(cartridge: Cartridge, serialout: SerialOutput) -> Self {
        let io = IO::new();
        let mmu = MMU::new(cartridge, io);
        let cpu = CPU::new(mmu);
        let timers = Timers::new();

        GameBoy { cpu, timers, serialout }
    }

    pub(crate) fn tick(&mut self) -> Result<ExecResult, Error> {

        let intmcycles = self.cpu.handle_interrupts() as MachineCycles;

        let mut result = self.cpu.step()?;

        result.mcycles += intmcycles;

        Ok(result)
    }

    pub(crate) fn joypad_down(&mut self) {
        
    }

    
}

