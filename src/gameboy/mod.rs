pub(super) mod cartridge;
mod ppu;
mod rom;
mod cpu;
pub(super) mod serial;

use self::{cartridge::Cartridge, rom::ROM, cpu::{mmu::MMU, io::IO, cpu::CPU}, ppu::PPU, serial::{DummySerialPort, Serializable}};
use std::io::{Error, ErrorKind};

pub struct GameBoy<S: Serializable> {
    cpu: CPU<S>,
    cycles_passed: u64,
    cycles_executed: u64
}

// We use machine cycles for reference, but in the translation we multiply by 4
#[derive(Debug, Clone)]
pub enum ClockCycles {
    One, Two, Three, Four, Five, Six
}

impl<S: Serializable> GameBoy<S> {
    pub fn new(cartridge: Cartridge, serial: S) -> Self {
        let mmu = MMU::new(cartridge, serial);
        let cpu = CPU::new(mmu);

        GameBoy { cpu, cycles_passed: 0, cycles_executed: 0 }
    }

    pub fn tick(&mut self) -> Result<ClockCycles, Error> {
        self.cpu.step()
    }
}

impl std::convert::From<ClockCycles> for u64  {
    fn from(cycles: ClockCycles) -> u64 {
        let machine_cycles = match cycles {
            ClockCycles::One => 1,
            ClockCycles::Two => 2,
            ClockCycles::Three => 3,
            ClockCycles::Four => 4,
            ClockCycles::Five => 5,
            ClockCycles::Six => 6
        };
        machine_cycles*4
    }
}