use std::path::PathBuf;

const HEADER_BEGIN: usize = 0x0100;
const HEADER_END: usize = 0x014F;

const CARTRIDGE_SIZE: usize = HEADER_END+1;

const ENTRY_START_ADDR: usize = 0x0100;
const ENTRY_END_ADDR: usize = 0x0103;
const ENTRY_SIZE: usize = ENTRY_END_ADDR-ENTRY_START_ADDR;

const LOGO_START_ADDR: usize = 0x0104;
const LOGO_END_ADDR: usize = 0x0133;
const LOGO_SIZE: usize = LOGO_END_ADDR-LOGO_START_ADDR;

const TITLE_START_ADDR: usize = 0x0134;
const TITLE_END_ADDR: usize = 0x0143;

const LICENSEE_ADDR: usize = 0x0144;


pub struct Cartridge {
    data: Vec<u8>,
    title: String
}

impl Cartridge {
    pub fn new(file: PathBuf) -> Result<Cartridge, std::io::Error> {
        let data = std::fs::read(file)?;       
        let title = parse_title(&data);

        Ok(Cartridge { data, title })
    }   

    pub fn empty() -> Cartridge {
        Cartridge { data: vec![0; CARTRIDGE_SIZE], title: "empty".to_string() }
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    pub fn cpu_instrs_01() -> Cartridge {
        let buffer = include_bytes!("../../assets/gb-test-roms/cpu_instrs/individual/01-special.gb");
        let data = buffer.to_vec();
        let title = parse_title(&data);
    
        Cartridge{ data, title }
    }
}

fn parse_title(buffer: &Vec<u8>) -> String {
    let start = TITLE_START_ADDR;
    let end = TITLE_END_ADDR;

    std::str::from_utf8(&buffer[start..end])
              .expect("invalid utf-8 sequence")
              .to_string()
}