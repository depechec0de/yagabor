mod emulation;
mod gameboy;
mod clock;

use std::io::Error;
use std::rc::Rc;

use clap::Parser;
use emulation::Emulation;

use crate::gameboy::{cartridge::Cartridge, serial::DummySerialPort, gameboy::GameBoy};

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

    let serial: DummySerialPort = DummySerialPort::new();

    let gb: GameBoy<_> = GameBoy::new(cartridge, Rc::new(&serial));

    let mut emu = Emulation::new(gb);

    let report = emu.run();
    println!("Emulation terminated in {} seconds, total executed cyles: {} with result {:?}", report.execution_time.as_secs_f32() , report.total_cycles, report.result);
    
    Ok(())
    
}
