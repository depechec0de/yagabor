use crate::cpu::CPU;

pub struct GameBoy {
    cpu: CPU
}

impl GameBoy {
    pub fn new() -> GameBoy {
        GameBoy { cpu: CPU::new() }
    }
}