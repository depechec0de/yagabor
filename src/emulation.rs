use std::time::Duration;

use crate::gameboy::GameBoy;

pub const CPU_CLOCK_HZ: usize = 4194304;
pub const FPS: f32 = 60.0;

pub struct Emulation {
    running: bool,
    gb: GameBoy
}

impl Emulation {
    pub fn new(gameboy: GameBoy) -> Emulation {
        Emulation { running: false, gb: gameboy }
    }

    pub fn run(&mut self){
        self.running = true;

        'running: loop {
            self.gb.step();

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
            

            //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

    }
}