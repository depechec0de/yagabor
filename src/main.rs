mod emulation;
mod gameboy;
mod screen;

use std::io::Error;

use clap::Parser;
use emulation::Emulation;
use sdl2::{pixels::Color, event::{Event, EventWatchCallback}, keyboard::Keycode};

use crate::{gameboy::{cartridge::Cartridge, gameboy::GameBoy}, emulation::EmulationReport, screen::{Screen}};

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

    // let gui: bool;
    // if let Some(val) = args.gui {
    //     gui = val;
    // }else{
    //     gui = false;
    // }

    let gb: GameBoy = GameBoy::new(cartridge);

    let mut emu = Emulation::new(gb, false);

    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Interaction with hosting machine: screen, keyboard input, ...    
    let video_subsystem = sdl_context.video().unwrap();
    let mut screen = Screen::new(video_subsystem);
    

    emu.start();

    'running: loop {
        // Emulation step
        match emu.step() {
            Ok(emustep) => {
                screen.render(emustep.framebuffer);
            },
            Err(error) => {
                break 'running println!("Emulation terminated in {} seconds,\
                                         total executed cycles: {} with error {:?}", 
                                         emu.execution_time.as_secs_f32(), 
                                         emu.total_cycles, 
                                         error);
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    //gb.joypad_down()
                },
                _ => {}
            }
        }
    }

    println!("Emulation terminated normally in {} seconds, total executed cycles: {}", emu.execution_time.as_secs_f32() , emu.total_cycles);
    
    Ok(())
    
}
