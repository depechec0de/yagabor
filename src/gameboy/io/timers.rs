use crate::gameboy::cpu::cpu::ClockCycles;

use super::interrupts::Interruption;

pub(crate) struct Timers {
    div: u8,
    tima: u8,
    tma: u8,
    tac: u8
}

impl Timers {
    pub(crate) fn new() -> Self {
        Self { 
            div: 0x0,
            tima: 0x0,
            tma: 0x0,
            tac: 0x0
        }
    }

    pub(crate) fn move_timers(&mut self, cycles: ClockCycles) -> Option<Interruption> {
        None
    }
}
