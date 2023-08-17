use std::io::Error;
use std::rc::Rc;

use super::cartridge::Cartridge;
use super::cpu::cpu::{CPU, ClockCycles};
use super::cpu::mmu::MMU;
use super::serial::Serializable;

pub(crate) struct GameBoy<S: Serializable> {
    cpu: CPU<S>,
    cycles_passed: u64,
    cycles_executed: u64
}

impl<S: Serializable> GameBoy<S> {
    pub(crate) fn new(cartridge: Cartridge, serial: Rc<S>) -> Self {
        let mmu = MMU::new(cartridge, serial);
        let cpu = CPU::new(mmu);

        GameBoy { cpu, cycles_passed: 0, cycles_executed: 0 }
    }

    pub(crate) fn tick(&mut self) -> Result<ClockCycles, Error> {
        self.cpu.step()
    }
}

