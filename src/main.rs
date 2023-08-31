mod emulation;
mod gameboy;
mod screen;
mod debug;

use std::time::{SystemTime, UNIX_EPOCH};
use std::{io::Error, time::{Duration, Instant}};

use clap::Parser;
use emulation::Emulation;
use sdl2::{pixels::{Color, PixelFormatEnum}, event::{Event, EventWatchCallback}, keyboard::Keycode, video::WindowPos, rect::{Point, Rect}};

use crate::debug::TileDataFrame;
use crate::{gameboy::{cartridge::Cartridge, gameboy::GameBoy, io::lcd::SCREEN_WIDTH}, emulation::EmulationReport, screen::{Screen}, debug::{TileDataDebug, TILEDATA_WIDTH, TILEDATA_HEIGHT, TILEDATA_COLS, TILEDATA_ROWS}};

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

    println!("Loading cartridge {} with type {:?}", cartridge.title(), cartridge.ctype());

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
    let video = sdl_context.video().unwrap();
    //let mut screen = Screen::new(&video);
    let mut debug = TileDataDebug::new(&video);
    
    let mut execution_time = Duration::from_secs(0);

    emu.start();

    'running: loop {

        if emu.running {
            let now = Instant::now();
            // Emulation step
            match emu.step() {
                Ok(emustep) => {
                    //screen.render(emustep.framebuffer);
                    debug.clear();
                    debug.render(emustep.tiledata);
                    debug.present();                  
                },
                Err(error) => {
                    break 'running println!("Emulation terminated in {} seconds,\
                                            total executed cycles: {} with error {:?}", 
                                            execution_time.as_secs_f32(), 
                                            emu.total_cycles, 
                                            error);
                }
            }

            std::thread::sleep(Duration::from_millis(1000/60));
            
            

            let elapsed = now.elapsed();
            execution_time += elapsed;
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

    println!("Emulation terminated normally in {} seconds, total executed cycles: {}", execution_time.as_secs_f32() , emu.total_cycles);
    
    Ok(())
    
}
