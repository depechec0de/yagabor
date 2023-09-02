use std::path::PathBuf;

use super::gameboy::GameBoy;

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

const CTYPE_ADDR: usize = 0x0147;

pub struct Cartridge {
    data: Vec<u8>,
    title: String,
    ctype: CartridgeType
}

#[derive(Debug, Clone)]
pub enum ROMVersion {
    Empty, Ram, RamBattery
}

#[derive(Debug, Clone)]
pub enum MBCExtras {
    Empty, Ram, RamBattery
}

#[derive(Debug, Clone)]
pub enum MBC2Extras {
    Empty, Battery
}

#[derive(Debug, Clone)]
pub enum MBC3Extras {
    Empty, Ram, RamBattery, TimerBattery, TimerRamBattery
}

#[derive(Debug, Clone)]
pub enum MBC5Extras {
    Empty, Ram, RamBattery, Rumble, RumbleRam, RumbleRamBattery
}

#[derive(Debug, Clone)]
pub enum CartridgeType {
    ROM(ROMVersion), 
    MBC1(MBCExtras), 
    MBC2(MBC2Extras), 
    MMM01(MBCExtras), 
    MBC3(MBC3Extras),
    MBC5(MBC5Extras),
    MBC6,
    MBC7,
    PocketCamera,
    Tama5,
    HuC3,
    HuC1
}

impl Cartridge {
    pub fn new(file: PathBuf) -> Result<Cartridge, std::io::Error> {
        let data = std::fs::read(file)?;       
        let title = parse_title(&data);
        let ctype = CartridgeType::from(data[CTYPE_ADDR]);

        Ok(Cartridge { data, title, ctype })
    }   

