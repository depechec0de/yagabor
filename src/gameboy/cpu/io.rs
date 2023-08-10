use crate::gameboy::cpu::cpu::Address;

pub(super) const IO_BEGIN: Address = 0xFF00;
pub(super) const IO_END: Address = 0xFF7F;
const IO_SIZE: usize = (IO_END - IO_BEGIN + 1) as usize;

const BOOT_SWITCH: Address = 0xFF50;

const LCD_CONTROL_BEGIN: Address = 0xFF40;
const LCD_CONTROL_END: Address = 0xFF4B;

pub(crate) struct IO {
    data: [u8; IO_SIZE],
}

pub(super) enum IOEvent {
    BootSwitched(bool)
}

impl IO {
    pub(crate) fn new() -> IO {
        let mut io = IO{ data:[0; IO_SIZE] };
        // TODO: Handle vertical line period properly
        io.data[(0xFF44 - IO_BEGIN) as usize] = 0x90;

        io
    }

    pub(super) fn read_byte(&self, address: Address) -> u8 {
        self.data[(address - IO_BEGIN) as usize]
    }

    pub(super) fn write_byte(&mut self, address: Address, value: u8) -> Option<IOEvent> {
        self.data[(address - IO_BEGIN) as usize] = value;

        match address {
            BOOT_SWITCH => Some(IOEvent::BootSwitched(value == 0)),
            _ => None
        }
    }
}