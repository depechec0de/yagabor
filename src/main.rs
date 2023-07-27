mod cartridge;
mod cpu;
mod gameboy;
mod rom;

use std::io::{Error, ErrorKind};

use clap::Parser;
use rom::ROM;

use crate::gameboy::GameBoy;
use crate::cartridge::Cartridge;

#[derive(clap::ValueEnum, Clone, Debug)]
enum BootROMSource {
    Empty,
    DMG,
    File
}


#[derive(Parser)]
struct Cli {
    cartridge: std::path::PathBuf,
    bootsource: BootROMSource,
    bootfile: Option<std::path::PathBuf>
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    let cartridge = Cartridge::new(args.cartridge)?;
    let brom;

    match args.bootsource {
        BootROMSource::File => { 
            match args.bootfile {
                Some(path) => { brom = ROM::from_file(path)?; }
                _ => { return Err(Error::new(ErrorKind::Other,"No boot ROM file provided.")) }
            }
        },
        BootROMSource::Empty => { brom = ROM::empty(); },
        BootROMSource::DMG => { brom = ROM::dmg(); },            
    }

    let gb = GameBoy::new(brom, cartridge);

    gb.start();

    Ok(())
}