    pub fn empty() -> Cartridge {
        // An empty cartridge reads 0xFF
        Cartridge { data: vec![0xFF; CARTRIDGE_SIZE], title: "empty".to_string(), ctype: CartridgeType::ROM(ROMVersion::Empty) }
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn ctype(&self) -> CartridgeType {
        self.ctype.clone()
    }

    pub(crate) fn read_byte(gb: &GameBoy, address: u16) -> u8 {
        gb.cartridge.data[address as usize]
    }

    pub fn cpu_instrs_01() -> Cartridge {
        let buffer = include_bytes!("../../assets/gb-test-roms/cpu_instrs/individual/01-special.gb");
        let data = buffer.to_vec();
        let title = parse_title(&data);
        let ctype = CartridgeType::from(data[CTYPE_ADDR]);
    
        Cartridge{ data, title, ctype }
    }

    pub fn cpu_instrs_02() -> Cartridge {
        let buffer = include_bytes!("../../assets/gb-test-roms/cpu_instrs/individual/02-interrupts.gb");
        let data = buffer.to_vec();
        let title = parse_title(&data);
        let ctype = CartridgeType::from(data[CTYPE_ADDR]);
    
        Cartridge{ data, title, ctype }
    }

    pub fn cpu_instrs_03() -> Cartridge {
        let buffer = include_bytes!("../../assets/gb-test-roms/cpu_instrs/individual/03-op sp,hl.gb");
        let data = buffer.to_vec();
        let title = parse_title(&data);
        let ctype = CartridgeType::from(data[CTYPE_ADDR]);
    
        Cartridge{ data, title, ctype }
    }

    pub fn cpu_instrs_04() -> Cartridge {
        let buffer = include_bytes!("../../assets/gb-test-roms/cpu_instrs/individual/04-op r,imm.gb");
        let data = buffer.to_vec();
        let title = parse_title(&data);
        let ctype = CartridgeType::from(data[CTYPE_ADDR]);
    
        Cartridge{ data, title, ctype }
    }

    pub fn cpu_instrs_05() -> Cartridge {
        let buffer = include_bytes!("../../assets/gb-test-roms/cpu_instrs/individual/05-op rp.gb");
        let data = buffer.to_vec();
        let title = parse_title(&data);
        let ctype = CartridgeType::from(data[CTYPE_ADDR]);
    
        Cartridge{ data, title, ctype }
    }

    pub fn cpu_instrs_06() -> Cartridge {
        let buffer = include_bytes!("../../assets/gb-test-roms/cpu_instrs/individual/06-ld r,r.gb");
        let data = buffer.to_vec();
        let title = parse_title(&data);
        let ctype = CartridgeType::from(data[CTYPE_ADDR]);
    
        Cartridge{ data, title, ctype }
    }

    pub fn cpu_instrs_07() -> Cartridge {
        let buffer = include_bytes!("../../assets/gb-test-roms/cpu_instrs/individual/07-jr,jp,call,ret,rst.gb");
        let data = buffer.to_vec();
        let title = parse_title(&data);
        let ctype = CartridgeType::from(data[CTYPE_ADDR]);
    
        Cartridge{ data, title, ctype }
    }

    pub fn cpu_instrs_08() -> Cartridge {
        let buffer = include_bytes!("../../assets/gb-test-roms/cpu_instrs/individual/08-misc instrs.gb");
        let data = buffer.to_vec();
        let title = parse_title(&data);
        let ctype = CartridgeType::from(data[CTYPE_ADDR]);
    
        Cartridge{ data, title, ctype }
    }

    pub fn cpu_instrs_09() -> Cartridge {
        let buffer = include_bytes!("../../assets/gb-test-roms/cpu_instrs/individual/09-op r,r.gb");
        let data = buffer.to_vec();
        let title = parse_title(&data);
        let ctype = CartridgeType::from(data[CTYPE_ADDR]);
    
        Cartridge{ data, title, ctype }
    }

    pub fn cpu_instrs_10() -> Cartridge {
        let buffer = include_bytes!("../../assets/gb-test-roms/cpu_instrs/individual/10-bit ops.gb");
        let data = buffer.to_vec();
        let title = parse_title(&data);
        let ctype = CartridgeType::from(data[CTYPE_ADDR]);
    
        Cartridge{ data, title, ctype }
    }

    pub fn cpu_instrs_11() -> Cartridge {
        let buffer = include_bytes!("../../assets/gb-test-roms/cpu_instrs/individual/11-op a,(hl).gb");
        let data = buffer.to_vec();
        let title = parse_title(&data);
        let ctype = CartridgeType::from(data[CTYPE_ADDR]);
    
        Cartridge{ data, title, ctype }
    }

    pub fn halt_bug() -> Cartridge {
        let buffer = include_bytes!("../../assets/gb-test-roms/halt_bug.gb");
        let data = buffer.to_vec();
        let title = parse_title(&data);
        let ctype = CartridgeType::from(data[CTYPE_ADDR]);
    
        Cartridge{ data, title, ctype }
    }
}

fn parse_title(buffer: &Vec<u8>) -> String {
    let start = TITLE_START_ADDR;
    let end = TITLE_END_ADDR;

    std::str::from_utf8(&buffer[start..end])
                        .expect("invalid utf-8 sequence")
                        .trim_matches(char::from(0))
                        .to_string()              
}

impl std::convert::From<u8> for CartridgeType {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => CartridgeType::ROM(ROMVersion::Empty),
            0x01 => CartridgeType::MBC1(MBCExtras::Empty),
            0x02 => CartridgeType::MBC1(MBCExtras::Ram),
            0x03 => CartridgeType::MBC1(MBCExtras::RamBattery),
            0x05 => CartridgeType::MBC2(MBC2Extras::Empty),
            0x06 => CartridgeType::MBC2(MBC2Extras::Battery),
            0x08 => CartridgeType::ROM(ROMVersion::Ram),
            0x09 => CartridgeType::ROM(ROMVersion::RamBattery),
            0x0B => CartridgeType::MMM01(MBCExtras::Empty),
            0x0C => CartridgeType::MMM01(MBCExtras::Ram),
            0x0D => CartridgeType::MMM01(MBCExtras::RamBattery),
            0x0F => CartridgeType::MBC3(MBC3Extras::TimerBattery),
            0x10 => CartridgeType::MBC3(MBC3Extras::TimerRamBattery),
            0x11 => CartridgeType::MBC3(MBC3Extras::Empty),
            0x12 => CartridgeType::MBC3(MBC3Extras::Ram),
            0x13 => CartridgeType::MBC3(MBC3Extras::RamBattery),
            0x19 => CartridgeType::MBC5(MBC5Extras::Empty),
            0x1A => CartridgeType::MBC5(MBC5Extras::Ram),
            0x1B => CartridgeType::MBC5(MBC5Extras::RamBattery),
            0x1C => CartridgeType::MBC5(MBC5Extras::Rumble),
            0x1D => CartridgeType::MBC5(MBC5Extras::RumbleRam),
            0x1E => CartridgeType::MBC5(MBC5Extras::RumbleRamBattery),
            0x20 => CartridgeType::MBC6,
            0x22 => CartridgeType::MBC7,
            0xFC => CartridgeType::PocketCamera,
            0xFD => CartridgeType::Tama5,
            0xFE => CartridgeType::HuC3,
            0xFF => CartridgeType::HuC1,
            _ => panic!("Invalid CartridgeType")
        }
    }
}