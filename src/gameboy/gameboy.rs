use std::io::Error;

use super::cartridge::Cartridge;
use super::cpu::cpu::{CPU, ClockCycles};
use super::cpu::mmu::MMU;

pub(crate) struct GameBoy {
    cpu: CPU,
}

pub(crate) struct ExecuteOutput {
    pub(crate) exec_cycles: ClockCycles,
    pub(crate) output: Option<u8>,
}

impl ExecuteOutput {
    pub(crate) fn new(exec_cycles: ClockCycles, output: Option<u8>) -> Self {
        ExecuteOutput { exec_cycles, output }
    }
}

impl GameBoy {
    pub(crate) fn new(cartridge: Cartridge) -> Self {
        let mmu = MMU::new(cartridge);
        let cpu = CPU::new(mmu);

        GameBoy { cpu }
    }

    pub(crate) fn tick(&mut self) -> Result<ExecuteOutput, Error> {
        let cpu_result = self.cpu.step();

        if self.cpu.interrupts_enabled() {

        }
    
        cpu_result
    }

    pub(crate) fn joypad_down(&mut self) {
        
    }

    
}

