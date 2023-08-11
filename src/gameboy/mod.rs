pub mod cartridge;
mod ppu;
mod rom;
mod cpu;

use self::{cartridge::Cartridge, rom::ROM, cpu::{mmu::MMU, io::IO, cpu::CPU}};
use std::io::{Error, ErrorKind};

pub struct GameBoy {
    cpu: CPU
}

// We use machine cycles for reference, but in the translation we multiply by 4
#[derive(Debug, Clone)]
pub enum ClockCycles {
    One, Two, Three, Four, Five, Six
}

impl GameBoy {
    pub fn new(cartridge: Cartridge) -> GameBoy {
        let bootrom = ROM::dmg();
        let io = IO::new();
        let mut mmu = MMU::new(bootrom, cartridge, io);
        let mut cpu = CPU::new(mmu);

        GameBoy { cpu }
    }

    pub fn step(&mut self) -> Result<ClockCycles, Error> {
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