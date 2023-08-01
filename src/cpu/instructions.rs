pub enum Instruction {
    NOP,
    HALT,
    STOP,
    // 8-bit arithmetic and logical instructions
    ADD(ArithmeticTarget),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    CP(ArithmeticTarget),
    XOR(ArithmeticTarget),
    INC(IncDecTarget),
    DEC(IncDecTarget),
    // 16-bit Arithmetic/Logic instructions
    INC16(WordRegister),
    DEC16(WordRegister),
    // 8-bit load instructions
    LD(LoadType),
    // Jump instructions
    JP(JumpTest),
    // Prefix instructions
    RLC(PrefixTarget)
}

pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
}

#[derive(Clone)]
pub enum ArithmeticTarget {
    A, B, C, D, E, H, L, HLI, D8
}

pub enum IncDecTarget {
    A, B, C, D, E, H, L, HLI
}

pub enum PrefixTarget {
    A, B, C, D, E, H, L,
}

pub enum WordRegister {
    BC, DE, HL, SP
}

pub enum AFromIndirectSource {
    BC, DE, HLInc, HLDec
}

pub enum LoadByteTarget {
    A, B, C, D, E, H, L, HLI
}
pub enum LoadByteSource {
    A, B, C, D, E, H, L, D8, HLI
}
pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
    Word(WordRegister),
    AFromIndirect(AFromIndirectSource),
    IndirectFromA(WordRegister),
}

impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::RLC(PrefixTarget::B)),
            0x01 => Some(Instruction::RLC(PrefixTarget::C)),
            0x02 => Some(Instruction::RLC(PrefixTarget::D)),
            0x03 => Some(Instruction::RLC(PrefixTarget::E)),
            0x04 => Some(Instruction::RLC(PrefixTarget::H)),
            0x05 => Some(Instruction::RLC(PrefixTarget::L)),
            0x06 => None, // TODO
            0x07 => Some(Instruction::RLC(PrefixTarget::A)),
            _ => /* TODO: Add mapping for rest of instructions */ None
        }
    }
    
    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            // Miscellaneous instructions
            0x00 => Some(Instruction::NOP),
            0x76 => Some(Instruction::HALT),
            0x10 => Some(Instruction::STOP),
            0x27 => todo!(),
            0x37 => todo!(),
            0x2F => todo!(),
            0x3F => todo!(),
            0xF3 => todo!(),
            0xFB => todo!(),

            // Rotate instructions
            0x07 => todo!(),
            0x17 => todo!(),
            0x0F => todo!(),
            0x1F => todo!(),

            // Stack instructions
            0xC1 => todo!(),
            0xD1 => todo!(),
            0xE1 => todo!(),
            0xF1 => todo!(),
            0xC5 => todo!(),
            0xD5 => todo!(),
            0xE5 => todo!(),
            0xF5 => todo!(),

            // Control flow instructions
            0x18 => todo!(),
            0x28 => todo!(),
            0x38 => todo!(),
            0xC0 => todo!(),
            0xD0 => todo!(),
            0xC2 => todo!(),
            0xD2 => todo!(),
            0xC3 => todo!(),
            0xC4 => todo!(),
            0xD4 => todo!(),
            0xC7 => todo!(),
            0xD7 => todo!(),
            0xE7 => todo!(),
            0xF7 => todo!(),
            0xC8 => todo!(),
            0xD8 => todo!(),
            0xC9 => todo!(),
            0xD9 => todo!(),
            0xE9 => todo!(),
            0xCA => todo!(),
            0xDA => todo!(),
            0xCC => todo!(),
            0xDC => todo!(),
            0xCD => todo!(),
            0xCF => todo!(),
            0xDF => todo!(),
            0xEF => todo!(),
            0xFF => todo!(),

            // 16-bit load instructions
            0x01 => todo!(),
            0x08 => todo!(),
            0x11 => todo!(),
            0x21 => todo!(),
            0x31 => todo!(),
            0xF8 => todo!(),
            0xF9 => todo!(),

            // 16-bit Arithmetic/Logic instructions
            0x09 => todo!(),
            0x19 => todo!(),
            0x29 => todo!(),
            0x39 => todo!(),
            0x03 => Some(Instruction::INC16(WordRegister::BC)),
            0x13 => Some(Instruction::INC16(WordRegister::DE)),
            0x23 => Some(Instruction::INC16(WordRegister::HL)),
            0x33 => Some(Instruction::INC16(WordRegister::SP)),
            0x0B => Some(Instruction::DEC16(WordRegister::BC)),
            0x1B => Some(Instruction::DEC16(WordRegister::DE)),
            0x2B => Some(Instruction::DEC16(WordRegister::HL)),
            0x3B => Some(Instruction::DEC16(WordRegister::SP)),
            
            // 8-bit load instructions
            0x40 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::B))),
            0x41 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::C))),
            0x42 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D))),
            0x43 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::E))),
            0x44 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::H))),
            0x45 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::L))),
            0x46 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::HLI))),
            0x47 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::A))),
            0x48 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::B))),
            0x49 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::C))),
            0x4A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D))),
            0x4B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::E))),
            0x4C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::H))),
            0x4D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::L))),
            0x4E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::HLI))),
            0x4F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::A))),

            0x50 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::B))),
            0x51 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::C))),
            0x52 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D))),
            0x53 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::E))),
            0x54 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::H))),
            0x55 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::L))),
            0x56 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::HLI))),
            0x57 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::A))),
            0x58 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::B))),
            0x59 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::C))),
            0x5A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D))),
            0x5B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::E))),
            0x5C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::H))),
            0x5D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::L))),
            0x5E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::HLI))),
            0x5F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::A))),

            0x60 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::B))),
            0x61 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::C))),
            0x62 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::D))),
            0x63 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::E))),
            0x64 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::H))),
            0x65 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::L))),
            0x66 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::HLI))),
            0x67 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::A))),
            0x68 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::B))),
            0x69 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::C))),
            0x6A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D))),
            0x6B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::E))),
            0x6C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::H))),
            0x6D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::L))),
            0x6E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::HLI))),
            0x6F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::A))),

            0x70 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::B))),
            0x71 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::C))),
            0x72 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::D))),
            0x73 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::E))),
            0x74 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::H))),
            0x75 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::L))),
            0x77 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::A))),
            0x78 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::B))),
            0x79 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::C))),
            0x7A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D))),
            0x7B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::E))),
            0x7C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::H))),
            0x7D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::L))),
            0x7E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::HLI))),
            0x7F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::A))),

            0x06 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D8))),
            0x16 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D8))),
            0x26 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::D8))),
            0x36 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::D8))),
            0x0E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D8))),
            0x1E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D8))),
            0x2E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D8))),
            0x3E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D8))),

            0x0A => Some(Instruction::LD(LoadType::AFromIndirect(AFromIndirectSource::BC))),
            0x1A => Some(Instruction::LD(LoadType::AFromIndirect(AFromIndirectSource::DE))),
            0x2A => Some(Instruction::LD(LoadType::AFromIndirect(AFromIndirectSource::HLInc))),
            0x3A => Some(Instruction::LD(LoadType::AFromIndirect(AFromIndirectSource::HLDec))),

            // 8-bit arithmetic and logical instructions
            0x04 => Some(Instruction::INC(IncDecTarget::B)),
            0x14 => Some(Instruction::INC(IncDecTarget::D)),
            0x24 => Some(Instruction::INC(IncDecTarget::H)),
            0x34 => Some(Instruction::INC(IncDecTarget::HLI)),

            0x05 => Some(Instruction::DEC(IncDecTarget::B)),
            0x15 => Some(Instruction::DEC(IncDecTarget::D)),
            0x25 => Some(Instruction::DEC(IncDecTarget::H)),
            0x35 => Some(Instruction::DEC(IncDecTarget::HLI)),

            0x0C => Some(Instruction::INC(IncDecTarget::C)),
            0x1C => Some(Instruction::INC(IncDecTarget::E)),
            0x2C => Some(Instruction::INC(IncDecTarget::L)),
            0x3C => Some(Instruction::INC(IncDecTarget::A)),

            0x0D => Some(Instruction::DEC(IncDecTarget::C)),
            0x1D => Some(Instruction::DEC(IncDecTarget::E)),
            0x2D => Some(Instruction::DEC(IncDecTarget::L)),
            0x3D => Some(Instruction::DEC(IncDecTarget::A)),
            
            0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),
            0x86 => Some(Instruction::ADD(ArithmeticTarget::HLI)),
            0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),

            0x88 => Some(Instruction::ADC(ArithmeticTarget::B)),
            0x89 => Some(Instruction::ADC(ArithmeticTarget::C)),
            0x8A => Some(Instruction::ADC(ArithmeticTarget::D)),
            0x8B => Some(Instruction::ADC(ArithmeticTarget::E)),
            0x8C => Some(Instruction::ADC(ArithmeticTarget::H)),
            0x8D => Some(Instruction::ADC(ArithmeticTarget::L)),
            0x8E => Some(Instruction::ADC(ArithmeticTarget::HLI)),
            0x8F => Some(Instruction::ADC(ArithmeticTarget::A)),
            
            0x90 => Some(Instruction::SUB(ArithmeticTarget::B)),
            0x91 => Some(Instruction::SUB(ArithmeticTarget::C)),
            0x92 => Some(Instruction::SUB(ArithmeticTarget::D)),
            0x93 => Some(Instruction::SUB(ArithmeticTarget::E)),
            0x94 => Some(Instruction::SUB(ArithmeticTarget::H)),
            0x95 => Some(Instruction::SUB(ArithmeticTarget::L)),
            0x96 => Some(Instruction::SUB(ArithmeticTarget::HLI)),
            0x97 => Some(Instruction::SUB(ArithmeticTarget::A)),

            0x98 => Some(Instruction::SBC(ArithmeticTarget::B)),
            0x99 => Some(Instruction::SBC(ArithmeticTarget::C)),
            0x9A => Some(Instruction::SBC(ArithmeticTarget::D)),
            0x9B => Some(Instruction::SBC(ArithmeticTarget::E)),
            0x9C => Some(Instruction::SBC(ArithmeticTarget::H)),
            0x9D => Some(Instruction::SBC(ArithmeticTarget::L)),
            0x9E => Some(Instruction::SBC(ArithmeticTarget::HLI)),
            0x9F => Some(Instruction::SBC(ArithmeticTarget::A)),

            0xA0 => Some(Instruction::AND(ArithmeticTarget::B)),
            0xA1 => Some(Instruction::AND(ArithmeticTarget::C)),
            0xA2 => Some(Instruction::AND(ArithmeticTarget::D)),
            0xA3 => Some(Instruction::AND(ArithmeticTarget::E)),
            0xA4 => Some(Instruction::AND(ArithmeticTarget::H)),
            0xA5 => Some(Instruction::AND(ArithmeticTarget::L)),
            0xA6 => Some(Instruction::AND(ArithmeticTarget::HLI)),
            0xA7 => Some(Instruction::AND(ArithmeticTarget::A)),

            0xA8 => Some(Instruction::XOR(ArithmeticTarget::B)),
            0xA9 => Some(Instruction::XOR(ArithmeticTarget::C)),
            0xAA => Some(Instruction::XOR(ArithmeticTarget::D)),
            0xAB => Some(Instruction::XOR(ArithmeticTarget::E)),
            0xAC => Some(Instruction::XOR(ArithmeticTarget::H)),
            0xAD => Some(Instruction::XOR(ArithmeticTarget::L)),
            0xAE => Some(Instruction::XOR(ArithmeticTarget::HLI)),
            0xAF => Some(Instruction::XOR(ArithmeticTarget::A)),

            0xB0 => Some(Instruction::OR(ArithmeticTarget::B)),
            0xB1 => Some(Instruction::OR(ArithmeticTarget::C)),
            0xB2 => Some(Instruction::OR(ArithmeticTarget::D)),
            0xB3 => Some(Instruction::OR(ArithmeticTarget::E)),
            0xB4 => Some(Instruction::OR(ArithmeticTarget::H)),
            0xB5 => Some(Instruction::OR(ArithmeticTarget::L)),
            0xB6 => Some(Instruction::OR(ArithmeticTarget::HLI)),
            0xB7 => Some(Instruction::OR(ArithmeticTarget::A)),

            0xB8 => Some(Instruction::CP(ArithmeticTarget::B)),
            0xB9 => Some(Instruction::CP(ArithmeticTarget::C)),
            0xBA => Some(Instruction::CP(ArithmeticTarget::D)),
            0xBB => Some(Instruction::CP(ArithmeticTarget::E)),
            0xBC => Some(Instruction::CP(ArithmeticTarget::H)),
            0xBD => Some(Instruction::CP(ArithmeticTarget::L)),
            0xBE => Some(Instruction::CP(ArithmeticTarget::HLI)),
            0xBF => Some(Instruction::CP(ArithmeticTarget::A)),

            0xC6 => Some(Instruction::ADD(ArithmeticTarget::D8)),
            0xD6 => Some(Instruction::SUB(ArithmeticTarget::D8)),
            0xE6 => Some(Instruction::AND(ArithmeticTarget::D8)),
            0xF6 => Some(Instruction::OR(ArithmeticTarget::D8)),

            0xCE => Some(Instruction::ADC(ArithmeticTarget::D8)),
            0xDE => Some(Instruction::SBC(ArithmeticTarget::D8)),
            0xEE => Some(Instruction::XOR(ArithmeticTarget::D8)),
            0xFE => Some(Instruction::CP(ArithmeticTarget::D8)),
            _ => None
        }
    }
}