mod emulation;
mod gameboy;
mod screen;

use std::{io::Error, time::{Duration, Instant}};

use clap::Parser;
use emulation::Emulation;
use sdl2::{event::Event, keyboard::Keycode};

use crate::gameboy::io::joypad::Button;
use crate::gameboy::io::lcd::{SCREEN_HEIGHT, TILEDATA_WIDTH, BACKGROUND_WIDTH, TILEDATA_HEIGHT, BACKGROUND_HEIGHT};
use crate::{gameboy::{cartridge::Cartridge, gameboy::GameBoy, io::lcd::SCREEN_WIDTH}, screen::Screen};

#[derive(Parser)]
struct Cli {
    cartridge: Option<std::path::PathBuf>
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();
    
    let cartridge: Option<Cartridge>;

    if let Some(c) = args.cartridge {
        cartridge = Some(Cartridge::new(c)?);
        println!("Loading cartridge {} with type {:?}", 
                cartridge.as_ref().unwrap().title(), 
                cartridge.as_ref().unwrap().ctype());
    }else {
        cartridge = None;
    }
    
    // let gui: bool;
    // if let Some(val) = args.gui {
    //     gui = val;
    // }else{
    //     gui = false;
    // }
    
    let gb: GameBoy = GameBoy::new(cartridge);

    let mut emu = Emulation::new(gb);

    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Interaction with hosting machine: screen, keyboard input, ...    
    let video = sdl_context.video().unwrap();

    let mut screen = Screen::new(&video, "Game Boy", SCREEN_WIDTH, SCREEN_HEIGHT, 4, 0);    
    let mut tddebug = Screen::new(&video, "Tile data", TILEDATA_WIDTH, TILEDATA_HEIGHT, 2, 500);
    let mut bgdebug = Screen::new(&video, "Background", BACKGROUND_WIDTH, BACKGROUND_HEIGHT, 2, 900);
    
    let mut execution_time = Duration::from_secs(0);
    let mut displayed_frames = 0;

    emu.start();

    'running: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'running,
                Event::KeyDown { keycode, .. } => {
                    match keycode {
                        Some(Keycode::Escape)   => break 'running,
                        Some(Keycode::A)        => emu.gameboy.button_pressed(Button::A),
                        Some(Keycode::S)        => emu.gameboy.button_pressed(Button::B),
                        Some(Keycode::Return)   => emu.gameboy.button_pressed(Button::Start),
                        Some(Keycode::Space)    => emu.gameboy.button_pressed(Button::Select),
                        Some(Keycode::Up)       => emu.gameboy.button_pressed(Button::Up),
                        Some(Keycode::Down)     => emu.gameboy.button_pressed(Button::Down),
                        Some(Keycode::Left)     => emu.gameboy.button_pressed(Button::Left),
                        Some(Keycode::Right)    => emu.gameboy.button_pressed(Button::Right),
                        _                       => {},
                    }
                    
                },
                Event::KeyUp { keycode, .. } => {
                    match keycode {
                        Some(Keycode::A)        => emu.gameboy.button_released(Button::A),
                        Some(Keycode::S)        => emu.gameboy.button_released(Button::B),
                        Some(Keycode::Return)   => emu.gameboy.button_released(Button::Start),
                        Some(Keycode::Space)    => emu.gameboy.button_released(Button::Select),
                        Some(Keycode::Up)       => emu.gameboy.button_released(Button::Up),
                        Some(Keycode::Down)     => emu.gameboy.button_released(Button::Down),
                        Some(Keycode::Left)     => emu.gameboy.button_released(Button::Left),
                        Some(Keycode::Right)    => emu.gameboy.button_released(Button::Right),
                        _                       => {},
                    }
                    
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
            displayed_frames += 1;
        }
    
    }

    println!("Emulation terminated normally in {} seconds, total executed cycles: {} and {} frames", execution_time.as_secs_f32() , emu.total_cycles, displayed_frames );
    
    Ok(())
}
