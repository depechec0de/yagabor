use std::{time::{Duration, Instant}, io::Error};

use sdl2::{Sdl, event::Event, keyboard::Keycode, pixels::Color, rect::Point};

use crate::gameboy::gameboy::GameBoy;

pub const CPU_CLOCK_HZ: usize = 4_194_304;
pub const FPS: f32 = 59.7;
pub const CPU_CYCLES_PER_FRAME: usize = (CPU_CLOCK_HZ as f32 / FPS) as usize;


pub struct Emulation {
    running: bool,
    gameboy: GameBoy,
    debug: bool,
    sdl_context: Sdl
}

#[derive(Debug)]
pub(crate) struct EmulationReport {
    pub(crate) execution_time: Duration,
    pub(crate) total_cycles: u64,
    pub(crate) result: Result<(), Error>,
}

impl Emulation {
    pub(crate) fn new(gameboy: GameBoy, debug: bool) -> Self {

        let sdl_context = sdl2::init().unwrap();

        Emulation { 
            running: false, 
            gameboy,
            debug,
            sdl_context
        }
    }

    pub(crate) fn run(&mut self) -> EmulationReport {
        self.running = true;

        let video_subsystem = self.sdl_context.video().unwrap();

        let window = video_subsystem.window("Game Boy", 160, 144)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let mut i = 0;
        //let mut rng = rand::thread_rng();

        let mut event_pump = self.sdl_context.event_pump().unwrap();

        let mut total_cycles: u64 = 0;
        let mut execution_time = Duration::from_secs(0);

        let mut now = Instant::now();

        'running: loop {
            i = (i + 1) % 255;
            canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
            let (w, h) = canvas.output_size().unwrap();
            let mut points = [Point::new(0, 0); 256];
            //points.fill_with(|| Point::new(rng.gen_range(0..w as i32), rng.gen_range(0..h as i32)));
            // For performance, it's probably better to draw a whole bunch of points at once
            canvas.draw_points(points.as_slice()).unwrap();
            canvas.present();

            let mut frame_cycles: usize = 0;
            
            while frame_cycles < CPU_CYCLES_PER_FRAME {
                let gbstepres = self.gameboy.tick();

                let mut executed_cycles: u64 = 0;

                match gbstepres {
                    Ok(gbstep) => {
                        executed_cycles += u64::from(gbstep.cycles);
                        frame_cycles += executed_cycles as usize;
                        total_cycles += executed_cycles;
                    },
                    Err(error) => {
                        break 'running EmulationReport { execution_time, total_cycles, result: Err(error) }
                    }
                }
            }

            std::thread::sleep(Duration::from_millis(1000/60));
            
            let elapsed = now.elapsed();
            execution_time += elapsed;           

            now = Instant::now();

    
            // canvas.clear();

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running EmulationReport { execution_time, total_cycles, result: Ok(()) }
                    },
                    Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                        self.gameboy.joypad_down()
                    },
                    _ => {}
                }
            }
            
        }
    }
}