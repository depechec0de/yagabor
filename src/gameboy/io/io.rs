use core::fmt;

use pretty_hex::*;

use crate::gameboy::{mmu::{Address, IO_SIZE, IO_BEGIN, IO_END}, serial::SerialOutput};

use super::interrupts::{Interruption, InterruptsRegister};

pub(crate) const JOYPAD_INPUT_ADDRESS: Address = 0xFF00;
pub(crate) const SERIAL_DATA_ADDRESS: Address = 0xFF01;
pub(crate) const SERIAL_CONTROL_ADDRESS: Address = 0xFF02;

pub(crate) const LCD_CONTROL_BEGIN: Address = 0xFF40;
pub(crate) const LCD_CONTROL_END: Address = 0xFF4B;
pub(crate) const BOOT_SWITCH_ADDRESS: Address = 0xFF50;

pub(crate) const INTERRUPT_FLAG_ADDRESS: Address = 0xFF0F;

pub(crate) struct IO {
    pub(crate) interrupts: InterruptsRegister,
    data: [u8; IO_SIZE]
}

pub(crate) enum IOEvent {
    BootSwitched(bool),
    SerialOutput(u8),
    Interrupt(Interruption)
}

impl IO {
    pub(crate) fn new() -> Self {
        Self{ interrupts: InterruptsRegister::new(), data:[0; IO_SIZE] }
    }

    pub(crate) fn read_byte(&self, address: Address) -> u8 {
        match address {
            0xFF44 => 0x90,
            INTERRUPT_ENABLE_ADDRESS => self.interrupts.read_byte(address),
            // TODO: Map the rest
            _ => self.data[(address - IO_BEGIN) as usize]
        }
    }

    pub(crate) fn write_byte(&mut self, address: Address, value: u8) -> Option<IOEvent> {
        match address {
            SERIAL_DATA_ADDRESS => { 
                Some(IOEvent::SerialOutput(value))
            },
            BOOT_SWITCH_ADDRESS => Some(IOEvent::BootSwitched(value == 0)),
            // INTERRUPT_ENABLE_ADDRESS => self.interrupts.write_byte(address, value),
            // INTERRUPT_FLAG_ADDRESS => self.interrupts.write_byte(address, value),
            _ => None
        }
    }

}