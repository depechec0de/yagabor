use core::fmt;

use pretty_hex::*;

use crate::gameboy::{mmu::{Address, IO_SIZE, IO_BEGIN, IO_END}, serial::SerialOutput};

pub(crate) const JOYPAD_INPUT_ADDRESS: Address = 0xFF00;
pub(crate) const SERIAL_DATA_ADDRESS: Address = 0xFF01;
pub(crate) const SERIAL_CONTROL_ADDRESS: Address = 0xFF02;

pub(crate) const LCD_CONTROL_BEGIN: Address = 0xFF40;
pub(crate) const LCD_CONTROL_END: Address = 0xFF4B;
pub(crate) const BOOT_SWITCH_ADDRESS: Address = 0xFF50;

pub(crate) struct IO {
    data: [u8; IO_SIZE]
}

#[derive(Debug)]
pub(crate) enum SerialControl {
    TransferStartInternal,
    TransferStartExternal,
    Undefined
}

pub(crate) enum IOEvent {
    BootSwitched(bool),
    SerialOutput(u8)
}

impl IO {
    pub(crate) fn new() -> Self {
        Self{ data:[0; IO_SIZE] }
    }

    pub(crate) fn read_byte(&self, address: Address) -> u8 {
        match address {
            0xFF44 => 0x90,

            _ => self.data[(address - IO_BEGIN) as usize]
        }
    }

    pub(crate) fn write_byte(&mut self, address: Address, value: u8) -> Option<IOEvent> {
        self.data[(address - IO_BEGIN) as usize] = value;

        match address {
            SERIAL_DATA_ADDRESS => { 
                Some(IOEvent::SerialOutput(value))
            },
            BOOT_SWITCH_ADDRESS => Some(IOEvent::BootSwitched(value == 0)),
            _ => None
        }
    }

    // https://gbdev.io/pandocs/Serial_Data_Transfer_(Link_Cable).html
    fn serial_control(&mut self) -> SerialControl {
        if self.read_byte(SERIAL_CONTROL_ADDRESS) == 0x81 {
            SerialControl::TransferStartInternal
        } else if self.read_byte(SERIAL_CONTROL_ADDRESS) == 0x80 {
            SerialControl::TransferStartExternal
        }else {
            SerialControl::Undefined
        }
    }

    pub(crate) fn read_serial_data(&mut self) -> Option<u8> {
        match self.serial_control() {
            SerialControl::TransferStartInternal | 
            SerialControl::TransferStartExternal => {
                Some(self.read_byte(SERIAL_DATA_ADDRESS))
            },
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