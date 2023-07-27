use crate::cpu::CPU;
use crate::rom::ROM;
use crate::cartridge::Cartridge;

pub struct GameBoy {
    cpu: CPU,
    cartridge: Cartridge
}

impl GameBoy {
    pub fn new(b: ROM, c: Cartridge) -> GameBoy {
        GameBoy { cpu: CPU::new(b), cartridge: c }
    }

    pub fn start(&self) {
        println!("Rust Game Boy emulator started with game {}", self.cartridge.title());
    }
}