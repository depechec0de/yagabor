use std::{time::{Duration, Instant}, io::Error};

use crate::{gameboy::{gameboy::GameBoy, io::lcd::{Frame}}, debug::TileDataFrame};

pub const CPU_CLOCK_HZ: usize = 4_194_304;
pub const FPS: f32 = 59.7;
pub const CPU_CYCLES_PER_FRAME: usize = (CPU_CLOCK_HZ as f32 / FPS) as usize;


pub struct Emulation {
    gameboy: GameBoy,
    pub(crate) running: bool,
    pub(crate) total_cycles: u64,
    debug: bool
}

#[derive(Debug)]
pub(crate) struct EmulationReport {
    pub(crate) execution_time: Duration,
    pub(crate) total_cycles: u64,
    pub(crate) result: Result<(), Error>,
}

pub(crate) struct EmulationStep {
    pub(crate) framebuffer: Frame,
    pub(crate) tiledata: TileDataFrame,
}

impl Emulation {
    pub(crate) fn new(gameboy: GameBoy, debug: bool) -> Self {
        Emulation { 
            gameboy,
            running: false,
            total_cycles: 0,
            debug
        }
    }

    pub(crate) fn start(&mut self) {
        self.running = true;
    }

    pub(crate) fn step(&mut self) -> Result<EmulationStep,Error> {

        let mut frame_cycles = 0;           
        
        while frame_cycles < CPU_CYCLES_PER_FRAME {
            let gb_step_res = self.gameboy.tick();

            match gb_step_res {
                Ok(gb_step) => {
                    let executed_cycles = u64::from(gb_step.cycles);
                    frame_cycles += executed_cycles as usize;
                    self.total_cycles += executed_cycles;
                    
                },
                Err(error) => {
                    return Err(error)
                }
            }
        }

        let framebuffer = self.gameboy.frame();
        let tiledata = self.gameboy.tiledata();
        Ok(EmulationStep { framebuffer, tiledata })  
    }
}