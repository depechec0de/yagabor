use std::io::Error;

#[derive(Debug, Clone)]
pub(crate) enum InstructionSize {
    OneByte,
    TwoBytes,
    ThreeBytes
}

impl std::convert::From<InstructionSize> for u16  {
    fn from(instsize: InstructionSize) -> u16 {
        match instsize {
            InstructionSize::OneByte => 1,
            InstructionSize::TwoBytes => 2,
            InstructionSize::ThreeBytes => 3,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Instruction {
    NOP,
    HALT,
    STOP,
    SCF,
    DAA,
    CCF,
    CPL,
    // 8-bit arithmetic and logical instructions
    ADD(RegistersIndDir),
    ADC(RegistersIndDir),
    SUB(RegistersIndDir),
    SBC(RegistersIndDir),
    AND(RegistersIndDir),
    OR(RegistersIndDir),
    CP(RegistersIndDir),
    XOR(RegistersIndDir),
    INC(RegistersIndirect),
    DEC(RegistersIndirect),
    ADDSPS8,
    // 16-bit Arithmetic/Logic instructions
    ADD16(WordRegister),
    INC16(WordRegister),
    DEC16(WordRegister),
    // 8-bit load instructions
    LD(LoadType),
    LDHLSPD8,
    LDSPHL,
    LDFF(LoadFFType),
    LDSPA16,
    // Control flow instructions
    JP(JumpTest),
    JR(JumpTest),
    JPHL,
    CALL(JumpTest),
    RET(JumpTest),
    RST(BitTarget),
    RETI,
    // Stack instructions
    PUSH(StackTarget),
    POP(StackTarget),
    // Prefix instructions
    BIT(BitType),
    RL(RegistersIndirect),
    RLC(RegistersIndirect),
    RR(RegistersIndirect),
    RRC(RegistersIndirect),
    RLA,
    RLCA,
    RRA,
    RRCA,
    SLA(RegistersIndirect),
    SRA(RegistersIndirect),
    SRL(RegistersIndirect),
    SWAP(RegistersIndirect),
    DI,
    EI,
    RES(ResSetType),
    SET(ResSetType)
}

#[derive(Clone, Debug)]
pub(crate) enum ResSetType {
    Registers(BitTarget, RegistersIndirect),    
}

#[derive(Clone, Debug)]
pub(crate) enum BitType {
    Registers(BitTarget, RegistersIndirect),    
}

#[derive(Clone, Debug)]
pub(crate) enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
}

#[derive(Clone, Debug)]
pub(crate) enum RegistersIndirect {
    A, B, C, D, E, H, L, HLI
}

#[derive(Clone, Debug)]
pub(crate) enum WordRegister {
    BC, DE, HL, SP
}

#[derive(Clone, Debug)]
pub(crate) enum StackTarget {
    BC, DE, HL, AF
}

#[derive(Clone, Debug)]
pub(crate) enum LoadIndirectSource {
    BC, DE, HLInc, HLDec
}

#[derive(Clone, Debug)]
pub(crate) enum RegistersIndDir {
    A, B, C, D, E, H, L, HLI, D8
}

#[derive(Clone, Debug)]
pub(crate) enum LoadType {
    Byte(RegistersIndirect, RegistersIndDir),
    Word(WordRegister),
    AFromIndirect(LoadIndirectSource),
    IndirectFromA(LoadIndirectSource),
    AFromDirect,
    DirectFromA,
}

#[derive(Clone, Debug)]
pub(crate) enum LoadFFType {
    AtoFFC,
    FFCtoA,
    FFa8toA,
    AtoFFa8
}

#[derive(Clone, Debug)]
pub(crate) enum BitTarget {
    Zero, One, Two, Three, Four, Five, Six, Seven
}

#[derive(Clone, Debug)]
pub(crate) enum RotateDirection {
    Left, Right
}

impl Instruction {
    pub(crate) fn size(&self) -> InstructionSize {
        match self {
            Instruction::NOP => InstructionSize::OneByte,
            Instruction::HALT => InstructionSize::OneByte,
            Instruction::STOP => InstructionSize::TwoBytes,
            Instruction::SCF => InstructionSize::OneByte,
            Instruction::CCF => InstructionSize::OneByte,
            Instruction::CPL => InstructionSize::OneByte,
            Instruction::ADD(atarget) => match atarget { RegistersIndDir::D8 => InstructionSize::TwoBytes, _ => InstructionSize::OneByte },
            Instruction::ADC(atarget) => match atarget { RegistersIndDir::D8 => InstructionSize::TwoBytes, _ => InstructionSize::OneByte },
            Instruction::INC(_) => InstructionSize::OneByte,
            Instruction::DEC(_) => InstructionSize::OneByte,
            Instruction::ADD16(_) => InstructionSize::OneByte,
            Instruction::INC16(_) => InstructionSize::OneByte,
            Instruction::DEC16(_) => InstructionSize::OneByte,
            Instruction::ADDSPS8 => InstructionSize::TwoBytes,
            Instruction::SUB(atarget) => match atarget { RegistersIndDir::D8 => InstructionSize::TwoBytes, _ => InstructionSize::OneByte },
            Instruction::SBC(atarget) => match atarget { RegistersIndDir::D8 => InstructionSize::TwoBytes, _ => InstructionSize::OneByte },
            Instruction::AND(atarget) => match atarget { RegistersIndDir::D8 => InstructionSize::TwoBytes, _ => InstructionSize::OneByte },
            Instruction::XOR(atarget) => match atarget { RegistersIndDir::D8 => InstructionSize::TwoBytes, _ => InstructionSize::OneByte },
            Instruction::OR(atarget) => match atarget { RegistersIndDir::D8 => InstructionSize::TwoBytes, _ => InstructionSize::OneByte },
            Instruction::CP(atarget) => match atarget { RegistersIndDir::D8 => InstructionSize::TwoBytes, _ => InstructionSize::OneByte },
            Instruction::LD(load_type) => match load_type {
                                                        LoadType::AFromDirect => InstructionSize::ThreeBytes,
                                                        LoadType::DirectFromA => InstructionSize::ThreeBytes,
                                                        LoadType::Byte(_, source) => match source {
                                                                                                            RegistersIndDir::D8 => InstructionSize::TwoBytes,
                                                                                                            _ => InstructionSize::OneByte
                                                                                                      },
                                                        LoadType::Word(_) => InstructionSize::ThreeBytes,
                                                        LoadType::AFromIndirect(_) => InstructionSize::OneByte,
                                                        LoadType::IndirectFromA(_) => InstructionSize::OneByte,
                                                    },
            Instruction::LDHLSPD8 => InstructionSize::TwoBytes,
            Instruction::LDSPHL => InstructionSize::OneByte,
            Instruction::LDSPA16 => InstructionSize::ThreeBytes,
            Instruction::LDFF(load_type) => match load_type {
                                                            LoadFFType::AtoFFa8 => InstructionSize::TwoBytes,
                                                            LoadFFType::FFa8toA => InstructionSize::TwoBytes,
                                                            LoadFFType::AtoFFC => InstructionSize::OneByte,
                                                            LoadFFType::FFCtoA => InstructionSize::OneByte,
                                                        },
            Instruction::JP(_) => InstructionSize::ThreeBytes,
            Instruction::JR(_) => InstructionSize::TwoBytes,
            Instruction::JPHL => InstructionSize::OneByte,
            Instruction::PUSH(_) => InstructionSize::OneByte,
            Instruction::POP(_) => InstructionSize::OneByte,
            Instruction::CALL(_) => InstructionSize::ThreeBytes,
            Instruction::RET(_) => InstructionSize::OneByte,
            Instruction::RST(_) => InstructionSize::OneByte,
            Instruction::BIT(_) => InstructionSize::TwoBytes,
            Instruction::RETI => InstructionSize::OneByte,
            Instruction::DAA => InstructionSize::OneByte,
            Instruction::RL(_) => InstructionSize::TwoBytes,
            Instruction::RLC(_) => InstructionSize::TwoBytes,
            Instruction::RR(_) => InstructionSize::TwoBytes,
            Instruction::RRC(_) => InstructionSize::TwoBytes,
            Instruction::RLA => InstructionSize::OneByte,
            Instruction::RLCA => InstructionSize::OneByte,
            Instruction::RRA => InstructionSize::OneByte,
            Instruction::RRCA => InstructionSize::OneByte,
            Instruction::SLA(_) => InstructionSize::TwoBytes,
            Instruction::SRA(_) => InstructionSize::TwoBytes,
            Instruction::SRL(_) => InstructionSize::TwoBytes,
            Instruction::SWAP(_) => InstructionSize::TwoBytes,
            Instruction::DI => InstructionSize::OneByte,
            Instruction::EI => InstructionSize::OneByte,
            Instruction::RES(_) => InstructionSize::TwoBytes,
            Instruction::SET(_) => InstructionSize::TwoBytes,
        }

    }

    pub(crate) fn from_byte_prefixed(byte: u8) -> Result<Instruction,Error> {
        match byte {
            0x00 => Ok(Instruction::RLC(RegistersIndirect::B)),
            0x01 => Ok(Instruction::RLC(RegistersIndirect::C)),
            0x02 => Ok(Instruction::RLC(RegistersIndirect::D)),
            0x03 => Ok(Instruction::RLC(RegistersIndirect::E)),
            0x04 => Ok(Instruction::RLC(RegistersIndirect::H)),
            0x05 => Ok(Instruction::RLC(RegistersIndirect::L)),
            0x06 => Ok(Instruction::RLC(RegistersIndirect::HLI)),
            0x07 => Ok(Instruction::RLC(RegistersIndirect::A)),

            0x08 => Ok(Instruction::RRC(RegistersIndirect::B)),
            0x09 => Ok(Instruction::RRC(RegistersIndirect::C)),
            0x0A => Ok(Instruction::RRC(RegistersIndirect::D)),
            0x0B => Ok(Instruction::RRC(RegistersIndirect::E)),
            0x0C => Ok(Instruction::RRC(RegistersIndirect::H)),
            0x0D => Ok(Instruction::RRC(RegistersIndirect::L)),
            0x0E => Ok(Instruction::RRC(RegistersIndirect::HLI)),
            0x0F => Ok(Instruction::RRC(RegistersIndirect::A)),

            0x10 => Ok(Instruction::RL(RegistersIndirect::B)),
            0x11 => Ok(Instruction::RL(RegistersIndirect::C)),
            0x12 => Ok(Instruction::RL(RegistersIndirect::D)),
            0x13 => Ok(Instruction::RL(RegistersIndirect::E)),
            0x14 => Ok(Instruction::RL(RegistersIndirect::H)),
            0x15 => Ok(Instruction::RL(RegistersIndirect::L)),
            0x16 => Ok(Instruction::RL(RegistersIndirect::HLI)),
            0x17 => Ok(Instruction::RL(RegistersIndirect::A)),

            0x18 => Ok(Instruction::RR(RegistersIndirect::B)),
            0x19 => Ok(Instruction::RR(RegistersIndirect::C)),
            0x1A => Ok(Instruction::RR(RegistersIndirect::D)),
            0x1B => Ok(Instruction::RR(RegistersIndirect::E)),
            0x1C => Ok(Instruction::RR(RegistersIndirect::H)),
            0x1D => Ok(Instruction::RR(RegistersIndirect::L)),
            0x1E => Ok(Instruction::RR(RegistersIndirect::HLI)),
            0x1F => Ok(Instruction::RR(RegistersIndirect::A)),

            0x20 => Ok(Instruction::SLA(RegistersIndirect::B)),
            0x21 => Ok(Instruction::SLA(RegistersIndirect::C)),
            0x22 => Ok(Instruction::SLA(RegistersIndirect::D)),
            0x23 => Ok(Instruction::SLA(RegistersIndirect::E)),
            0x24 => Ok(Instruction::SLA(RegistersIndirect::H)),
            0x25 => Ok(Instruction::SLA(RegistersIndirect::L)),
            0x26 => Ok(Instruction::SLA(RegistersIndirect::HLI)),
            0x27 => Ok(Instruction::SLA(RegistersIndirect::A)),

            0x28 => Ok(Instruction::SRA(RegistersIndirect::B)),
            0x29 => Ok(Instruction::SRA(RegistersIndirect::C)),
            0x2A => Ok(Instruction::SRA(RegistersIndirect::D)),
            0x2B => Ok(Instruction::SRA(RegistersIndirect::E)),
            0x2C => Ok(Instruction::SRA(RegistersIndirect::H)),
            0x2D => Ok(Instruction::SRA(RegistersIndirect::L)),
            0x2E => Ok(Instruction::SRA(RegistersIndirect::HLI)),
            0x2F => Ok(Instruction::SRA(RegistersIndirect::A)),

            0x30 => Ok(Instruction::SWAP(RegistersIndirect::B)),
            0x31 => Ok(Instruction::SWAP(RegistersIndirect::C)),
            0x32 => Ok(Instruction::SWAP(RegistersIndirect::D)),
            0x33 => Ok(Instruction::SWAP(RegistersIndirect::E)),
            0x34 => Ok(Instruction::SWAP(RegistersIndirect::H)),
            0x35 => Ok(Instruction::SWAP(RegistersIndirect::L)),
            0x36 => Ok(Instruction::SWAP(RegistersIndirect::HLI)),
            0x37 => Ok(Instruction::SWAP(RegistersIndirect::A)),

            0x38 => Ok(Instruction::SRL(RegistersIndirect::B)),
            0x39 => Ok(Instruction::SRL(RegistersIndirect::C)),
            0x3A => Ok(Instruction::SRL(RegistersIndirect::D)),
            0x3B => Ok(Instruction::SRL(RegistersIndirect::E)),
            0x3C => Ok(Instruction::SRL(RegistersIndirect::H)),
            0x3D => Ok(Instruction::SRL(RegistersIndirect::L)),
            0x3E => Ok(Instruction::SRL(RegistersIndirect::HLI)),
            0x3F => Ok(Instruction::SRL(RegistersIndirect::A)),

            // BIT
            0x40 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Zero, RegistersIndirect::B))),
            0x41 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Zero, RegistersIndirect::C))),
            0x42 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Zero, RegistersIndirect::D))),
            0x43 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Zero, RegistersIndirect::E))),
            0x44 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Zero, RegistersIndirect::H))),
            0x45 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Zero, RegistersIndirect::L))),
            0x46 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Zero, RegistersIndirect::HLI))),
            0x47 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Zero, RegistersIndirect::A))),

            0x48 => Ok(Instruction::BIT(BitType::Registers(BitTarget::One, RegistersIndirect::B))),
            0x49 => Ok(Instruction::BIT(BitType::Registers(BitTarget::One, RegistersIndirect::C))),
            0x4A => Ok(Instruction::BIT(BitType::Registers(BitTarget::One, RegistersIndirect::D))),
            0x4B => Ok(Instruction::BIT(BitType::Registers(BitTarget::One, RegistersIndirect::E))),
            0x4C => Ok(Instruction::BIT(BitType::Registers(BitTarget::One, RegistersIndirect::H))),
            0x4D => Ok(Instruction::BIT(BitType::Registers(BitTarget::One, RegistersIndirect::L))),
            0x4E => Ok(Instruction::BIT(BitType::Registers(BitTarget::One, RegistersIndirect::HLI))),
            0x4F => Ok(Instruction::BIT(BitType::Registers(BitTarget::One, RegistersIndirect::A))),

            0x50 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Two, RegistersIndirect::B))),
            0x51 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Two, RegistersIndirect::C))),
            0x52 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Two, RegistersIndirect::D))),
            0x53 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Two, RegistersIndirect::E))),
            0x54 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Two, RegistersIndirect::H))),
            0x55 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Two, RegistersIndirect::L))),
            0x56 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Two, RegistersIndirect::HLI))),
            0x57 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Two, RegistersIndirect::A))),
            
            0x58 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Three, RegistersIndirect::B))),
            0x59 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Three, RegistersIndirect::C))),
            0x5A => Ok(Instruction::BIT(BitType::Registers(BitTarget::Three, RegistersIndirect::D))),
            0x5B => Ok(Instruction::BIT(BitType::Registers(BitTarget::Three, RegistersIndirect::E))),
            0x5C => Ok(Instruction::BIT(BitType::Registers(BitTarget::Three, RegistersIndirect::H))),
            0x5D => Ok(Instruction::BIT(BitType::Registers(BitTarget::Three, RegistersIndirect::L))),
            0x5E => Ok(Instruction::BIT(BitType::Registers(BitTarget::Three, RegistersIndirect::HLI))),
            0x5F => Ok(Instruction::BIT(BitType::Registers(BitTarget::Three, RegistersIndirect::A))),

            0x60 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Four, RegistersIndirect::B))),
            0x61 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Four, RegistersIndirect::C))),
            0x62 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Four, RegistersIndirect::D))),
            0x63 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Four, RegistersIndirect::E))),
            0x64 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Four, RegistersIndirect::H))),
            0x65 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Four, RegistersIndirect::L))),
            0x66 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Four, RegistersIndirect::HLI))),
            0x67 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Four, RegistersIndirect::A))),
            
            0x68 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Five, RegistersIndirect::B))),
            0x69 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Five, RegistersIndirect::C))),
            0x6A => Ok(Instruction::BIT(BitType::Registers(BitTarget::Five, RegistersIndirect::D))),
            0x6B => Ok(Instruction::BIT(BitType::Registers(BitTarget::Five, RegistersIndirect::E))),
            0x6C => Ok(Instruction::BIT(BitType::Registers(BitTarget::Five, RegistersIndirect::H))),
            0x6D => Ok(Instruction::BIT(BitType::Registers(BitTarget::Five, RegistersIndirect::L))),
            0x6E => Ok(Instruction::BIT(BitType::Registers(BitTarget::Five, RegistersIndirect::HLI))),
            0x6F => Ok(Instruction::BIT(BitType::Registers(BitTarget::Five, RegistersIndirect::A))),

            0x70 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Six, RegistersIndirect::B))),
            0x71 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Six, RegistersIndirect::C))),
            0x72 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Six, RegistersIndirect::D))),
            0x73 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Six, RegistersIndirect::E))),
            0x74 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Six, RegistersIndirect::H))),
            0x75 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Six, RegistersIndirect::L))),
            0x76 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Six, RegistersIndirect::HLI))),
            0x77 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Six, RegistersIndirect::A))),

            0x78 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Seven, RegistersIndirect::B))),
            0x79 => Ok(Instruction::BIT(BitType::Registers(BitTarget::Seven, RegistersIndirect::C))),
            0x7A => Ok(Instruction::BIT(BitType::Registers(BitTarget::Seven, RegistersIndirect::D))),
            0x7B => Ok(Instruction::BIT(BitType::Registers(BitTarget::Seven, RegistersIndirect::E))),
            0x7C => Ok(Instruction::BIT(BitType::Registers(BitTarget::Seven, RegistersIndirect::H))),
            0x7D => Ok(Instruction::BIT(BitType::Registers(BitTarget::Seven, RegistersIndirect::L))),
            0x7E => Ok(Instruction::BIT(BitType::Registers(BitTarget::Seven, RegistersIndirect::HLI))),
            0x7F => Ok(Instruction::BIT(BitType::Registers(BitTarget::Seven, RegistersIndirect::A))),

            0x80 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::B))),
            0x81 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::C))),
            0x82 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::D))),
            0x83 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::E))),
            0x84 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::H))),
            0x85 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::L))),
            0x86 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::HLI))),
            0x87 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::A))),

            0x88 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::One, RegistersIndirect::B))),
            0x89 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::One, RegistersIndirect::C))),
            0x8A => Ok(Instruction::RES(ResSetType::Registers(BitTarget::One, RegistersIndirect::D))),
            0x8B => Ok(Instruction::RES(ResSetType::Registers(BitTarget::One, RegistersIndirect::E))),
            0x8C => Ok(Instruction::RES(ResSetType::Registers(BitTarget::One, RegistersIndirect::H))),
            0x8D => Ok(Instruction::RES(ResSetType::Registers(BitTarget::One, RegistersIndirect::L))),
            0x8E => Ok(Instruction::RES(ResSetType::Registers(BitTarget::One, RegistersIndirect::HLI))),
            0x8F => Ok(Instruction::RES(ResSetType::Registers(BitTarget::One, RegistersIndirect::A))),

            0x90 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Two, RegistersIndirect::B))),
            0x91 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Two, RegistersIndirect::C))),
            0x92 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Two, RegistersIndirect::D))),
            0x93 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Two, RegistersIndirect::E))),
            0x94 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Two, RegistersIndirect::H))),
            0x95 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Two, RegistersIndirect::L))),
            0x96 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Two, RegistersIndirect::HLI))),
            0x97 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Two, RegistersIndirect::A))),
            
            0x98 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Three, RegistersIndirect::B))),
            0x99 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Three, RegistersIndirect::C))),
            0x9A => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Three, RegistersIndirect::D))),
            0x9B => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Three, RegistersIndirect::E))),
            0x9C => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Three, RegistersIndirect::H))),
            0x9D => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Three, RegistersIndirect::L))),
            0x9E => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Three, RegistersIndirect::HLI))),
            0x9F => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Three, RegistersIndirect::A))),

            0xA0 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Four, RegistersIndirect::B))),
            0xA1 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Four, RegistersIndirect::C))),
            0xA2 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Four, RegistersIndirect::D))),
            0xA3 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Four, RegistersIndirect::E))),
            0xA4 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Four, RegistersIndirect::H))),
            0xA5 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Four, RegistersIndirect::L))),
            0xA6 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Four, RegistersIndirect::HLI))),
            0xA7 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Four, RegistersIndirect::A))),
            
            0xA8 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Five, RegistersIndirect::B))),
            0xA9 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Five, RegistersIndirect::C))),
            0xAA => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Five, RegistersIndirect::D))),
            0xAB => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Five, RegistersIndirect::E))),
            0xAC => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Five, RegistersIndirect::H))),
            0xAD => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Five, RegistersIndirect::L))),
            0xAE => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Five, RegistersIndirect::HLI))),
            0xAF => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Five, RegistersIndirect::A))),

            0xB0 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Six, RegistersIndirect::B))),
            0xB1 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Six, RegistersIndirect::C))),
            0xB2 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Six, RegistersIndirect::D))),
            0xB3 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Six, RegistersIndirect::E))),
            0xB4 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Six, RegistersIndirect::H))),
            0xB5 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Six, RegistersIndirect::L))),
            0xB6 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Six, RegistersIndirect::HLI))),
            0xB7 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Six, RegistersIndirect::A))),

            0xB8 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::B))),
            0xB9 => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::C))),
            0xBA => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::D))),
            0xBB => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::E))),
            0xBC => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::H))),
            0xBD => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::L))),
            0xBE => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::HLI))),
            0xBF => Ok(Instruction::RES(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::A))),

            0xC0 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::B))),
            0xC1 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::C))),
            0xC2 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::D))),
            0xC3 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::E))),
            0xC4 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::H))),
            0xC5 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::L))),
            0xC6 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::HLI))),
            0xC7 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::A))),

            0xC8 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::One, RegistersIndirect::B))),
            0xC9 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::One, RegistersIndirect::C))),
            0xCA => Ok(Instruction::SET(ResSetType::Registers(BitTarget::One, RegistersIndirect::D))),
            0xCB => Ok(Instruction::SET(ResSetType::Registers(BitTarget::One, RegistersIndirect::E))),
            0xCC => Ok(Instruction::SET(ResSetType::Registers(BitTarget::One, RegistersIndirect::H))),
            0xCD => Ok(Instruction::SET(ResSetType::Registers(BitTarget::One, RegistersIndirect::L))),
            0xCE => Ok(Instruction::SET(ResSetType::Registers(BitTarget::One, RegistersIndirect::HLI))),
            0xCF => Ok(Instruction::SET(ResSetType::Registers(BitTarget::One, RegistersIndirect::A))),

            0xD0 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Two, RegistersIndirect::B))),
            0xD1 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Two, RegistersIndirect::C))),
            0xD2 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Two, RegistersIndirect::D))),
            0xD3 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Two, RegistersIndirect::E))),
            0xD4 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Two, RegistersIndirect::H))),
            0xD5 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Two, RegistersIndirect::L))),
            0xD6 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Two, RegistersIndirect::HLI))),
            0xD7 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Two, RegistersIndirect::A))),
            
            0xD8 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Three, RegistersIndirect::B))),
            0xD9 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Three, RegistersIndirect::C))),
            0xDA => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Three, RegistersIndirect::D))),
            0xDB => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Three, RegistersIndirect::E))),
            0xDC => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Three, RegistersIndirect::H))),
            0xDD => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Three, RegistersIndirect::L))),
            0xDE => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Three, RegistersIndirect::HLI))),
            0xDF => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Three, RegistersIndirect::A))),

            0xE0 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Four, RegistersIndirect::B))),
            0xE1 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Four, RegistersIndirect::C))),
            0xE2 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Four, RegistersIndirect::D))),
            0xE3 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Four, RegistersIndirect::E))),
            0xE4 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Four, RegistersIndirect::H))),
            0xE5 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Four, RegistersIndirect::L))),
            0xE6 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Four, RegistersIndirect::HLI))),
            0xE7 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Four, RegistersIndirect::A))),
            
            0xE8 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Five, RegistersIndirect::B))),
            0xE9 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Five, RegistersIndirect::C))),
            0xEA => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Five, RegistersIndirect::D))),
            0xEB => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Five, RegistersIndirect::E))),
            0xEC => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Five, RegistersIndirect::H))),
            0xED => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Five, RegistersIndirect::L))),
            0xEE => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Five, RegistersIndirect::HLI))),
            0xEF => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Five, RegistersIndirect::A))),

            0xF0 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Six, RegistersIndirect::B))),
            0xF1 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Six, RegistersIndirect::C))),
            0xF2 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Six, RegistersIndirect::D))),
            0xF3 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Six, RegistersIndirect::E))),
            0xF4 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Six, RegistersIndirect::H))),
            0xF5 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Six, RegistersIndirect::L))),
            0xF6 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Six, RegistersIndirect::HLI))),
            0xF7 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Six, RegistersIndirect::A))),

            0xF8 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::B))),
            0xF9 => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::C))),
            0xFA => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::D))),
            0xFB => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::E))),
            0xFC => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::H))),
            0xFD => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::L))),
            0xFE => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::HLI))),
            0xFF => Ok(Instruction::SET(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::A))),
        }
    }
    
    pub(crate) fn from_byte_not_prefixed(byte: u8) -> Result<Instruction, Error> {
        match byte {
            // Miscellaneous InstructionTypes
            0x00 => Ok(Instruction::NOP),
            0x76 => Ok(Instruction::HALT),
            0x10 => Ok(Instruction::STOP),
            0x27 => Ok(Instruction::DAA),
            0x37 => Ok(Instruction::SCF),
            0x2F => Ok(Instruction::CPL),
            0x3F => Ok(Instruction::CCF),
            0xF3 => Ok(Instruction::DI),
            0xFB => Ok(Instruction::EI),

            // Rotate InstructionTypes
            0x07 => Ok(Instruction::RLCA),
            0x17 => Ok(Instruction::RLA),
            0x0F => Ok(Instruction::RRCA),
            0x1F => Ok(Instruction::RRA),

            // Stack InstructionTypes
            0xC1 => Ok(Instruction::POP(StackTarget::BC)),
            0xD1 => Ok(Instruction::POP(StackTarget::DE)),
            0xE1 => Ok(Instruction::POP(StackTarget::HL)),
            0xF1 => Ok(Instruction::POP(StackTarget::AF)),
            0xC5 => Ok(Instruction::PUSH(StackTarget::BC)),
            0xD5 => Ok(Instruction::PUSH(StackTarget::DE)),
            0xE5 => Ok(Instruction::PUSH(StackTarget::HL)),
            0xF5 => Ok(Instruction::PUSH(StackTarget::AF)),
            0xF8 => Ok(Instruction::LDHLSPD8),
            0xF9 => Ok(Instruction::LDSPHL),
            0x08 => Ok(Instruction::LDSPA16),

            // Control flow InstructionTypes
            0x18 => Ok(Instruction::JR(JumpTest::Always)),
            0x28 => Ok(Instruction::JR(JumpTest::Zero)),
            0x38 => Ok(Instruction::JR(JumpTest::Carry)),
            0xC0 => Ok(Instruction::RET(JumpTest::NotZero)),
            0xD0 => Ok(Instruction::RET(JumpTest::NotCarry)),
            0xC2 => Ok(Instruction::JP(JumpTest::NotZero)),
            0xD2 => Ok(Instruction::JP(JumpTest::NotCarry)),
            0xC3 => Ok(Instruction::JP(JumpTest::Always)),
            0xC4 => Ok(Instruction::CALL(JumpTest::NotZero)),
            0xD4 => Ok(Instruction::CALL(JumpTest::NotCarry)),
            0xC7 => Ok(Instruction::RST(BitTarget::Zero)),
            0xD7 => Ok(Instruction::RST(BitTarget::Two)),
            0xE7 => Ok(Instruction::RST(BitTarget::Four)),
            0xF7 => Ok(Instruction::RST(BitTarget::Six)),
            0xC8 => Ok(Instruction::RET(JumpTest::Zero)),
            0xD8 => Ok(Instruction::RET(JumpTest::Carry)),
            0xC9 => Ok(Instruction::RET(JumpTest::Always)),
            0xD9 => Ok(Instruction::RETI),
            0xE9 => Ok(Instruction::JPHL),
            0xCA => Ok(Instruction::JP(JumpTest::Zero)),
            0xDA => Ok(Instruction::JP(JumpTest::Carry)),
            0xCC => Ok(Instruction::CALL(JumpTest::Zero)),
            0xDC => Ok(Instruction::CALL(JumpTest::Carry)),
            0xCD => Ok(Instruction::CALL(JumpTest::Always)),
            0xCF => Ok(Instruction::RST(BitTarget::One)),
            0xDF => Ok(Instruction::RST(BitTarget::Three)),
            0xEF => Ok(Instruction::RST(BitTarget::Five)),
            0xFF => Ok(Instruction::RST(BitTarget::Seven)),
            0x20 => Ok(Instruction::JR(JumpTest::NotZero)),
            0x30 => Ok(Instruction::JR(JumpTest::NotCarry)),

            // 16-bit load InstructionTypes
            0x01 => Ok(Instruction::LD(LoadType::Word(WordRegister::BC))),
            0x11 => Ok(Instruction::LD(LoadType::Word(WordRegister::DE))),
            0x21 => Ok(Instruction::LD(LoadType::Word(WordRegister::HL))),
            0x31 => Ok(Instruction::LD(LoadType::Word(WordRegister::SP))),

            // 16-bit Arithmetic/Logic InstructionTypes
            0x09 => Ok(Instruction::ADD16(WordRegister::BC)),
            0x19 => Ok(Instruction::ADD16(WordRegister::DE)),
            0x29 => Ok(Instruction::ADD16(WordRegister::HL)),
            0x39 => Ok(Instruction::ADD16(WordRegister::SP)),
            0x03 => Ok(Instruction::INC16(WordRegister::BC)),
            0x13 => Ok(Instruction::INC16(WordRegister::DE)),
            0x23 => Ok(Instruction::INC16(WordRegister::HL)),
            0x33 => Ok(Instruction::INC16(WordRegister::SP)),
            0x0B => Ok(Instruction::DEC16(WordRegister::BC)),
            0x1B => Ok(Instruction::DEC16(WordRegister::DE)),
            0x2B => Ok(Instruction::DEC16(WordRegister::HL)),
            0x3B => Ok(Instruction::DEC16(WordRegister::SP)),
            0xE8 => Ok(Instruction::ADDSPS8),
            
            // 8-bit load InstructionTypes
            0x02 => Ok(Instruction::LD(LoadType::IndirectFromA(LoadIndirectSource::BC))),
            0x12 => Ok(Instruction::LD(LoadType::IndirectFromA(LoadIndirectSource::DE))),
            0x22 => Ok(Instruction::LD(LoadType::IndirectFromA(LoadIndirectSource::HLInc))),
            0x32 => Ok(Instruction::LD(LoadType::IndirectFromA(LoadIndirectSource::HLDec))),
            0x40 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::B, RegistersIndDir::B))),
            0x41 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::B, RegistersIndDir::C))),
            0x42 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::B, RegistersIndDir::D))),
            0x43 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::B, RegistersIndDir::E))),
            0x44 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::B, RegistersIndDir::H))),
            0x45 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::B, RegistersIndDir::L))),
            0x46 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::B, RegistersIndDir::HLI))),
            0x47 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::B, RegistersIndDir::A))),
            0x48 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::C, RegistersIndDir::B))),
            0x49 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::C, RegistersIndDir::C))),
            0x4A => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::C, RegistersIndDir::D))),
            0x4B => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::C, RegistersIndDir::E))),
            0x4C => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::C, RegistersIndDir::H))),
            0x4D => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::C, RegistersIndDir::L))),
            0x4E => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::C, RegistersIndDir::HLI))),
            0x4F => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::C, RegistersIndDir::A))),

            0x50 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::D, RegistersIndDir::B))),
            0x51 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::D, RegistersIndDir::C))),
            0x52 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::D, RegistersIndDir::D))),
            0x53 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::D, RegistersIndDir::E))),
            0x54 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::D, RegistersIndDir::H))),
            0x55 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::D, RegistersIndDir::L))),
            0x56 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::D, RegistersIndDir::HLI))),
            0x57 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::D, RegistersIndDir::A))),
            0x58 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::E, RegistersIndDir::B))),
            0x59 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::E, RegistersIndDir::C))),
            0x5A => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::E, RegistersIndDir::D))),
            0x5B => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::E, RegistersIndDir::E))),
            0x5C => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::E, RegistersIndDir::H))),
            0x5D => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::E, RegistersIndDir::L))),
            0x5E => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::E, RegistersIndDir::HLI))),
            0x5F => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::E, RegistersIndDir::A))),

            0x60 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::H, RegistersIndDir::B))),
            0x61 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::H, RegistersIndDir::C))),
            0x62 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::H, RegistersIndDir::D))),
            0x63 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::H, RegistersIndDir::E))),
            0x64 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::H, RegistersIndDir::H))),
            0x65 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::H, RegistersIndDir::L))),
            0x66 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::H, RegistersIndDir::HLI))),
            0x67 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::H, RegistersIndDir::A))),
            0x68 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::L, RegistersIndDir::B))),
            0x69 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::L, RegistersIndDir::C))),
            0x6A => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::L, RegistersIndDir::D))),
            0x6B => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::L, RegistersIndDir::E))),
            0x6C => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::L, RegistersIndDir::H))),
            0x6D => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::L, RegistersIndDir::L))),
            0x6E => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::L, RegistersIndDir::HLI))),
            0x6F => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::L, RegistersIndDir::A))),

            0x70 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::HLI, RegistersIndDir::B))),
            0x71 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::HLI, RegistersIndDir::C))),
            0x72 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::HLI, RegistersIndDir::D))),
            0x73 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::HLI, RegistersIndDir::E))),
            0x74 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::HLI, RegistersIndDir::H))),
            0x75 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::HLI, RegistersIndDir::L))),
            0x77 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::HLI, RegistersIndDir::A))),
            0x78 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::A, RegistersIndDir::B))),
            0x79 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::A, RegistersIndDir::C))),
            0x7A => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::A, RegistersIndDir::D))),
            0x7B => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::A, RegistersIndDir::E))),
            0x7C => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::A, RegistersIndDir::H))),
            0x7D => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::A, RegistersIndDir::L))),
            0x7E => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::A, RegistersIndDir::HLI))),
            0x7F => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::A, RegistersIndDir::A))),

            0x06 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::B, RegistersIndDir::D8))),
            0x16 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::D, RegistersIndDir::D8))),
            0x26 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::H, RegistersIndDir::D8))),
            0x36 => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::HLI, RegistersIndDir::D8))),
            0x0E => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::C, RegistersIndDir::D8))),
            0x1E => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::E, RegistersIndDir::D8))),
            0x2E => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::L, RegistersIndDir::D8))),
            0x3E => Ok(Instruction::LD(LoadType::Byte(RegistersIndirect::A, RegistersIndDir::D8))),

            0x0A => Ok(Instruction::LD(LoadType::AFromIndirect(LoadIndirectSource::BC))),
            0x1A => Ok(Instruction::LD(LoadType::AFromIndirect(LoadIndirectSource::DE))),
            0x2A => Ok(Instruction::LD(LoadType::AFromIndirect(LoadIndirectSource::HLInc))),
            0x3A => Ok(Instruction::LD(LoadType::AFromIndirect(LoadIndirectSource::HLDec))),
            0xE0 => Ok(Instruction::LDFF(LoadFFType::AtoFFa8)),
            0xF0 => Ok(Instruction::LDFF(LoadFFType::FFa8toA)),
            0xE2 => Ok(Instruction::LDFF(LoadFFType::AtoFFC)),
            0xF2 => Ok(Instruction::LDFF(LoadFFType::FFCtoA)),
            0xEA => Ok(Instruction::LD(LoadType::DirectFromA)),
            0xFA => Ok(Instruction::LD(LoadType::AFromDirect)),

            // 8-bit arithmetic and logical InstructionTypes
            0x04 => Ok(Instruction::INC(RegistersIndirect::B)),
            0x14 => Ok(Instruction::INC(RegistersIndirect::D)),
            0x24 => Ok(Instruction::INC(RegistersIndirect::H)),
            0x34 => Ok(Instruction::INC(RegistersIndirect::HLI)),

            0x05 => Ok(Instruction::DEC(RegistersIndirect::B)),
            0x15 => Ok(Instruction::DEC(RegistersIndirect::D)),
            0x25 => Ok(Instruction::DEC(RegistersIndirect::H)),
            0x35 => Ok(Instruction::DEC(RegistersIndirect::HLI)),

            0x0C => Ok(Instruction::INC(RegistersIndirect::C)),
            0x1C => Ok(Instruction::INC(RegistersIndirect::E)),
            0x2C => Ok(Instruction::INC(RegistersIndirect::L)),
            0x3C => Ok(Instruction::INC(RegistersIndirect::A)),

            0x0D => Ok(Instruction::DEC(RegistersIndirect::C)),
            0x1D => Ok(Instruction::DEC(RegistersIndirect::E)),
            0x2D => Ok(Instruction::DEC(RegistersIndirect::L)),
            0x3D => Ok(Instruction::DEC(RegistersIndirect::A)),
            
            0x80 => Ok(Instruction::ADD(RegistersIndDir::B)),
            0x81 => Ok(Instruction::ADD(RegistersIndDir::C)),
            0x82 => Ok(Instruction::ADD(RegistersIndDir::D)),
            0x83 => Ok(Instruction::ADD(RegistersIndDir::E)),
            0x84 => Ok(Instruction::ADD(RegistersIndDir::H)),
            0x85 => Ok(Instruction::ADD(RegistersIndDir::L)),
            0x86 => Ok(Instruction::ADD(RegistersIndDir::HLI)),
            0x87 => Ok(Instruction::ADD(RegistersIndDir::A)),

            0x88 => Ok(Instruction::ADC(RegistersIndDir::B)),
            0x89 => Ok(Instruction::ADC(RegistersIndDir::C)),
            0x8A => Ok(Instruction::ADC(RegistersIndDir::D)),
            0x8B => Ok(Instruction::ADC(RegistersIndDir::E)),
            0x8C => Ok(Instruction::ADC(RegistersIndDir::H)),
            0x8D => Ok(Instruction::ADC(RegistersIndDir::L)),
            0x8E => Ok(Instruction::ADC(RegistersIndDir::HLI)),
            0x8F => Ok(Instruction::ADC(RegistersIndDir::A)),
            
            0x90 => Ok(Instruction::SUB(RegistersIndDir::B)),
            0x91 => Ok(Instruction::SUB(RegistersIndDir::C)),
            0x92 => Ok(Instruction::SUB(RegistersIndDir::D)),
            0x93 => Ok(Instruction::SUB(RegistersIndDir::E)),
            0x94 => Ok(Instruction::SUB(RegistersIndDir::H)),
            0x95 => Ok(Instruction::SUB(RegistersIndDir::L)),
            0x96 => Ok(Instruction::SUB(RegistersIndDir::HLI)),
            0x97 => Ok(Instruction::SUB(RegistersIndDir::A)),

            0x98 => Ok(Instruction::SBC(RegistersIndDir::B)),
            0x99 => Ok(Instruction::SBC(RegistersIndDir::C)),
            0x9A => Ok(Instruction::SBC(RegistersIndDir::D)),
            0x9B => Ok(Instruction::SBC(RegistersIndDir::E)),
            0x9C => Ok(Instruction::SBC(RegistersIndDir::H)),
            0x9D => Ok(Instruction::SBC(RegistersIndDir::L)),
            0x9E => Ok(Instruction::SBC(RegistersIndDir::HLI)),
            0x9F => Ok(Instruction::SBC(RegistersIndDir::A)),

            0xA0 => Ok(Instruction::AND(RegistersIndDir::B)),
            0xA1 => Ok(Instruction::AND(RegistersIndDir::C)),
            0xA2 => Ok(Instruction::AND(RegistersIndDir::D)),
            0xA3 => Ok(Instruction::AND(RegistersIndDir::E)),
            0xA4 => Ok(Instruction::AND(RegistersIndDir::H)),
            0xA5 => Ok(Instruction::AND(RegistersIndDir::L)),
            0xA6 => Ok(Instruction::AND(RegistersIndDir::HLI)),
            0xA7 => Ok(Instruction::AND(RegistersIndDir::A)),

            0xA8 => Ok(Instruction::XOR(RegistersIndDir::B)),
            0xA9 => Ok(Instruction::XOR(RegistersIndDir::C)),
            0xAA => Ok(Instruction::XOR(RegistersIndDir::D)),
            0xAB => Ok(Instruction::XOR(RegistersIndDir::E)),
            0xAC => Ok(Instruction::XOR(RegistersIndDir::H)),
            0xAD => Ok(Instruction::XOR(RegistersIndDir::L)),
            0xAE => Ok(Instruction::XOR(RegistersIndDir::HLI)),
            0xAF => Ok(Instruction::XOR(RegistersIndDir::A)),

            0xB0 => Ok(Instruction::OR(RegistersIndDir::B)),
            0xB1 => Ok(Instruction::OR(RegistersIndDir::C)),
            0xB2 => Ok(Instruction::OR(RegistersIndDir::D)),
            0xB3 => Ok(Instruction::OR(RegistersIndDir::E)),
            0xB4 => Ok(Instruction::OR(RegistersIndDir::H)),
            0xB5 => Ok(Instruction::OR(RegistersIndDir::L)),
            0xB6 => Ok(Instruction::OR(RegistersIndDir::HLI)),
            0xB7 => Ok(Instruction::OR(RegistersIndDir::A)),

            0xB8 => Ok(Instruction::CP(RegistersIndDir::B)),
            0xB9 => Ok(Instruction::CP(RegistersIndDir::C)),
            0xBA => Ok(Instruction::CP(RegistersIndDir::D)),
            0xBB => Ok(Instruction::CP(RegistersIndDir::E)),
            0xBC => Ok(Instruction::CP(RegistersIndDir::H)),
            0xBD => Ok(Instruction::CP(RegistersIndDir::L)),
            0xBE => Ok(Instruction::CP(RegistersIndDir::HLI)),
            0xBF => Ok(Instruction::CP(RegistersIndDir::A)),

            0xC6 => Ok(Instruction::ADD(RegistersIndDir::D8)),
            0xD6 => Ok(Instruction::SUB(RegistersIndDir::D8)),
            0xE6 => Ok(Instruction::AND(RegistersIndDir::D8)),
            0xF6 => Ok(Instruction::OR(RegistersIndDir::D8)),

            0xCE => Ok(Instruction::ADC(RegistersIndDir::D8)),
            0xDE => Ok(Instruction::SBC(RegistersIndDir::D8)),
            0xEE => Ok(Instruction::XOR(RegistersIndDir::D8)),
            0xFE => Ok(Instruction::CP(RegistersIndDir::D8)),

            // Invalid
            addr => Err(Error::new(std::io::ErrorKind::Other, format!("Decoded instruction {:x} is invalid", addr)))
        }
    }
}