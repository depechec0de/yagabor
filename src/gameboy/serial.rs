pub trait SerialPort {
    fn receive(&mut self, data: u8);
}

#[derive(Debug)]
pub(crate) enum SerialTransferMode {
    NoTransfer,
    TransferExternalClock,
    TransferInternalClock
}

impl SerialTransferMode {
    pub(crate) fn parse_from_byte(byte: u8) -> SerialTransferMode {
        if byte == 0x81 {
            SerialTransferMode::TransferInternalClock
        }else if byte == 0x80 {
            SerialTransferMode::TransferExternalClock
        }else{
            SerialTransferMode::NoTransfer
        }
    }
}

pub(crate) struct DummySerial {

}

impl SerialPort for DummySerial {
    fn receive(&mut self, data: u8) {
    }
}