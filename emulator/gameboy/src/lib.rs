pub mod cartridge;
pub(crate) mod io;
pub(crate) mod gameboy;
mod ppu;
mod rom;
mod cpu;
mod mmu;

use std::io::Error;

use cartridge::Cartridge;
use gameboy::GameBoy;
use io::{interrupts::{Interruption, Interrupts}, joypad::Joypad};
use wasm_bindgen::prelude::*;

pub const SCREEN_WIDTH: u32 = 160;
pub const SCREEN_HEIGHT: u32 = 144;

pub const BACKGROUND_WIDTH: u32 = 256;
pub const BACKGROUND_HEIGHT: u32 = 256;

pub const TILEDATA_WIDTH: u32 = 128;
pub const TILEDATA_HEIGHT: u32 = 192;

pub const CPU_CLOCK_HZ: usize = 4_194_304;
pub const FPS: f32 = 59.7;
pub const CPU_CYCLES_PER_FRAME: usize = (CPU_CLOCK_HZ as f32 / FPS) as usize;

pub struct EmulationStep {
    pub framebuffer: GameBoyFrame,
    pub tiledata: GameBoyFrame,
    pub background: GameBoyFrame,
}

#[wasm_bindgen]
pub enum Button {
    Up, Down, Left, Right, Start, Select, A, B
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColoredPixel {
  White = 0, 
  LightGray = 1,
  DarkGray = 2, 
  Black = 3
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameBoyFrame {
  pub width: u32,
  pub height: u32,
  pub buffer: Vec<ColoredPixel>
}


pub struct Emulation {
  pub(crate) gameboy: GameBoy,
  pub running: bool,
  pub total_cycles: u64
}

#[wasm_bindgen]
pub struct EmulationWasm {
  pub(crate) gameboy: GameBoy,
  pub(crate) screenbuffer: Vec<ColoredPixel>,
  pub total_cycles: u64
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

#[wasm_bindgen]
impl EmulationWasm {
  pub fn new() -> Self {
    let gameboy = GameBoy::new(None);
    let screenbuffer: Vec<ColoredPixel> = Vec::new();
    EmulationWasm { 
        gameboy,
        screenbuffer,
        total_cycles: 0
    }
  }

  pub fn screen(&self) -> *const ColoredPixel {
		self.screenbuffer.as_ptr()
	} 

  pub fn step(&mut self) -> Result<JsValue,JsValue> {

    let mut frame_cycles = 0;           
    
    while frame_cycles < CPU_CYCLES_PER_FRAME {
        let gb_step_res = self.gameboy.tick();

        match gb_step_res {
            Ok(cycles) => {
                let executed_cycles = u64::from(cycles);
                frame_cycles += executed_cycles as usize;
                self.total_cycles += executed_cycles;
            },
            Err(_) => {
                return Err(JsValue::from_str("error"))
            }
        }
    }

    self.screenbuffer = self.gameboy.frame().buffer.clone();

    Ok(JsValue::from_str(&self.total_cycles.to_string()))  
  }

  pub fn button_pressed(&mut self, b: Button) {
      Joypad::button_pressed(&mut self.gameboy, b);
      Interrupts::turnon(&mut self.gameboy, Interruption::Joypad);
  } 

  pub fn button_released(&mut self, b: Button) {
      Joypad::button_released(&mut self.gameboy, b);
  }
}