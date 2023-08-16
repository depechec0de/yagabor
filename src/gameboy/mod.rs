pub(super) mod cartridge;
mod ppu;
mod rom;
mod cpu;
pub(super) mod serial;

use self::{cartridge::Cartridge, rom::ROM, cpu::{mmu::MMU, io::IO, cpu::{CPU, ClockCycles}}, ppu::PPU, serial::{TestSerialPort, Serializable}};
use std::io::{Error, ErrorKind};

pub(crate) struct GameBoy<S: Serializable> {
    cpu: CPU<S>,
    cycles_passed: u64,
    cycles_executed: u64
}

impl<S: Serializable> GameBoy<S> {
    pub(crate) fn new(cartridge: Cartridge, serial: S) -> Self {
        let mmu = MMU::new(cartridge, serial);
        let cpu = CPU::new(mmu);

        GameBoy { cpu, cycles_passed: 0, cycles_executed: 0 }
    }

    pub(crate) fn tick(&mut self) -> Result<ClockCycles, Error> {
        self.cpu.step()
    }
}

