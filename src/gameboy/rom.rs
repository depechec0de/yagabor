use std::path::PathBuf;

pub(super) const BOOT_BEGIN: usize = 0x0000;
pub(super) const BOOT_END: usize = 0x0100;

// 16-bit address ROM
pub struct ROM {
    data: [u8; 0xFFFF]
}

impl ROM {
    pub fn empty() -> ROM {
        ROM { data: [0; 0xFFFF] }
    }

    pub fn new(buffer: Vec<u8>) -> ROM {
        let mut d = [0; 0xFFFF];

        for addr in 0..0xFFFF {
            if (addr as usize) < buffer.len() {
                d[addr] = buffer[addr];
            }else{
                d[addr] = 0x0;
            }
        }
        ROM { data: d }
    }

    pub fn from_file(file: PathBuf) -> Result<ROM, std::io::Error> {
        let buffer = std::fs::read(file)?;

        Ok(ROM::new(buffer))
    }

    pub fn dmg() -> ROM {
        let data = include_bytes!("../../assets/dmgrom.bin");
        let buffer = data.to_vec();

        ROM::new(buffer)
    }

    pub fn size(&self) -> u16 {
        self.data.len() as u16
    }

    pub fn read_byte(&self, address: usize) -> u8 {
        self.data[address].clone()
    }

}