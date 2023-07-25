mod cartridge;
mod cpu;
mod gameboy;

use clap::Parser;

use crate::gameboy::GameBoy;
use crate::cartridge::Cartridge;

#[derive(Parser)]
struct Cli {
    /// The path to the cartridge file
    cartridge: std::path::PathBuf,
}

fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();

    let cartridge = Cartridge::new(args.cartridge)?;  

    let gb = GameBoy::new(cartridge);

    gb.start();

    Ok(())
}
