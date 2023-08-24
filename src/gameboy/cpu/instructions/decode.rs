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
pub(crate) enum InstructionType {
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
pub(crate) enum PrefixTarget {
    A, B, C, D, E, H, L,
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

impl InstructionType {
    pub(crate) fn size(&self) -> InstructionSize {
        match self {
            InstructionType::NOP => InstructionSize::OneByte,
            InstructionType::HALT => InstructionSize::OneByte,
            InstructionType::STOP => InstructionSize::TwoBytes,
            InstructionType::SCF => InstructionSize::OneByte,
            InstructionType::CCF => InstructionSize::OneByte,
            InstructionType::CPL => InstructionSize::OneByte,
            InstructionType::ADD(atarget) => match atarget { RegistersIndDir::D8 => InstructionSize::TwoBytes, _ => InstructionSize::OneByte },
            InstructionType::ADC(atarget) => match atarget { RegistersIndDir::D8 => InstructionSize::TwoBytes, _ => InstructionSize::OneByte },
            InstructionType::INC(_) => InstructionSize::OneByte,
            InstructionType::DEC(_) => InstructionSize::OneByte,
            InstructionType::ADD16(_) => InstructionSize::OneByte,
            InstructionType::INC16(_) => InstructionSize::OneByte,
            InstructionType::DEC16(_) => InstructionSize::OneByte,
            InstructionType::ADDSPS8 => InstructionSize::TwoBytes,
            InstructionType::SUB(atarget) => match atarget { RegistersIndDir::D8 => InstructionSize::TwoBytes, _ => InstructionSize::OneByte },
            InstructionType::SBC(atarget) => match atarget { RegistersIndDir::D8 => InstructionSize::TwoBytes, _ => InstructionSize::OneByte },
            InstructionType::AND(atarget) => match atarget { RegistersIndDir::D8 => InstructionSize::TwoBytes, _ => InstructionSize::OneByte },
            InstructionType::XOR(atarget) => match atarget { RegistersIndDir::D8 => InstructionSize::TwoBytes, _ => InstructionSize::OneByte },
            InstructionType::OR(atarget) => match atarget { RegistersIndDir::D8 => InstructionSize::TwoBytes, _ => InstructionSize::OneByte },
            InstructionType::CP(atarget) => match atarget { RegistersIndDir::D8 => InstructionSize::TwoBytes, _ => InstructionSize::OneByte },
            InstructionType::LD(load_type) => match load_type {
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
            InstructionType::LDHLSPD8 => InstructionSize::TwoBytes,
            InstructionType::LDSPHL => InstructionSize::OneByte,
            InstructionType::LDSPA16 => InstructionSize::ThreeBytes,
            InstructionType::LDFF(load_type) => match load_type {
                                                            LoadFFType::AtoFFa8 => InstructionSize::TwoBytes,
                                                            LoadFFType::FFa8toA => InstructionSize::TwoBytes,
                                                            LoadFFType::AtoFFC => InstructionSize::OneByte,
                                                            LoadFFType::FFCtoA => InstructionSize::OneByte,
                                                        },
            InstructionType::JP(_) => InstructionSize::ThreeBytes,
            InstructionType::JR(_) => InstructionSize::TwoBytes,
            InstructionType::JPHL => InstructionSize::OneByte,
            InstructionType::PUSH(_) => InstructionSize::OneByte,
            InstructionType::POP(_) => InstructionSize::OneByte,
            InstructionType::CALL(_) => InstructionSize::ThreeBytes,
            InstructionType::RET(_) => InstructionSize::OneByte,
            InstructionType::RST(_) => InstructionSize::OneByte,
            InstructionType::BIT(_) => InstructionSize::TwoBytes,
            InstructionType::RETI => InstructionSize::OneByte,
            InstructionType::DAA => InstructionSize::OneByte,
            InstructionType::RL(_) => InstructionSize::TwoBytes,
            InstructionType::RLC(_) => InstructionSize::TwoBytes,
            InstructionType::RR(_) => InstructionSize::TwoBytes,
            InstructionType::RRC(_) => InstructionSize::TwoBytes,
            InstructionType::RLA => InstructionSize::OneByte,
            InstructionType::RLCA => InstructionSize::OneByte,
            InstructionType::RRA => InstructionSize::OneByte,
            InstructionType::RRCA => InstructionSize::OneByte,
            InstructionType::SLA(_) => InstructionSize::TwoBytes,
            InstructionType::SRA(_) => InstructionSize::TwoBytes,
            InstructionType::SRL(_) => InstructionSize::TwoBytes,
            InstructionType::SWAP(_) => InstructionSize::TwoBytes,
            InstructionType::DI => InstructionSize::OneByte,
            InstructionType::EI => InstructionSize::OneByte,
            InstructionType::RES(_) => InstructionSize::TwoBytes,
            InstructionType::SET(_) => InstructionSize::TwoBytes,
        }

    }

    pub(crate) fn from_byte_prefixed(byte: u8) -> Option<InstructionType> {
        match byte {
            0x00 => Some(InstructionType::RLC(RegistersIndirect::B)),
            0x01 => Some(InstructionType::RLC(RegistersIndirect::C)),
            0x02 => Some(InstructionType::RLC(RegistersIndirect::D)),
            0x03 => Some(InstructionType::RLC(RegistersIndirect::E)),
            0x04 => Some(InstructionType::RLC(RegistersIndirect::H)),
            0x05 => Some(InstructionType::RLC(RegistersIndirect::L)),
            0x06 => Some(InstructionType::RLC(RegistersIndirect::HLI)),
            0x07 => Some(InstructionType::RLC(RegistersIndirect::A)),

            0x08 => Some(InstructionType::RRC(RegistersIndirect::B)),
            0x09 => Some(InstructionType::RRC(RegistersIndirect::C)),
            0x0A => Some(InstructionType::RRC(RegistersIndirect::D)),
            0x0B => Some(InstructionType::RRC(RegistersIndirect::E)),
            0x0C => Some(InstructionType::RRC(RegistersIndirect::H)),
            0x0D => Some(InstructionType::RRC(RegistersIndirect::L)),
            0x0E => Some(InstructionType::RRC(RegistersIndirect::HLI)),
            0x0F => Some(InstructionType::RRC(RegistersIndirect::A)),

            0x10 => Some(InstructionType::RL(RegistersIndirect::B)),
            0x11 => Some(InstructionType::RL(RegistersIndirect::C)),
            0x12 => Some(InstructionType::RL(RegistersIndirect::D)),
            0x13 => Some(InstructionType::RL(RegistersIndirect::E)),
            0x14 => Some(InstructionType::RL(RegistersIndirect::H)),
            0x15 => Some(InstructionType::RL(RegistersIndirect::L)),
            0x16 => Some(InstructionType::RL(RegistersIndirect::HLI)),
            0x17 => Some(InstructionType::RL(RegistersIndirect::A)),

            0x18 => Some(InstructionType::RR(RegistersIndirect::B)),
            0x19 => Some(InstructionType::RR(RegistersIndirect::C)),
            0x1A => Some(InstructionType::RR(RegistersIndirect::D)),
            0x1B => Some(InstructionType::RR(RegistersIndirect::E)),
            0x1C => Some(InstructionType::RR(RegistersIndirect::H)),
            0x1D => Some(InstructionType::RR(RegistersIndirect::L)),
            0x1E => Some(InstructionType::RR(RegistersIndirect::HLI)),
            0x1F => Some(InstructionType::RR(RegistersIndirect::A)),

            0x20 => Some(InstructionType::SLA(RegistersIndirect::B)),
            0x21 => Some(InstructionType::SLA(RegistersIndirect::C)),
            0x22 => Some(InstructionType::SLA(RegistersIndirect::D)),
            0x23 => Some(InstructionType::SLA(RegistersIndirect::E)),
            0x24 => Some(InstructionType::SLA(RegistersIndirect::H)),
            0x25 => Some(InstructionType::SLA(RegistersIndirect::L)),
            0x26 => Some(InstructionType::SLA(RegistersIndirect::HLI)),
            0x27 => Some(InstructionType::SLA(RegistersIndirect::A)),

            0x28 => Some(InstructionType::SRA(RegistersIndirect::B)),
            0x29 => Some(InstructionType::SRA(RegistersIndirect::C)),
            0x2A => Some(InstructionType::SRA(RegistersIndirect::D)),
            0x2B => Some(InstructionType::SRA(RegistersIndirect::E)),
            0x2C => Some(InstructionType::SRA(RegistersIndirect::H)),
            0x2D => Some(InstructionType::SRA(RegistersIndirect::L)),
            0x2E => Some(InstructionType::SRA(RegistersIndirect::HLI)),
            0x2F => Some(InstructionType::SRA(RegistersIndirect::A)),

            0x30 => Some(InstructionType::SWAP(RegistersIndirect::B)),
            0x31 => Some(InstructionType::SWAP(RegistersIndirect::C)),
            0x32 => Some(InstructionType::SWAP(RegistersIndirect::D)),
            0x33 => Some(InstructionType::SWAP(RegistersIndirect::E)),
            0x34 => Some(InstructionType::SWAP(RegistersIndirect::H)),
            0x35 => Some(InstructionType::SWAP(RegistersIndirect::L)),
            0x36 => Some(InstructionType::SWAP(RegistersIndirect::HLI)),
            0x37 => Some(InstructionType::SWAP(RegistersIndirect::A)),

            0x38 => Some(InstructionType::SRL(RegistersIndirect::B)),
            0x39 => Some(InstructionType::SRL(RegistersIndirect::C)),
            0x3A => Some(InstructionType::SRL(RegistersIndirect::D)),
            0x3B => Some(InstructionType::SRL(RegistersIndirect::E)),
            0x3C => Some(InstructionType::SRL(RegistersIndirect::H)),
            0x3D => Some(InstructionType::SRL(RegistersIndirect::L)),
            0x3E => Some(InstructionType::SRL(RegistersIndirect::HLI)),
            0x3F => Some(InstructionType::SRL(RegistersIndirect::A)),

            // BIT
            0x40 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Zero, RegistersIndirect::B))),
            0x41 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Zero, RegistersIndirect::C))),
            0x42 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Zero, RegistersIndirect::D))),
            0x43 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Zero, RegistersIndirect::E))),
            0x44 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Zero, RegistersIndirect::H))),
            0x45 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Zero, RegistersIndirect::L))),
            0x46 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Zero, RegistersIndirect::HLI))),
            0x47 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Zero, RegistersIndirect::A))),

            0x48 => Some(InstructionType::BIT(BitType::Registers(BitTarget::One, RegistersIndirect::B))),
            0x49 => Some(InstructionType::BIT(BitType::Registers(BitTarget::One, RegistersIndirect::C))),
            0x4A => Some(InstructionType::BIT(BitType::Registers(BitTarget::One, RegistersIndirect::D))),
            0x4B => Some(InstructionType::BIT(BitType::Registers(BitTarget::One, RegistersIndirect::E))),
            0x4C => Some(InstructionType::BIT(BitType::Registers(BitTarget::One, RegistersIndirect::H))),
            0x4D => Some(InstructionType::BIT(BitType::Registers(BitTarget::One, RegistersIndirect::L))),
            0x4E => Some(InstructionType::BIT(BitType::Registers(BitTarget::One, RegistersIndirect::HLI))),
            0x4F => Some(InstructionType::BIT(BitType::Registers(BitTarget::One, RegistersIndirect::A))),

            0x50 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Two, RegistersIndirect::B))),
            0x51 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Two, RegistersIndirect::C))),
            0x52 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Two, RegistersIndirect::D))),
            0x53 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Two, RegistersIndirect::E))),
            0x54 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Two, RegistersIndirect::H))),
            0x55 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Two, RegistersIndirect::L))),
            0x56 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Two, RegistersIndirect::HLI))),
            0x57 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Two, RegistersIndirect::A))),
            
            0x58 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Three, RegistersIndirect::B))),
            0x59 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Three, RegistersIndirect::C))),
            0x5A => Some(InstructionType::BIT(BitType::Registers(BitTarget::Three, RegistersIndirect::D))),
            0x5B => Some(InstructionType::BIT(BitType::Registers(BitTarget::Three, RegistersIndirect::E))),
            0x5C => Some(InstructionType::BIT(BitType::Registers(BitTarget::Three, RegistersIndirect::H))),
            0x5D => Some(InstructionType::BIT(BitType::Registers(BitTarget::Three, RegistersIndirect::L))),
            0x5E => Some(InstructionType::BIT(BitType::Registers(BitTarget::Three, RegistersIndirect::HLI))),
            0x5F => Some(InstructionType::BIT(BitType::Registers(BitTarget::Three, RegistersIndirect::A))),

            0x60 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Four, RegistersIndirect::B))),
            0x61 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Four, RegistersIndirect::C))),
            0x62 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Four, RegistersIndirect::D))),
            0x63 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Four, RegistersIndirect::E))),
            0x64 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Four, RegistersIndirect::H))),
            0x65 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Four, RegistersIndirect::L))),
            0x66 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Four, RegistersIndirect::HLI))),
            0x67 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Four, RegistersIndirect::A))),
            
            0x68 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Five, RegistersIndirect::B))),
            0x69 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Five, RegistersIndirect::C))),
            0x6A => Some(InstructionType::BIT(BitType::Registers(BitTarget::Five, RegistersIndirect::D))),
            0x6B => Some(InstructionType::BIT(BitType::Registers(BitTarget::Five, RegistersIndirect::E))),
            0x6C => Some(InstructionType::BIT(BitType::Registers(BitTarget::Five, RegistersIndirect::H))),
            0x6D => Some(InstructionType::BIT(BitType::Registers(BitTarget::Five, RegistersIndirect::L))),
            0x6E => Some(InstructionType::BIT(BitType::Registers(BitTarget::Five, RegistersIndirect::HLI))),
            0x6F => Some(InstructionType::BIT(BitType::Registers(BitTarget::Five, RegistersIndirect::A))),

            0x70 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Six, RegistersIndirect::B))),
            0x71 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Six, RegistersIndirect::C))),
            0x72 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Six, RegistersIndirect::D))),
            0x73 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Six, RegistersIndirect::E))),
            0x74 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Six, RegistersIndirect::H))),
            0x75 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Six, RegistersIndirect::L))),
            0x76 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Six, RegistersIndirect::HLI))),
            0x77 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Six, RegistersIndirect::A))),

            0x78 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Seven, RegistersIndirect::B))),
            0x79 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Seven, RegistersIndirect::C))),
            0x7A => Some(InstructionType::BIT(BitType::Registers(BitTarget::Seven, RegistersIndirect::D))),
            0x7B => Some(InstructionType::BIT(BitType::Registers(BitTarget::Seven, RegistersIndirect::E))),
            0x7C => Some(InstructionType::BIT(BitType::Registers(BitTarget::Seven, RegistersIndirect::H))),
            0x7D => Some(InstructionType::BIT(BitType::Registers(BitTarget::Seven, RegistersIndirect::L))),
            0x7E => Some(InstructionType::BIT(BitType::Registers(BitTarget::Seven, RegistersIndirect::HLI))),
            0x7F => Some(InstructionType::BIT(BitType::Registers(BitTarget::Seven, RegistersIndirect::A))),

            0x80 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::B))),
            0x81 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::C))),
            0x82 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::D))),
            0x83 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::E))),
            0x84 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::H))),
            0x85 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::L))),
            0x86 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::HLI))),
            0x87 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::A))),

            0x88 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::One, RegistersIndirect::B))),
            0x89 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::One, RegistersIndirect::C))),
            0x8A => Some(InstructionType::RES(ResSetType::Registers(BitTarget::One, RegistersIndirect::D))),
            0x8B => Some(InstructionType::RES(ResSetType::Registers(BitTarget::One, RegistersIndirect::E))),
            0x8C => Some(InstructionType::RES(ResSetType::Registers(BitTarget::One, RegistersIndirect::H))),
            0x8D => Some(InstructionType::RES(ResSetType::Registers(BitTarget::One, RegistersIndirect::L))),
            0x8E => Some(InstructionType::RES(ResSetType::Registers(BitTarget::One, RegistersIndirect::HLI))),
            0x8F => Some(InstructionType::RES(ResSetType::Registers(BitTarget::One, RegistersIndirect::A))),

            0x90 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Two, RegistersIndirect::B))),
            0x91 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Two, RegistersIndirect::C))),
            0x92 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Two, RegistersIndirect::D))),
            0x93 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Two, RegistersIndirect::E))),
            0x94 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Two, RegistersIndirect::H))),
            0x95 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Two, RegistersIndirect::L))),
            0x96 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Two, RegistersIndirect::HLI))),
            0x97 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Two, RegistersIndirect::A))),
            
            0x98 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Three, RegistersIndirect::B))),
            0x99 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Three, RegistersIndirect::C))),
            0x9A => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Three, RegistersIndirect::D))),
            0x9B => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Three, RegistersIndirect::E))),
            0x9C => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Three, RegistersIndirect::H))),
            0x9D => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Three, RegistersIndirect::L))),
            0x9E => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Three, RegistersIndirect::HLI))),
            0x9F => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Three, RegistersIndirect::A))),

            0xA0 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Four, RegistersIndirect::B))),
            0xA1 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Four, RegistersIndirect::C))),
            0xA2 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Four, RegistersIndirect::D))),
            0xA3 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Four, RegistersIndirect::E))),
            0xA4 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Four, RegistersIndirect::H))),
            0xA5 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Four, RegistersIndirect::L))),
            0xA6 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Four, RegistersIndirect::HLI))),
            0xA7 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Four, RegistersIndirect::A))),
            
            0xA8 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Five, RegistersIndirect::B))),
            0xA9 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Five, RegistersIndirect::C))),
            0xAA => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Five, RegistersIndirect::D))),
            0xAB => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Five, RegistersIndirect::E))),
            0xAC => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Five, RegistersIndirect::H))),
            0xAD => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Five, RegistersIndirect::L))),
            0xAE => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Five, RegistersIndirect::HLI))),
            0xAF => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Five, RegistersIndirect::A))),

            0xB0 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Six, RegistersIndirect::B))),
            0xB1 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Six, RegistersIndirect::C))),
            0xB2 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Six, RegistersIndirect::D))),
            0xB3 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Six, RegistersIndirect::E))),
            0xB4 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Six, RegistersIndirect::H))),
            0xB5 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Six, RegistersIndirect::L))),
            0xB6 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Six, RegistersIndirect::HLI))),
            0xB7 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Six, RegistersIndirect::A))),

            0xB8 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::B))),
            0xB9 => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::C))),
            0xBA => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::D))),
            0xBB => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::E))),
            0xBC => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::H))),
            0xBD => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::L))),
            0xBE => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::HLI))),
            0xBF => Some(InstructionType::RES(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::A))),

            0xC0 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::B))),
            0xC1 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::C))),
            0xC2 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::D))),
            0xC3 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::E))),
            0xC4 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::H))),
            0xC5 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::L))),
            0xC6 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::HLI))),
            0xC7 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Zero, RegistersIndirect::A))),

            0xC8 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::One, RegistersIndirect::B))),
            0xC9 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::One, RegistersIndirect::C))),
            0xCA => Some(InstructionType::SET(ResSetType::Registers(BitTarget::One, RegistersIndirect::D))),
            0xCB => Some(InstructionType::SET(ResSetType::Registers(BitTarget::One, RegistersIndirect::E))),
            0xCC => Some(InstructionType::SET(ResSetType::Registers(BitTarget::One, RegistersIndirect::H))),
            0xCD => Some(InstructionType::SET(ResSetType::Registers(BitTarget::One, RegistersIndirect::L))),
            0xCE => Some(InstructionType::SET(ResSetType::Registers(BitTarget::One, RegistersIndirect::HLI))),
            0xCF => Some(InstructionType::SET(ResSetType::Registers(BitTarget::One, RegistersIndirect::A))),

            0xD0 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Two, RegistersIndirect::B))),
            0xD1 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Two, RegistersIndirect::C))),
            0xD2 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Two, RegistersIndirect::D))),
            0xD3 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Two, RegistersIndirect::E))),
            0xD4 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Two, RegistersIndirect::H))),
            0xD5 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Two, RegistersIndirect::L))),
            0xD6 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Two, RegistersIndirect::HLI))),
            0xD7 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Two, RegistersIndirect::A))),
            
            0xD8 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Three, RegistersIndirect::B))),
            0xD9 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Three, RegistersIndirect::C))),
            0xDA => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Three, RegistersIndirect::D))),
            0xDB => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Three, RegistersIndirect::E))),
            0xDC => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Three, RegistersIndirect::H))),
            0xDD => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Three, RegistersIndirect::L))),
            0xDE => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Three, RegistersIndirect::HLI))),
            0xDF => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Three, RegistersIndirect::A))),

            0xE0 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Four, RegistersIndirect::B))),
            0xE1 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Four, RegistersIndirect::C))),
            0xE2 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Four, RegistersIndirect::D))),
            0xE3 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Four, RegistersIndirect::E))),
            0xE4 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Four, RegistersIndirect::H))),
            0xE5 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Four, RegistersIndirect::L))),
            0xE6 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Four, RegistersIndirect::HLI))),
            0xE7 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Four, RegistersIndirect::A))),
            
            0xE8 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Five, RegistersIndirect::B))),
            0xE9 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Five, RegistersIndirect::C))),
            0xEA => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Five, RegistersIndirect::D))),
            0xEB => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Five, RegistersIndirect::E))),
            0xEC => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Five, RegistersIndirect::H))),
            0xED => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Five, RegistersIndirect::L))),
            0xEE => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Five, RegistersIndirect::HLI))),
            0xEF => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Five, RegistersIndirect::A))),

            0xF0 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Six, RegistersIndirect::B))),
            0xF1 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Six, RegistersIndirect::C))),
            0xF2 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Six, RegistersIndirect::D))),
            0xF3 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Six, RegistersIndirect::E))),
            0xF4 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Six, RegistersIndirect::H))),
            0xF5 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Six, RegistersIndirect::L))),
            0xF6 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Six, RegistersIndirect::HLI))),
            0xF7 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Six, RegistersIndirect::A))),

            0xF8 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::B))),
            0xF9 => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::C))),
            0xFA => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::D))),
            0xFB => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::E))),
            0xFC => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::H))),
            0xFD => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::L))),
            0xFE => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::HLI))),
            0xFF => Some(InstructionType::SET(ResSetType::Registers(BitTarget::Seven, RegistersIndirect::A))),
            
            _ => /* TODO: Add mapping for rest of InstructionTypes */ None
        }
    }
    
    pub(crate) fn from_byte_not_prefixed(byte: u8) -> Option<InstructionType> {
        match byte {
            // Miscellaneous InstructionTypes
            0x00 => Some(InstructionType::NOP),
            0x76 => Some(InstructionType::HALT),
            0x10 => Some(InstructionType::STOP),
            0x27 => Some(InstructionType::DAA),
            0x37 => Some(InstructionType::SCF),
            0x2F => Some(InstructionType::CPL),
            0x3F => Some(InstructionType::CCF),
            0xF3 => Some(InstructionType::DI),
            0xFB => Some(InstructionType::EI),

            // Rotate InstructionTypes
            0x07 => Some(InstructionType::RLCA),
            0x17 => Some(InstructionType::RLA),
            0x0F => Some(InstructionType::RRCA),
            0x1F => Some(InstructionType::RRA),

            // Stack InstructionTypes
            0xC1 => Some(InstructionType::POP(StackTarget::BC)),
            0xD1 => Some(InstructionType::POP(StackTarget::DE)),
            0xE1 => Some(InstructionType::POP(StackTarget::HL)),
            0xF1 => Some(InstructionType::POP(StackTarget::AF)),
            0xC5 => Some(InstructionType::PUSH(StackTarget::BC)),
            0xD5 => Some(InstructionType::PUSH(StackTarget::DE)),
            0xE5 => Some(InstructionType::PUSH(StackTarget::HL)),
            0xF5 => Some(InstructionType::PUSH(StackTarget::AF)),
            0xF8 => Some(InstructionType::LDHLSPD8),
            0xF9 => Some(InstructionType::LDSPHL),
            0x08 => Some(InstructionType::LDSPA16),

            // Control flow InstructionTypes
            0x18 => Some(InstructionType::JR(JumpTest::Always)),
            0x28 => Some(InstructionType::JR(JumpTest::Zero)),
            0x38 => Some(InstructionType::JR(JumpTest::Carry)),
            0xC0 => Some(InstructionType::RET(JumpTest::NotZero)),
            0xD0 => Some(InstructionType::RET(JumpTest::NotCarry)),
            0xC2 => Some(InstructionType::JP(JumpTest::NotZero)),
            0xD2 => Some(InstructionType::JP(JumpTest::NotCarry)),
            0xC3 => Some(InstructionType::JP(JumpTest::Always)),
            0xC4 => Some(InstructionType::CALL(JumpTest::NotZero)),
            0xD4 => Some(InstructionType::CALL(JumpTest::NotCarry)),
            0xC7 => Some(InstructionType::RST(BitTarget::Zero)),
            0xD7 => Some(InstructionType::RST(BitTarget::Two)),
            0xE7 => Some(InstructionType::RST(BitTarget::Four)),
            0xF7 => Some(InstructionType::RST(BitTarget::Six)),
            0xC8 => Some(InstructionType::RET(JumpTest::Zero)),
            0xD8 => Some(InstructionType::RET(JumpTest::Carry)),
            0xC9 => Some(InstructionType::RET(JumpTest::Always)),
            0xD9 => Some(InstructionType::RETI),
            0xE9 => Some(InstructionType::JPHL),
            0xCA => Some(InstructionType::JP(JumpTest::Zero)),
            0xDA => Some(InstructionType::JP(JumpTest::Carry)),
            0xCC => Some(InstructionType::CALL(JumpTest::Zero)),
            0xDC => Some(InstructionType::CALL(JumpTest::Carry)),
            0xCD => Some(InstructionType::CALL(JumpTest::Always)),
            0xCF => Some(InstructionType::RST(BitTarget::One)),
            0xDF => Some(InstructionType::RST(BitTarget::Three)),
            0xEF => Some(InstructionType::RST(BitTarget::Five)),
            0xFF => Some(InstructionType::RST(BitTarget::Seven)),
            0x20 => Some(InstructionType::JR(JumpTest::NotZero)),
            0x30 => Some(InstructionType::JR(JumpTest::NotCarry)),

            // 16-bit load InstructionTypes
            0x01 => Some(InstructionType::LD(LoadType::Word(WordRegister::BC))),
            0x11 => Some(InstructionType::LD(LoadType::Word(WordRegister::DE))),
            0x21 => Some(InstructionType::LD(LoadType::Word(WordRegister::HL))),
            0x31 => Some(InstructionType::LD(LoadType::Word(WordRegister::SP))),

            // 16-bit Arithmetic/Logic InstructionTypes
            0x09 => Some(InstructionType::ADD16(WordRegister::BC)),
            0x19 => Some(InstructionType::ADD16(WordRegister::DE)),
            0x29 => Some(InstructionType::ADD16(WordRegister::HL)),
            0x39 => Some(InstructionType::ADD16(WordRegister::SP)),
            0x03 => Some(InstructionType::INC16(WordRegister::BC)),
            0x13 => Some(InstructionType::INC16(WordRegister::DE)),
            0x23 => Some(InstructionType::INC16(WordRegister::HL)),
            0x33 => Some(InstructionType::INC16(WordRegister::SP)),
            0x0B => Some(InstructionType::DEC16(WordRegister::BC)),
            0x1B => Some(InstructionType::DEC16(WordRegister::DE)),
            0x2B => Some(InstructionType::DEC16(WordRegister::HL)),
            0x3B => Some(InstructionType::DEC16(WordRegister::SP)),
            0xE8 => Some(InstructionType::ADDSPS8),
            
            // 8-bit load InstructionTypes
            0x02 => Some(InstructionType::LD(LoadType::IndirectFromA(LoadIndirectSource::BC))),
            0x12 => Some(InstructionType::LD(LoadType::IndirectFromA(LoadIndirectSource::DE))),
            0x22 => Some(InstructionType::LD(LoadType::IndirectFromA(LoadIndirectSource::HLInc))),
            0x32 => Some(InstructionType::LD(LoadType::IndirectFromA(LoadIndirectSource::HLDec))),
            0x40 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::B, RegistersIndDir::B))),
            0x41 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::B, RegistersIndDir::C))),
            0x42 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::B, RegistersIndDir::D))),
            0x43 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::B, RegistersIndDir::E))),
            0x44 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::B, RegistersIndDir::H))),
            0x45 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::B, RegistersIndDir::L))),
            0x46 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::B, RegistersIndDir::HLI))),
            0x47 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::B, RegistersIndDir::A))),
            0x48 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::C, RegistersIndDir::B))),
            0x49 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::C, RegistersIndDir::C))),
            0x4A => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::C, RegistersIndDir::D))),
            0x4B => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::C, RegistersIndDir::E))),
            0x4C => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::C, RegistersIndDir::H))),
            0x4D => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::C, RegistersIndDir::L))),
            0x4E => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::C, RegistersIndDir::HLI))),
            0x4F => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::C, RegistersIndDir::A))),

            0x50 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::D, RegistersIndDir::B))),
            0x51 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::D, RegistersIndDir::C))),
            0x52 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::D, RegistersIndDir::D))),
            0x53 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::D, RegistersIndDir::E))),
            0x54 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::D, RegistersIndDir::H))),
            0x55 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::D, RegistersIndDir::L))),
            0x56 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::D, RegistersIndDir::HLI))),
            0x57 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::D, RegistersIndDir::A))),
            0x58 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::E, RegistersIndDir::B))),
            0x59 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::E, RegistersIndDir::C))),
            0x5A => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::E, RegistersIndDir::D))),
            0x5B => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::E, RegistersIndDir::E))),
            0x5C => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::E, RegistersIndDir::H))),
            0x5D => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::E, RegistersIndDir::L))),
            0x5E => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::E, RegistersIndDir::HLI))),
            0x5F => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::E, RegistersIndDir::A))),

            0x60 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::H, RegistersIndDir::B))),
            0x61 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::H, RegistersIndDir::C))),
            0x62 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::H, RegistersIndDir::D))),
            0x63 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::H, RegistersIndDir::E))),
            0x64 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::H, RegistersIndDir::H))),
            0x65 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::H, RegistersIndDir::L))),
            0x66 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::H, RegistersIndDir::HLI))),
            0x67 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::H, RegistersIndDir::A))),
            0x68 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::L, RegistersIndDir::B))),
            0x69 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::L, RegistersIndDir::C))),
            0x6A => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::L, RegistersIndDir::D))),
            0x6B => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::L, RegistersIndDir::E))),
            0x6C => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::L, RegistersIndDir::H))),
            0x6D => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::L, RegistersIndDir::L))),
            0x6E => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::L, RegistersIndDir::HLI))),
            0x6F => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::L, RegistersIndDir::A))),

            0x70 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::HLI, RegistersIndDir::B))),
            0x71 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::HLI, RegistersIndDir::C))),
            0x72 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::HLI, RegistersIndDir::D))),
            0x73 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::HLI, RegistersIndDir::E))),
            0x74 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::HLI, RegistersIndDir::H))),
            0x75 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::HLI, RegistersIndDir::L))),
            0x77 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::HLI, RegistersIndDir::A))),
            0x78 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::A, RegistersIndDir::B))),
            0x79 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::A, RegistersIndDir::C))),
            0x7A => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::A, RegistersIndDir::D))),
            0x7B => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::A, RegistersIndDir::E))),
            0x7C => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::A, RegistersIndDir::H))),
            0x7D => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::A, RegistersIndDir::L))),
            0x7E => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::A, RegistersIndDir::HLI))),
            0x7F => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::A, RegistersIndDir::A))),

            0x06 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::B, RegistersIndDir::D8))),
            0x16 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::D, RegistersIndDir::D8))),
            0x26 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::H, RegistersIndDir::D8))),
            0x36 => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::HLI, RegistersIndDir::D8))),
            0x0E => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::C, RegistersIndDir::D8))),
            0x1E => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::E, RegistersIndDir::D8))),
            0x2E => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::L, RegistersIndDir::D8))),
            0x3E => Some(InstructionType::LD(LoadType::Byte(RegistersIndirect::A, RegistersIndDir::D8))),

            0x0A => Some(InstructionType::LD(LoadType::AFromIndirect(LoadIndirectSource::BC))),
            0x1A => Some(InstructionType::LD(LoadType::AFromIndirect(LoadIndirectSource::DE))),
            0x2A => Some(InstructionType::LD(LoadType::AFromIndirect(LoadIndirectSource::HLInc))),
            0x3A => Some(InstructionType::LD(LoadType::AFromIndirect(LoadIndirectSource::HLDec))),
            0xE0 => Some(InstructionType::LDFF(LoadFFType::AtoFFa8)),
            0xF0 => Some(InstructionType::LDFF(LoadFFType::FFa8toA)),
            0xE2 => Some(InstructionType::LDFF(LoadFFType::AtoFFC)),
            0xF2 => Some(InstructionType::LDFF(LoadFFType::FFCtoA)),
            0xEA => Some(InstructionType::LD(LoadType::DirectFromA)),
            0xFA => Some(InstructionType::LD(LoadType::AFromDirect)),

            // 8-bit arithmetic and logical InstructionTypes
            0x04 => Some(InstructionType::INC(RegistersIndirect::B)),
            0x14 => Some(InstructionType::INC(RegistersIndirect::D)),
            0x24 => Some(InstructionType::INC(RegistersIndirect::H)),
            0x34 => Some(InstructionType::INC(RegistersIndirect::HLI)),

            0x05 => Some(InstructionType::DEC(RegistersIndirect::B)),
            0x15 => Some(InstructionType::DEC(RegistersIndirect::D)),
            0x25 => Some(InstructionType::DEC(RegistersIndirect::H)),
            0x35 => Some(InstructionType::DEC(RegistersIndirect::HLI)),

            0x0C => Some(InstructionType::INC(RegistersIndirect::C)),
            0x1C => Some(InstructionType::INC(RegistersIndirect::E)),
            0x2C => Some(InstructionType::INC(RegistersIndirect::L)),
            0x3C => Some(InstructionType::INC(RegistersIndirect::A)),

            0x0D => Some(InstructionType::DEC(RegistersIndirect::C)),
            0x1D => Some(InstructionType::DEC(RegistersIndirect::E)),
            0x2D => Some(InstructionType::DEC(RegistersIndirect::L)),
            0x3D => Some(InstructionType::DEC(RegistersIndirect::A)),
            
            0x80 => Some(InstructionType::ADD(RegistersIndDir::B)),
            0x81 => Some(InstructionType::ADD(RegistersIndDir::C)),
            0x82 => Some(InstructionType::ADD(RegistersIndDir::D)),
            0x83 => Some(InstructionType::ADD(RegistersIndDir::E)),
            0x84 => Some(InstructionType::ADD(RegistersIndDir::H)),
            0x85 => Some(InstructionType::ADD(RegistersIndDir::L)),
            0x86 => Some(InstructionType::ADD(RegistersIndDir::HLI)),
            0x87 => Some(InstructionType::ADD(RegistersIndDir::A)),

            0x88 => Some(InstructionType::ADC(RegistersIndDir::B)),
            0x89 => Some(InstructionType::ADC(RegistersIndDir::C)),
            0x8A => Some(InstructionType::ADC(RegistersIndDir::D)),
            0x8B => Some(InstructionType::ADC(RegistersIndDir::E)),
            0x8C => Some(InstructionType::ADC(RegistersIndDir::H)),
            0x8D => Some(InstructionType::ADC(RegistersIndDir::L)),
            0x8E => Some(InstructionType::ADC(RegistersIndDir::HLI)),
            0x8F => Some(InstructionType::ADC(RegistersIndDir::A)),
            
            0x90 => Some(InstructionType::SUB(RegistersIndDir::B)),
            0x91 => Some(InstructionType::SUB(RegistersIndDir::C)),
            0x92 => Some(InstructionType::SUB(RegistersIndDir::D)),
            0x93 => Some(InstructionType::SUB(RegistersIndDir::E)),
            0x94 => Some(InstructionType::SUB(RegistersIndDir::H)),
            0x95 => Some(InstructionType::SUB(RegistersIndDir::L)),
            0x96 => Some(InstructionType::SUB(RegistersIndDir::HLI)),
            0x97 => Some(InstructionType::SUB(RegistersIndDir::A)),

            0x98 => Some(InstructionType::SBC(RegistersIndDir::B)),
            0x99 => Some(InstructionType::SBC(RegistersIndDir::C)),
            0x9A => Some(InstructionType::SBC(RegistersIndDir::D)),
            0x9B => Some(InstructionType::SBC(RegistersIndDir::E)),
            0x9C => Some(InstructionType::SBC(RegistersIndDir::H)),
            0x9D => Some(InstructionType::SBC(RegistersIndDir::L)),
            0x9E => Some(InstructionType::SBC(RegistersIndDir::HLI)),
            0x9F => Some(InstructionType::SBC(RegistersIndDir::A)),

            0xA0 => Some(InstructionType::AND(RegistersIndDir::B)),
            0xA1 => Some(InstructionType::AND(RegistersIndDir::C)),
            0xA2 => Some(InstructionType::AND(RegistersIndDir::D)),
            0xA3 => Some(InstructionType::AND(RegistersIndDir::E)),
            0xA4 => Some(InstructionType::AND(RegistersIndDir::H)),
            0xA5 => Some(InstructionType::AND(RegistersIndDir::L)),
            0xA6 => Some(InstructionType::AND(RegistersIndDir::HLI)),
            0xA7 => Some(InstructionType::AND(RegistersIndDir::A)),

            0xA8 => Some(InstructionType::XOR(RegistersIndDir::B)),
            0xA9 => Some(InstructionType::XOR(RegistersIndDir::C)),
            0xAA => Some(InstructionType::XOR(RegistersIndDir::D)),
            0xAB => Some(InstructionType::XOR(RegistersIndDir::E)),
            0xAC => Some(InstructionType::XOR(RegistersIndDir::H)),
            0xAD => Some(InstructionType::XOR(RegistersIndDir::L)),
            0xAE => Some(InstructionType::XOR(RegistersIndDir::HLI)),
            0xAF => Some(InstructionType::XOR(RegistersIndDir::A)),

            0xB0 => Some(InstructionType::OR(RegistersIndDir::B)),
            0xB1 => Some(InstructionType::OR(RegistersIndDir::C)),
            0xB2 => Some(InstructionType::OR(RegistersIndDir::D)),
            0xB3 => Some(InstructionType::OR(RegistersIndDir::E)),
            0xB4 => Some(InstructionType::OR(RegistersIndDir::H)),
            0xB5 => Some(InstructionType::OR(RegistersIndDir::L)),
            0xB6 => Some(InstructionType::OR(RegistersIndDir::HLI)),
            0xB7 => Some(InstructionType::OR(RegistersIndDir::A)),

            0xB8 => Some(InstructionType::CP(RegistersIndDir::B)),
            0xB9 => Some(InstructionType::CP(RegistersIndDir::C)),
            0xBA => Some(InstructionType::CP(RegistersIndDir::D)),
            0xBB => Some(InstructionType::CP(RegistersIndDir::E)),
            0xBC => Some(InstructionType::CP(RegistersIndDir::H)),
            0xBD => Some(InstructionType::CP(RegistersIndDir::L)),
            0xBE => Some(InstructionType::CP(RegistersIndDir::HLI)),
            0xBF => Some(InstructionType::CP(RegistersIndDir::A)),

            0xC6 => Some(InstructionType::ADD(RegistersIndDir::D8)),
            0xD6 => Some(InstructionType::SUB(RegistersIndDir::D8)),
            0xE6 => Some(InstructionType::AND(RegistersIndDir::D8)),
            0xF6 => Some(InstructionType::OR(RegistersIndDir::D8)),

            0xCE => Some(InstructionType::ADC(RegistersIndDir::D8)),
            0xDE => Some(InstructionType::SBC(RegistersIndDir::D8)),
            0xEE => Some(InstructionType::XOR(RegistersIndDir::D8)),
            0xFE => Some(InstructionType::CP(RegistersIndDir::D8)),

            // Invalid
            0xD3 => None,
            0xE3 => None,
            0xE4 => None,
            0xF4 => None,
            0xCB => None,
            0xDB => None,
            0xEB => None,
            0xEC => None,
            0xFC => None,
            0xDD => None,
            0xED => None,
            0xFD => None,
        }
    }
}