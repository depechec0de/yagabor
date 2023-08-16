pub trait Serializable {
    fn send(&mut self, value: u8);
}

pub(crate) struct TestSerialPort {
    pub(crate) sent_data: String
}

impl TestSerialPort {
    pub(crate) fn new() -> TestSerialPort {
        TestSerialPort { sent_data: String::new() }
    }   

    pub(crate) fn test_finished(&self) -> Option<bool> {
        if self.sent_data.contains("Passed") {
            Some(true)
        } else if self.sent_data.contains("Failed") {
            Some(false)
        }else{
            None
        }
    }
}

impl Serializable for TestSerialPort {
    fn send(&mut self, value: u8) {
        self.sent_data.push(value as char);
    }
}