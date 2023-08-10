mod emulation;
mod gameboy;
mod clock;

use std::io::Error;

use clap::Parser;
use emulation::Emulation;
use gameboy::GameBoy;

use crate::gameboy::cartridge::Cartridge;

#[derive(Parser)]
struct Cli {
    cartridge: Option<std::path::PathBuf>
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();
    let cartridge;

    if let Some(c) = args.cartridge {
        cartridge = Cartridge::new(c)?;
    }else{
        cartridge = Cartridge::empty();
    }
    let gb: GameBoy = GameBoy::new(cartridge);

    let mut emu = Emulation::new(gb);

    emu.run();

    Ok(())
}
