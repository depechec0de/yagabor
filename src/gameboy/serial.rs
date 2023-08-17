pub trait Serializable {
    fn send(&mut self, value: u8);
}

pub(crate) struct DummySerialPort {
}

impl DummySerialPort {
    pub(crate) fn new() -> Self {
        Self {}
    }  
}

impl Serializable for DummySerialPort {
    fn send(&mut self, value: u8) {
    }
}