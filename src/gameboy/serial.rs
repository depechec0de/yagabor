pub(crate) struct SerialOutput {
    send_buffer: Vec<u8>
}

impl SerialOutput {
    pub(crate) fn new() -> Self {
        Self { send_buffer: Vec::new() }
    }  

    pub(crate) fn send(&self, value: u8) {
        
    }

    
}
