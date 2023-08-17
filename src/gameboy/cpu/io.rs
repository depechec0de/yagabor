use core::fmt;
use std::rc::Rc;

use crate::gameboy::{cpu::cpu::Address, serial::Serializable};

use pretty_hex::*;

pub(super) const IO_BEGIN: Address = 0xFF00;
pub(super) const IO_END: Address = 0xFF7F;
const IO_SIZE: usize = (IO_END - IO_BEGIN + 1) as usize;

const BOOT_SWITCH_ADDRESS: Address = 0xFF50;
const SERIAL_DATA_ADDRESS: Address = 0xFF01;

const LCD_CONTROL_BEGIN: Address = 0xFF40;
const LCD_CONTROL_END: Address = 0xFF4B;

pub(crate) struct IO<S: Serializable> {
    data: [u8; IO_SIZE],
    serial: Rc<S>,
}

pub(super) enum IOEvent {
    BootSwitched(bool)
}

impl<S: Serializable> IO<S> {
    pub(crate) fn new(serial: Rc<S>) -> Self {
        Self{ data:[0; IO_SIZE], serial }
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
                self.serial.send(value); 
                None
            },
            BOOT_SWITCH_ADDRESS => Some(IOEvent::BootSwitched(value == 0)),
            _ => None
        }
    }
}

impl<S: Serializable> fmt::Display for IO<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:x}-{:x}\n", "IO", IO_BEGIN, IO_END)?;
        write!(f, "{}", pretty_hex(&self.data))
    }
}