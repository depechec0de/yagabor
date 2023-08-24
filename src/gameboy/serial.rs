pub(crate) struct SerialOutput {
    send_buffer: Vec<u8>
}

#[derive(Debug)]
pub(crate) enum SerialTransferMode {
    NoTransfer,
    TransferExternalClock,
    TransferInternalClock
}
impl SerialTransferMode {
    pub(crate) fn parse_from_byte(byte: u8) -> SerialTransferMode {
        if byte == 81 {
            SerialTransferMode::TransferInternalClock
        }else if byte == 81 {
            SerialTransferMode::TransferExternalClock
        }else{
            SerialTransferMode::NoTransfer
        }
    }
}

impl SerialOutput {
    pub(crate) fn new() -> Self {
        Self { send_buffer: Vec::new() }
    }  

    pub(crate) fn send(&self, value: u8) {
        
    }

    
}
