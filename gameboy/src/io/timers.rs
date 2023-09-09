use crate::gameboy::GameBoy;

use super::{io::IO, interrupts::{Interruption, Interrupts}};

pub(crate) struct Timers {
    pub(super) div_counter: u8,
    // Because the max frecuency is 1024 => 10 bits
    pub(super) tima_counter: u16, 
}

impl Timers {
    pub(crate) fn new() -> Self {
        Timers { div_counter: 0, tima_counter: 0 }
    }
    
    pub(crate) fn tick(gb: &mut GameBoy, cycles: u8) {

        let (new_div, div_overflow) = gb.io.timers.div_counter.overflowing_add(cycles);

        if div_overflow {
            gb.io.timers.div_counter = new_div;
            IO::inc_div(gb);
        }

        if Timers::timer_enabled(gb) {
            gb.io.timers.tima_counter = gb.io.timers.tima_counter.wrapping_add(cycles as u16);

            while gb.io.timers.tima_counter >= Timers::timer_frecuency(gb) {
                let tima_overflow = IO::inc_tima(gb);
                if tima_overflow {
                    Interrupts::turnon(gb, Interruption::Timer);
                    IO::reset_tima(gb);
                }
                gb.io.timers.tima_counter -= Timers::timer_frecuency(gb);
            }
        }
    }
    
    pub(crate) fn timer_enabled(gb: &GameBoy) -> bool {
        // if bit 2 is high, timer is enabled 
        IO::get_tac_register(gb) & 0b00000100 > 0
    }

    pub(crate) fn timer_frecuency(gb: &GameBoy) -> u16 {
        let tac = IO::get_tac_register(gb);
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
}