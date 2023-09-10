use std::io::Error;

use crate::cartridge::Cartridge;
use crate::{Button, GameBoyFrame};
use crate::gameboy::GameBoy;
use crate::io::interrupts::{Interrupts, Interruption};
use crate::io::joypad::Joypad;

pub const CPU_CLOCK_HZ: usize = 4_194_304;
pub const FPS: f32 = 59.7;
pub const CPU_CYCLES_PER_FRAME: usize = (CPU_CLOCK_HZ as f32 / FPS) as usize;


pub struct Emulation {
    pub(crate) gameboy: GameBoy,
    pub running: bool,
    pub total_cycles: u64
}

pub struct EmulationStep {
    pub framebuffer: GameBoyFrame,
    pub tiledata: GameBoyFrame,
    pub background: GameBoyFrame,
}

impl Emulation {
    pub fn new(cartridge: Option<Cartridge>) -> Self {
        let gameboy = GameBoy::new(cartridge);
        Emulation { 
            gameboy,
            running: false,
            total_cycles: 0
        }
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn step(&mut self) -> Result<EmulationStep,Error> {

        let mut frame_cycles = 0;           
        
        while frame_cycles < CPU_CYCLES_PER_FRAME {
            let gb_step_res = self.gameboy.tick();

            match gb_step_res {
                Ok(cycles) => {
                    let executed_cycles = u64::from(cycles);
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
        let background = self.gameboy.background();

        Ok(EmulationStep { framebuffer, tiledata, background })  
    }

    pub fn button_pressed(&mut self, b: Button) {
        Joypad::button_pressed(&mut self.gameboy, b);
        Interrupts::turnon(&mut self.gameboy, Interruption::Joypad);
    } 

    pub fn button_released(&mut self, b: Button) {
        Joypad::button_released(&mut self.gameboy, b);
    }
}