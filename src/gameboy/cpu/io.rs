use core::fmt;

use crate::gameboy::cpu::cpu::Address;

use pretty_hex::*;

use super::mmu::{IO_END, IO_BEGIN, IO_SIZE};

const JOYPAD_INPUT_ADDRESS: Address = 0xFF00;
const SERIAL_DATA_ADDRESS: Address = 0xFF01;
const SERIAL_CONTROL_ADDRESS: Address = 0xFF02;

const LCD_CONTROL_BEGIN: Address = 0xFF40;
const LCD_CONTROL_END: Address = 0xFF4B;
const BOOT_SWITCH_ADDRESS: Address = 0xFF50;

pub(crate) struct IO {
    data: [u8; IO_SIZE]
}

pub(crate) enum IOEvent {
    BootSwitched(bool),
    SerialOutput(u8)
}

impl IO {
    pub(crate) fn new() -> Self {
        Self{ data:[0; IO_SIZE] }
    }

    pub(super) fn read_byte(&self, address: Address) -> u8 {
        match address {
            0xFF44 => 0x90,
            _ => self.data[(address - IO_BEGIN) as usize]
        }
    }

    pub(super) fn write_byte(&mut self, address: Address, value: u8) -> Option<IOEvent> {
        self.data[(address - IO_BEGIN) as usize] = value;

        match address {
            SERIAL_DATA_ADDRESS => { 
                Some(IOEvent::SerialOutput(value))
            },
            BOOT_SWITCH_ADDRESS => Some(IOEvent::BootSwitched(value == 0)),
            _ => None
        }
    }
}

impl fmt::Display for IO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:x}-{:x}\n", "IO", IO_BEGIN, IO_END)?;
        write!(f, "{}", pretty_hex(&self.data))
    }
}