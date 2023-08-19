pub(crate) enum SerialControl {
    TransferStartInternal,
    TransferStartExternal,
    Undefined
}

pub(crate) struct SerialPort {
}

impl SerialPort {
    pub(crate) fn new() -> Self {
        Self {}
    }  

    pub(crate) fn send(&self, value: u8) {
        
    }
}
