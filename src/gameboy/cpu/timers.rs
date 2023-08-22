use crate::gameboy::{cpu::{cpu::{MachineCycles, ClockCycles}}, io::interrupts::Interruption};

pub(crate) struct Timers {
    pub(super) div: u16,
    pub(super) tima: u8,
    pub(super) tma: u8,
    pub(super) tac: u8,
    pub(super) clocks_counter: u16
}

pub(crate) enum TimerFired {
    DIV,
    TIMA,
    DIVTIMA
}

impl Timers {
    pub(super) fn new() -> Self {
        Self { 
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            clocks_counter: 0
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

    pub(crate) fn set_tma(&mut self, value: u8) {
        self.tma = value;
    }

    pub(crate) fn handle_timers(&mut self, cycles: ClockCycles) -> Option<TimerFired> {
        let mut div_fired = false;
        let mut tima_fired = false;

        self.div += cycles as u16;

        if self.div >= 256 {
            self.div -= 256;
            div_fired = true; 
        }

        if self.is_enabled() {
            self.clocks_counter += cycles as u16;

            while self.clocks_counter >= self.get_frecuency() {
                let (_, tima_overflow) = self.tima.overflowing_add(1);
                if tima_overflow {
                    tima_fired = true;
                    self.tima = self.tma;
                }
                self.clocks_counter -= self.get_frecuency();
            }
        }
        
        if div_fired && tima_fired {
            Some(TimerFired::DIVTIMA)
        } else if div_fired {
            Some(TimerFired::DIV)
        } else if tima_fired {
            Some(TimerFired::TIMA)
        }else{
            None
        }
    }
}
