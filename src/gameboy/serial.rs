pub trait Serializable {
    fn send(&self, value: u8);
}

pub(crate) struct DummySerialPort {
    
}

impl DummySerialPort {
    pub(crate) fn new() -> DummySerialPort {
        DummySerialPort {}
    }   
}

impl Serializable for DummySerialPort {
    fn send(&self, value: u8) {

    }
}