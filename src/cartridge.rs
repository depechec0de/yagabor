use std::path::PathBuf;
pub struct Cartridge {
    data: Vec<u8>,
}

impl Cartridge {
    pub fn new(file: PathBuf) -> Result<Cartridge, std::io::Error> {

        let buffer = std::fs::read(file)?;

        Ok(Cartridge { data: buffer })
    }

    pub fn get_title(&self) -> String {
        let start: usize = 0x0134;
        let end: usize = 0x0143;
        
        std::str::from_utf8(&self.data[start..end]).expect("invalid utf-8 sequence").to_string()
    } 
}