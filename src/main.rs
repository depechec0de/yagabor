mod emulation;
mod gameboy;

use std::io::Error;

use clap::Parser;
use emulation::Emulation;

use crate::gameboy::{cartridge::Cartridge, gameboy::GameBoy, serial::SerialOutput};

#[derive(Parser)]
struct Cli {
    cartridge: Option<std::path::PathBuf>,
    debug: Option<bool>
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();
    let cartridge;

    if let Some(c) = args.cartridge {
        cartridge = Cartridge::new(c)?;
    }else{
        cartridge = Cartridge::empty();
    }

    let debug: bool;
    if let Some(val) = args.debug {
        debug = val;
    }else{
        debug = false;
    }

    let soutput = SerialOutput::new();
    let gb: GameBoy = GameBoy::new(cartridge, soutput);

    let mut emu = Emulation::new(gb, debug);

    let report = emu.run();
    //println!("Emulation terminated in {} seconds, total executed cyles: {} with result {:?}", report.execution_time.as_secs_f32() , report.total_cycles, report.result);
    
    Ok(())
    
}
