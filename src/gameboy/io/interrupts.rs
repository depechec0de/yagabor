use crate::gameboy::mmu::Address;

const SERIAL_HANDLER: Address = 0x0040;

pub(crate) enum Interruption {
    Serial,
    VBlank
}

impl Interruption {
    pub(crate) fn handler() -> Address {
        SERIAL_HANDLER
    }
}