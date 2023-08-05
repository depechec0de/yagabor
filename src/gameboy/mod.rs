
pub(crate) mod cartridge;
mod cpu;
mod screen;
mod keypad;

use self::cartridge::Cartridge;
use self::cpu::CPU;
use self::keypad::Joypad;
use self::screen::Screen;

pub struct GameBoy {
    cpu: CPU,
    cartridge: Cartridge,
    screen: Screen,
    controller: Joypad
}

impl GameBoy {
    pub fn new(c: Cartridge) -> GameBoy {
        GameBoy { cpu: CPU::new(), cartridge: c, screen: Screen::new(), controller: Joypad::new() }
    }

    pub fn start(&self) {
        println!("Rust Game Boy emulator started with game {}", self.cartridge.title());
    }

    pub fn step(&mut self) {
        self.cpu.step();
    }
}