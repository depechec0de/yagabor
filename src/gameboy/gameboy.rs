use std::io::Error;

use super::cartridge::Cartridge;
use super::cpu::cpu::{CPU, ExecResult, ClockCycles};
use super::io::io::IO;
use super::mmu::MMU;
use super::serial::SerialOutput;

pub(crate) struct GameBoy {
    cpu: CPU,
    serialout: Option<SerialOutput>
}

impl GameBoy {
    pub(crate) fn new(cartridge: Cartridge, serialout: Option<SerialOutput>) -> Self {
        let io = IO::new();
        let mmu = MMU::new(cartridge, io);
        let cpu = CPU::new(mmu);

        GameBoy { cpu, serialout }
    }

    pub(crate) fn tick(&mut self) -> Result<ExecResult, Error> {
        let mut cycles_consumed: ClockCycles = 0;

        let cycles_consumed = self.cpu.step()? as ClockCycles;

        self.cpu.timer_tick(cycles_consumed);

        let external_event = self.cpu.output_event();

        Ok(ExecResult{ event: external_event , clockcycles: cycles_consumed })
    }

    pub(crate) fn joypad_down(&mut self) {
        
    }

    
}

