use crate::{gameboy::GameBoy, clock::Clock};

pub const CPU_CLOCK_HZ: usize = 4194304;
pub const FPS: f32 = 59.7;

pub struct Emulation {
    running: bool,
    gb: GameBoy,
    clock: Clock,
    executed_cycles: u64
}

impl Emulation {
    pub fn new(gameboy: GameBoy) -> Emulation {
        Emulation { running: false, gb: gameboy, clock: Clock::new(CPU_CLOCK_HZ), executed_cycles: 0 }
    }

    pub fn run(&mut self){
        self.running = true;

        'running: loop {
            if let Ok(cycles_passed) = self.gb.step() {
                self.executed_cycles = self.executed_cycles + u64::from(cycles_passed);
            }else{
                println!("Emulation terminated in {} cycles", self.executed_cycles);
                break
            }
        }

    }
}