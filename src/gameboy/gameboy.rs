use std::io::Error;

use super::cartridge::Cartridge;
use super::cpu::cpu::{CPU, ExecResult};
use super::io::interrupts::Interruption;
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
        let result = self.cpu.step()?;

        let interrupt: Option<Interruption> = self.timers.move_timers(result.cycles.clone());

        if self.cpu.interrupts_enabled() {

        }

        Ok(result)
    }

    pub(crate) fn joypad_down(&mut self) {
        
    }

    
}

