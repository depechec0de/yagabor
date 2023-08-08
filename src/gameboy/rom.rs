use std::path::PathBuf;

pub(super) const BOOT_BEGIN: u16 = 0x0000;
pub(super) const BOOT_END: u16 = 0x00FF;
pub(super) const BOOT_SIZE: usize = (BOOT_END - BOOT_BEGIN + 1) as usize;

// 16-bit address ROM
pub struct ROM {
    data: [u8; BOOT_SIZE]
}

impl ROM {
    pub fn empty() -> ROM {
        ROM { data: [0; BOOT_SIZE] }
    }

    pub fn new(buffer: Vec<u8>) -> ROM {
        let mut d = [0; BOOT_SIZE];

        for addr in 0..BOOT_SIZE {
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

    pub fn read_byte(&self, address: u16) -> u8 {
        self.data[address as usize].clone()
    }

}