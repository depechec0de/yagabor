mod emulation;
mod gameboy;
mod screen;

use std::time::{SystemTime, UNIX_EPOCH};
use std::{io::Error, time::{Duration, Instant}};

use clap::Parser;
use emulation::Emulation;
use sdl2::{pixels::{Color, PixelFormatEnum}, event::{Event, EventWatchCallback}, keyboard::Keycode, video::WindowPos, rect::{Point, Rect}};

use crate::gameboy::io::lcd::{SCREEN_HEIGHT, TILEDATA_WIDTH, BACKGROUND_WIDTH, TILEDATA_HEIGHT, BACKGROUND_HEIGHT};
use crate::{gameboy::{cartridge::Cartridge, gameboy::GameBoy, io::lcd::SCREEN_WIDTH}, emulation::EmulationReport, screen::{Screen}};

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
    let scale = 2;

    let mut screen = Screen::new(&video, "Game Boy", SCREEN_WIDTH, SCREEN_HEIGHT, scale, 0);    
    let mut tddebug = Screen::new(&video, "Tile data", TILEDATA_WIDTH, TILEDATA_HEIGHT, scale, 500);
    let mut bgdebug = Screen::new(&video, "Background", BACKGROUND_WIDTH, BACKGROUND_HEIGHT, scale, 900);
    
    let mut execution_time = Duration::from_secs(0);

    emu.start();

    'running: loop {

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

        if emu.running {
            let now = Instant::now();
            // Emulation step
            match emu.step() {
                Ok(emustep) => {
                    screen.render(emustep.framebuffer);
                    tddebug.render(emustep.tiledata);  
                    bgdebug.render(emustep.background);            
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
    
    }

    println!("Emulation terminated normally in {} seconds, total executed cycles: {}", execution_time.as_secs_f32() , emu.total_cycles);
    
    Ok(())
    
}
