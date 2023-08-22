use core::fmt;

use pretty_hex::*;

use crate::gameboy::{mmu::{Address, IO_SIZE, IO_BEGIN, IO_END}, serial::SerialOutput};

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
    data: [u8; IO_SIZE]
}

#[derive(Debug)]
pub(crate) enum IOEvent {
    BootSwitched(bool),
    SerialOutput(u8),
    Interrupt(Interruption),
    TimerDIV(u8),
    TimerControl(u8),
    TimerTMA(u8),
    TimerTIMA(u8),
}

impl IO {
    pub(crate) fn new() -> Self {
        Self{ interrupts: InterruptsRegister::new(), data:[0; IO_SIZE] }
    }

    pub(crate) fn read_byte(&self, address: Address) -> u8 {
        match address {
            0xFF44 => 0x90,
            INTERRUPT_ENABLE_ADDRESS => self.interrupts.read_enable(),
            INTERRUPT_FLAG_ADDRESS => self.interrupts.read_flag(),
            // TODO: Map the rest
            _ => self.data[(address - IO_BEGIN) as usize]
        }
    }

    pub(crate) fn write_byte(&mut self, address: Address, value: u8) -> Option<IOEvent> {
        match address {
            // Serial
            SERIAL_DATA_ADDRESS => Some(IOEvent::SerialOutput(value)),
            // ROM
            BOOT_SWITCH_ADDRESS => Some(IOEvent::BootSwitched(value == 0)),
            INTERRUPT_ENABLE_ADDRESS => self.interrupts.write_enable(value),
            INTERRUPT_FLAG_ADDRESS => self.interrupts.write_flag(value),
            // Timers
            TAC_ADDRESS => Some(IOEvent::TimerControl(value)),
            TMA_ADDRESS => Some(IOEvent::TimerTMA(value)),
            DIV_ADDRESS => Some(IOEvent::TimerDIV(value)),
            TIMA_ADDRESS => Some(IOEvent::TimerTIMA(value)),
            _ => None
        }
    }

}