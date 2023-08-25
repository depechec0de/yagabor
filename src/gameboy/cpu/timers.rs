use crate::gameboy::{cpu::{cpu::{MachineCycles, ClockCycles}}, io::interrupts::Interruption};

#[derive(Debug)]
pub(crate) struct Timers {
    pub(super) div_counter: u16,
    pub(super) counter: u16
}

pub(crate) enum TimerEvent {
    Fired{div: u16, tima: u8},
    Tick{div: u16, tima: u8},
}

impl Timers {
    pub(super) fn new() -> Self {
        Self { 
            div_counter: 0,
            counter: 0
        }
    }

    pub(crate) fn tick(&mut self, tac: u8, tma: u8, tima: u8, div: u16, cycles: ClockCycles) -> TimerEvent {
        let timer_fired = false;
        let new_tima: u8;
        let new_div = div;

        self.div_counter += cycles;
        
        if self.div_counter >= 256 {

            self.div_counter -= 256;
            new_div = new_div.wrapping_add(1);
        }

        if timer_enabled(tac) {
            self.counter += cycles;

            while self.counter >= timer_frecuency(tac) {
                let (new_tima, tima_overflow) = new_tima.overflowing_add(1);
                if tima_overflow {
                    timer_fired = true;
                }

                self.counter -= timer_frecuency(tac);
            }
        }
        
        if timer_fired {
            TimerEvent::Fired{ div: new_div, tima: tma }
        }else{
            TimerEvent::Tick{ div: new_div, tima: tma }
        }
    }
}

pub(crate) fn timer_enabled(tac: u8) -> bool {
    // if bit 2 is high, timer is enabled 
    tac & 0b00000100 > 0
}

pub(crate) fn timer_frecuency(tac: u8) -> u16 {
    if tac & 0b00000011 == 0 {
        1024
    }else if tac & 0b00000011 == 1 {
        16
    }else if tac & 0b00000011 == 2 {
        64
    }else {
        256
    }
}