mod cpu;
mod gameboy;

use crate::gameboy::GameBoy;

fn main() {
    println!("Starting Game Boy emulator.");

    let gb = GameBoy::new();
}
