use std::time::Duration;

use sdl2::{event::Event, keyboard::Keycode};

use crate::gameboy::GameBoy;

pub struct Emulation {
    ticks: u64,
    running: bool,
    gb: GameBoy
}

impl Emulation {
    pub fn new(gameboy: GameBoy) -> Emulation {
        Emulation { ticks: 0, running: false, gb: gameboy }
    }

    pub fn run(&mut self){
        self.running = true;

        'running: loop {
            self.gb.step();
            self.ticks += 1;

            // Input events
            // for event in self.event_pump.poll_iter() {
            //     match event {
            //         Event::Quit {..} |
            //         Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
            //             break 'running
            //         },
            //         _ => {}
            //     }
            // }
            

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

    }
}