use crate::cpu::CPU;
use crate::cartridge::Cartridge;

pub struct GameBoy {
    cpu: CPU,
    cartridge: Cartridge
}

impl GameBoy {
    pub fn new(c: Cartridge) -> GameBoy {
        GameBoy { cpu: CPU::new(), cartridge: c }
    }

    pub fn start(&self) {
        println!("Rust Game Boy emulator started with game {}", self.cartridge.get_title());
    }
}