use self::{screen::Screen, keypad::Joypad, cartridge::Cartridge, rom::ROM, cpu::{mmu::MMU, CPU}};

pub mod cartridge;
mod ppu;
mod rom;
mod keypad;
mod cpu;
mod screen;

pub struct GameBoy {
    cpu: CPU
}

impl GameBoy {
    pub fn new(cartridge: Cartridge) -> GameBoy {
        let bootrom = ROM::dmg();
        let mut mmu = MMU::new(bootrom, cartridge);
        let mut cpu = CPU::new(mmu);

        GameBoy { 
                cpu
        }
    }

    pub fn step(&mut self) {
        self.cpu.step();
    }
}