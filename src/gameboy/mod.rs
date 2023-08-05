
pub(crate) mod cartridge;
mod cpu;
mod rom;
mod screen;
mod keypad;

use self::cartridge::Cartridge;
use self::cpu::CPU;
use self::keypad::KeyPad;
use self::rom::ROM;
use self::screen::Screen;

pub struct GameBoy {
    cpu: CPU,
    bootrom: ROM,
    cartridge: Cartridge,
    screen: Screen,
    controller: KeyPad
}

impl GameBoy {
    pub fn new(c: Cartridge) -> GameBoy {
        let brom = ROM::dmg();

        GameBoy { cpu: CPU::new(), cartridge: c, bootrom: brom, screen: Screen::new(), controller: KeyPad::new() }
    }

    pub fn start(&self) {
        println!("Rust Game Boy emulator started with game {}", self.cartridge.title());
    }

    pub fn step(&mut self) {
    }
}