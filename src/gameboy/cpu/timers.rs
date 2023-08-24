use crate::gameboy::{cpu::{cpu::{MachineCycles, ClockCycles}}, io::interrupts::Interruption};

#[derive(Debug)]
pub(crate) struct Timers {
    pub(super) div: u16,
    pub(super) tima: u8,
    pub(super) tma: u8,
    pub(super) tac: u8,
    pub(super) div_counter: u16,
    pub(super) counter: u16
}

impl Timers {
    pub(super) fn new() -> Self {
        Self { 
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            div_counter: 0,
            counter: 0
        }
    }

    pub(crate) fn is_enabled(&self) -> bool {
        // if bit 2 is high, timer is enabled 
        self.tac & 0b00000100 > 0
    }

    pub(crate) fn get_frecuency(&self) -> u16 {
        if self.tac & 0b00000011 == 0 {
            1024
        }else if self.tac & 0b00000011 == 1 {
            16
        }else if self.tac & 0b00000011 == 2 {
            64
        }else {
            256
        }
    }
}
