use core::fmt;

use pretty_hex::*;

use crate::gameboy::{mmu::{Address, IO_SIZE, IO_BEGIN, IO_END}, cpu::cpu::ClockCycles};

use super::interrupts::{Interruption, InterruptsRegister};

pub(crate) const JOYPAD_INPUT_ADDRESS: Address = 0xFF00;
pub(crate) const SERIAL_DATA_ADDRESS: Address = 0xFF01;
pub(crate) const SERIAL_CONTROL_ADDRESS: Address = 0xFF02;

pub(crate) const DIV_ADDRESS: Address = 0xFF04;
pub(crate) const TIMA_ADDRESS: Address = 0xFF05;
pub(crate) const TMA_ADDRESS: Address = 0xFF06;
pub(crate) const TAC_ADDRESS: Address = 0xFF07;

pub(crate) const LCD_CONTROL_BEGIN: Address = 0xFF40;
pub(crate) const LCD_CONTROL_END: Address = 0xFF4B;
pub(crate) const BOOT_SWITCH_ADDRESS: Address = 0xFF50;

pub(crate) const INTERRUPT_FLAG_ADDRESS: Address = 0xFF0F;
pub(crate) const INTERRUPT_ENABLE_ADDRESS: Address = 0xFFFF;

pub(crate) struct IO {
    pub(crate) interrupts: InterruptsRegister,
    data: [u8; IO_SIZE],
}

#[derive(Debug)]
pub(crate) enum IOEvent {
    BootSwitched(bool),
}

impl IO {
    pub(crate) fn new() -> Self {
        Self{ interrupts: InterruptsRegister::new(), data:[0; IO_SIZE] }
    }

    pub(crate) fn read_byte(&self, address: Address) -> u8 {
        match address {
            0xFF44 => 0x90,
            INTERRUPT_FLAG_ADDRESS => self.interrupts.read_flag(),
            // DIV value is 8 upper bits
            DIV_ADDRESS => self.get_timers_div(),
            _ => self.data[(address - IO_BEGIN) as usize]
        }
    }

    pub(crate) fn write_byte(&mut self, address: Address, value: u8) -> Option<IOEvent> {
        
        match address {
            BOOT_SWITCH_ADDRESS => {
                self.data[(address - IO_BEGIN) as usize] = value;
                Some(IOEvent::BootSwitched(value == 0))
            },
            INTERRUPT_FLAG_ADDRESS => {
                self.interrupts.write_flag(value);
                None
            },
            DIV_ADDRESS => {
                // Writing DIV reset it
                self.data[(DIV_ADDRESS - IO_BEGIN) as usize] = 0;
                None
            },
            _ => {
                self.data[(address - IO_BEGIN) as usize] = value;
                None
            }
        }
    }

    pub(crate) fn serial_control_clear(&mut self) {
        // Turn off bit 7
        self.data[(SERIAL_CONTROL_ADDRESS - IO_BEGIN) as usize] = self.data[(SERIAL_CONTROL_ADDRESS - IO_BEGIN) as usize] & 0b01111111;
    }

    pub(crate) fn get_timers_tac(&self) -> u8 {
        self.data[(TAC_ADDRESS - IO_BEGIN) as usize]
    }

    pub(crate) fn get_timers_div(&self) -> u8 {
        self.data[(DIV_ADDRESS - IO_BEGIN) as usize]
    }

    pub(crate) fn get_timers_tma(&self) -> u8 {
        self.data[(TMA_ADDRESS - IO_BEGIN) as usize]
    }

    pub(crate) fn tick_div(&mut self) {
        let div = self.data[(DIV_ADDRESS - IO_BEGIN) as usize];
        self.data[(DIV_ADDRESS - IO_BEGIN) as usize] = div.wrapping_add(1);
    }

    pub(crate) fn tick_tima(&mut self) -> bool {
        let tima = self.data[(TIMA_ADDRESS - IO_BEGIN) as usize];
        let (new_tima, overflow) = tima.overflowing_add(1);
        self.data[(TIMA_ADDRESS - IO_BEGIN) as usize] = new_tima;
        overflow
    }

    pub(crate) fn reset_tima(&mut self) {
        let tma: u8 = self.get_timers_tma();
        self.data[(TIMA_ADDRESS - IO_BEGIN) as usize] = tma;
    }
    
}