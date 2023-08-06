mod emulation;
mod gameboy;

use std::io::Error;

use clap::Parser;
use emulation::Emulation;
use gameboy::GameBoy;

use crate::gameboy::cartridge::Cartridge;

#[derive(Parser)]
struct Cli {
    cartridge: std::path::PathBuf
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    let cartridge = Cartridge::new(args.cartridge)?;
    let mut gb: GameBoy = GameBoy::new(cartridge);

    let mut emu = Emulation::new(gb);

    emu.run();

    Ok(())
}
