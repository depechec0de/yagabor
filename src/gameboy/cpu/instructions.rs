use crate::gameboy::ClockCycles;

use crate::gameboy::cpu::cpu::ProgramCounter;

#[derive(Debug, Clone)]
pub(super) struct Instruction {
    pub(super) op: InstructionType,
    pub(super) size: InstructionSize,
    pub(super) payload: Option<u16>
}

impl Instruction {
    pub(super) fn size_bytes(&self) -> u16 {
        match self.size {
            InstructionSize::OneByte => 1,
            InstructionSize::TwoBytes => 2,
            InstructionSize::ThreeBytes => 3,
        }
    }

    // An Instruction can length 3 bytes max 
    pub(super) fn parse_instruction(inst_byte: u8, byte0: u8, byte1: u8) -> Option<Instruction> {
        let prefixed = inst_byte == 0xCB;
        let mut instruction_byte = inst_byte;
        if prefixed {
            instruction_byte = byte0;
        }

        let inst_type: Option<InstructionType>;

        if prefixed {
            inst_type = InstructionType::from_byte_prefixed(instruction_byte)
        } else {
            inst_type = InstructionType::from_byte_not_prefixed(instruction_byte)
        }

        if let Some(op) = inst_type {
            let size = op.size();
            let payload = match op.size() {
                InstructionSize::OneByte => None,
                InstructionSize::TwoBytes => Some(byte0 as u16),
                InstructionSize::ThreeBytes => Some(((byte0 as u16) << 8) | byte1 as u16),
            };

            Some(Instruction{op, size, payload})
        }else{
            None
        }        
    }
}

#[derive(Debug, Clone)]
pub(super) enum InstructionSize {
    OneByte,
    TwoBytes,
    ThreeBytes
}

#[derive(Debug, Clone)]
pub(super) enum InstructionType {
    NOP,
    HALT,
    STOP,
    SCF,
    DAA,
    CCF,
    CPL,
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
    ADDSP8,
    // 16-bit Arithmetic/Logic instructions
    ADD16(WordRegister),
    INC16(WordRegister),
    DEC16(WordRegister),
    // 8-bit load instructions
    LD(LoadType),
    LDSIG,
    LDSPHL,
    LDFF(LoadFFType),
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
    RLC(PrefixTarget),
    BIT(BitType),
    RL(IncDecTarget),
    RLA,
    RLAC,
    RRA,
    RRCA
}

#[derive(Clone, Debug)]
pub(super) enum BitType {
    Registers(BitTarget, BitSource),    
}

#[derive(Clone, Debug)]
pub(super) enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
}

#[derive(Clone, Debug)]
pub(super) enum ArithmeticTarget {
    A, B, C, D, E, H, L, HLI, D8
}

#[derive(Clone, Debug)]
pub(super) enum IncDecTarget {
    A, B, C, D, E, H, L, HLI
}

#[derive(Clone, Debug)]
pub(super) enum PrefixTarget {
    A, B, C, D, E, H, L,
}

#[derive(Clone, Debug)]
pub(super) enum WordRegister {
    BC, DE, HL, SP
}

#[derive(Clone, Debug)]
pub(super) enum StackTarget {
    BC, DE, HL, AF
}

#[derive(Clone, Debug)]
pub(super) enum LoadIndirectSource {
    BC, DE, HLInc, HLDec
}

#[derive(Clone, Debug)]
pub(super) enum LoadByteTarget {
    A, B, C, D, E, H, L, HLI
}

#[derive(Clone, Debug)]
pub(super) enum LoadByteSource {
    A, B, C, D, E, H, L, D8, HLI
}

#[derive(Clone, Debug)]
pub(super) enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
    Word(WordRegister),
    AFromIndirect(LoadIndirectSource),
    IndirectFromA(LoadIndirectSource),
    AFromDirect,
    DirectFromA,
}

#[derive(Clone, Debug)]
pub(super) enum LoadFFType {
    AtoFFC,
    FFCtoA,
    FFa8toA,
    AtoFFa8
}

#[derive(Clone, Debug)]
pub(super) enum  BitSource {
    A, B, C, D, E, H, L, HLI
}

#[derive(Clone, Debug)]
pub(super) enum BitTarget {
    Zero, One, Two, Three, Four, Five, Six, Seven
}

impl InstructionType {
    pub(super) fn size(&self) -> InstructionSize {
        match self {
            InstructionType::NOP => InstructionSize::OneByte,
            InstructionType::HALT => InstructionSize::OneByte,
            InstructionType::STOP => InstructionSize::TwoBytes,
            InstructionType::SCF => InstructionSize::OneByte,
            InstructionType::CCF => InstructionSize::OneByte,
            InstructionType::CPL => InstructionSize::OneByte,
            InstructionType::ADD(atarget) => match atarget { ArithmeticTarget::D8 => InstructionSize::TwoBytes, _ => InstructionSize::OneByte },
            InstructionType::ADC(atarget) => match atarget { ArithmeticTarget::D8 => InstructionSize::TwoBytes, _ => InstructionSize::OneByte },
            InstructionType::INC(_) => InstructionSize::OneByte,
            InstructionType::DEC(_) => InstructionSize::OneByte,
            InstructionType::ADD16(_) => InstructionSize::OneByte,
            InstructionType::INC16(_) => InstructionSize::OneByte,
            InstructionType::DEC16(_) => InstructionSize::OneByte,
            InstructionType::ADDSP8 => InstructionSize::TwoBytes,
            InstructionType::SUB(atarget) => match atarget { ArithmeticTarget::D8 => InstructionSize::TwoBytes, _ => InstructionSize::OneByte },
            InstructionType::SBC(atarget) => match atarget { ArithmeticTarget::D8 => InstructionSize::TwoBytes, _ => InstructionSize::OneByte },
            InstructionType::AND(atarget) => match atarget { ArithmeticTarget::D8 => InstructionSize::TwoBytes, _ => InstructionSize::OneByte },
            InstructionType::XOR(atarget) => match atarget { ArithmeticTarget::D8 => InstructionSize::TwoBytes, _ => InstructionSize::OneByte },
            InstructionType::OR(atarget) => match atarget { ArithmeticTarget::D8 => InstructionSize::TwoBytes, _ => InstructionSize::OneByte },
            InstructionType::CP(atarget) => match atarget { ArithmeticTarget::D8 => InstructionSize::TwoBytes, _ => InstructionSize::OneByte },
            InstructionType::LD(load_type) => match load_type {
                                                        LoadType::AFromDirect => InstructionSize::ThreeBytes,
                                                        LoadType::DirectFromA => InstructionSize::ThreeBytes,
                                                        LoadType::Byte(_, source) => match source {
                                                                                                            LoadByteSource::D8 => InstructionSize::TwoBytes,
                                                                                                            _ => InstructionSize::OneByte
                                                                                                      },
                                                        LoadType::Word(_) => InstructionSize::ThreeBytes,
                                                        LoadType::AFromIndirect(_) => InstructionSize::OneByte,
                                                        LoadType::IndirectFromA(_) => InstructionSize::OneByte,
                                                    },
            InstructionType::LDSIG => InstructionSize::TwoBytes,
            InstructionType::LDSPHL => InstructionSize::OneByte,
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
            InstructionType::RLA => InstructionSize::OneByte,
            InstructionType::RLC(_) => InstructionSize::TwoBytes,
            InstructionType::RLAC => InstructionSize::OneByte,
            InstructionType::RRA => InstructionSize::OneByte,
            InstructionType::RRCA => InstructionSize::OneByte,
        }

    }

    pub(super) fn from_byte_prefixed(byte: u8) -> Option<InstructionType> {
        match byte {
            0x00 => Some(InstructionType::RLC(PrefixTarget::B)),
            0x01 => Some(InstructionType::RLC(PrefixTarget::C)),
            0x02 => Some(InstructionType::RLC(PrefixTarget::D)),
            0x03 => Some(InstructionType::RLC(PrefixTarget::E)),
            0x04 => Some(InstructionType::RLC(PrefixTarget::H)),
            0x05 => Some(InstructionType::RLC(PrefixTarget::L)),
            0x06 => None, // TODO
            0x07 => Some(InstructionType::RLC(PrefixTarget::A)),

            // RL
            0x10 => Some(InstructionType::RL(IncDecTarget::B)),
            0x11 => Some(InstructionType::RL(IncDecTarget::C)),
            0x12 => Some(InstructionType::RL(IncDecTarget::D)),
            0x13 => Some(InstructionType::RL(IncDecTarget::E)),
            0x14 => Some(InstructionType::RL(IncDecTarget::H)),
            0x15 => Some(InstructionType::RL(IncDecTarget::L)),
            0x16 => Some(InstructionType::RL(IncDecTarget::HLI)),
            0x17 => Some(InstructionType::RL(IncDecTarget::A)),

            // BIT
            0x40 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Zero, BitSource::B))),
            0x41 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Zero, BitSource::C))),
            0x42 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Zero, BitSource::D))),
            0x43 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Zero, BitSource::E))),
            0x44 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Zero, BitSource::H))),
            0x45 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Zero, BitSource::L))),
            0x46 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Zero, BitSource::HLI))),
            0x47 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Zero, BitSource::A))),

            0x48 => Some(InstructionType::BIT(BitType::Registers(BitTarget::One, BitSource::B))),
            0x49 => Some(InstructionType::BIT(BitType::Registers(BitTarget::One, BitSource::C))),
            0x4A => Some(InstructionType::BIT(BitType::Registers(BitTarget::One, BitSource::D))),
            0x4B => Some(InstructionType::BIT(BitType::Registers(BitTarget::One, BitSource::E))),
            0x4C => Some(InstructionType::BIT(BitType::Registers(BitTarget::One, BitSource::H))),
            0x4D => Some(InstructionType::BIT(BitType::Registers(BitTarget::One, BitSource::L))),
            0x4E => Some(InstructionType::BIT(BitType::Registers(BitTarget::One, BitSource::HLI))),
            0x4F => Some(InstructionType::BIT(BitType::Registers(BitTarget::One, BitSource::A))),

            0x50 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Two, BitSource::B))),
            0x51 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Two, BitSource::C))),
            0x52 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Two, BitSource::D))),
            0x53 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Two, BitSource::E))),
            0x54 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Two, BitSource::H))),
            0x55 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Two, BitSource::L))),
            0x56 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Two, BitSource::HLI))),
            0x57 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Two, BitSource::A))),
            
            0x58 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Three, BitSource::B))),
            0x59 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Three, BitSource::C))),
            0x5A => Some(InstructionType::BIT(BitType::Registers(BitTarget::Three, BitSource::D))),
            0x5B => Some(InstructionType::BIT(BitType::Registers(BitTarget::Three, BitSource::E))),
            0x5C => Some(InstructionType::BIT(BitType::Registers(BitTarget::Three, BitSource::H))),
            0x5D => Some(InstructionType::BIT(BitType::Registers(BitTarget::Three, BitSource::L))),
            0x5E => Some(InstructionType::BIT(BitType::Registers(BitTarget::Three, BitSource::HLI))),
            0x5F => Some(InstructionType::BIT(BitType::Registers(BitTarget::Three, BitSource::A))),

            0x60 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Four, BitSource::B))),
            0x61 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Four, BitSource::C))),
            0x62 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Four, BitSource::D))),
            0x63 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Four, BitSource::E))),
            0x64 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Four, BitSource::H))),
            0x65 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Four, BitSource::L))),
            0x66 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Four, BitSource::HLI))),
            0x67 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Four, BitSource::A))),
            
            0x68 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Five, BitSource::B))),
            0x69 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Five, BitSource::C))),
            0x6A => Some(InstructionType::BIT(BitType::Registers(BitTarget::Five, BitSource::D))),
            0x6B => Some(InstructionType::BIT(BitType::Registers(BitTarget::Five, BitSource::E))),
            0x6C => Some(InstructionType::BIT(BitType::Registers(BitTarget::Five, BitSource::H))),
            0x6D => Some(InstructionType::BIT(BitType::Registers(BitTarget::Five, BitSource::L))),
            0x6E => Some(InstructionType::BIT(BitType::Registers(BitTarget::Five, BitSource::HLI))),
            0x6F => Some(InstructionType::BIT(BitType::Registers(BitTarget::Five, BitSource::A))),

            0x70 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Six, BitSource::B))),
            0x71 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Six, BitSource::C))),
            0x72 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Six, BitSource::D))),
            0x73 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Six, BitSource::E))),
            0x74 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Six, BitSource::H))),
            0x75 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Six, BitSource::L))),
            0x76 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Six, BitSource::HLI))),
            0x77 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Six, BitSource::A))),

            0x78 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Seven, BitSource::B))),
            0x79 => Some(InstructionType::BIT(BitType::Registers(BitTarget::Seven, BitSource::C))),
            0x7A => Some(InstructionType::BIT(BitType::Registers(BitTarget::Seven, BitSource::D))),
            0x7B => Some(InstructionType::BIT(BitType::Registers(BitTarget::Seven, BitSource::E))),
            0x7C => Some(InstructionType::BIT(BitType::Registers(BitTarget::Seven, BitSource::H))),
            0x7D => Some(InstructionType::BIT(BitType::Registers(BitTarget::Seven, BitSource::L))),
            0x7E => Some(InstructionType::BIT(BitType::Registers(BitTarget::Seven, BitSource::HLI))),
            0x7F => Some(InstructionType::BIT(BitType::Registers(BitTarget::Seven, BitSource::A))),
            _ => /* TODO: Add mapping for rest of InstructionTypes */ None
        }
    }
    
    pub(super) fn from_byte_not_prefixed(byte: u8) -> Option<InstructionType> {
        match byte {
            // Miscellaneous InstructionTypes
            0x00 => Some(InstructionType::NOP),
            0x76 => Some(InstructionType::HALT),
            0x10 => Some(InstructionType::STOP),
            0x27 => Some(InstructionType::DAA),
            0x37 => Some(InstructionType::SCF),
            0x2F => Some(InstructionType::CPL),
            0x3F => Some(InstructionType::CCF),
            0xF3 => None,
            0xFB => None,

            // Rotate InstructionTypes
            0x07 => Some(InstructionType::RLAC),
            0x17 => Some(InstructionType::RLA),
            0x0F => None,
            0x1F => None,

            // Stack InstructionTypes
            0xC1 => Some(InstructionType::POP(StackTarget::BC)),
            0xD1 => Some(InstructionType::POP(StackTarget::DE)),
            0xE1 => Some(InstructionType::POP(StackTarget::HL)),
            0xF1 => Some(InstructionType::POP(StackTarget::AF)),
            0xC5 => Some(InstructionType::PUSH(StackTarget::BC)),
            0xD5 => Some(InstructionType::PUSH(StackTarget::DE)),
            0xE5 => Some(InstructionType::PUSH(StackTarget::HL)),
            0xF5 => Some(InstructionType::PUSH(StackTarget::AF)),
            0xF8 => Some(InstructionType::LDSIG),
            0xF9 => Some(InstructionType::LDSPHL),
            0x08 => None,

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
            0xE8 => Some(InstructionType::ADDSP8),
            
            // 8-bit load InstructionTypes
            0x02 => Some(InstructionType::LD(LoadType::IndirectFromA(LoadIndirectSource::BC))),
            0x12 => Some(InstructionType::LD(LoadType::IndirectFromA(LoadIndirectSource::DE))),
            0x22 => Some(InstructionType::LD(LoadType::IndirectFromA(LoadIndirectSource::HLInc))),
            0x32 => Some(InstructionType::LD(LoadType::IndirectFromA(LoadIndirectSource::HLDec))),
            0x40 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::B))),
            0x41 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::C))),
            0x42 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D))),
            0x43 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::E))),
            0x44 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::H))),
            0x45 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::L))),
            0x46 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::HLI))),
            0x47 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::A))),
            0x48 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::B))),
            0x49 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::C))),
            0x4A => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D))),
            0x4B => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::E))),
            0x4C => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::H))),
            0x4D => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::L))),
            0x4E => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::HLI))),
            0x4F => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::A))),

            0x50 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::B))),
            0x51 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::C))),
            0x52 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D))),
            0x53 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::E))),
            0x54 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::H))),
            0x55 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::L))),
            0x56 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::HLI))),
            0x57 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::A))),
            0x58 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::B))),
            0x59 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::C))),
            0x5A => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D))),
            0x5B => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::E))),
            0x5C => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::H))),
            0x5D => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::L))),
            0x5E => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::HLI))),
            0x5F => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::A))),

            0x60 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::B))),
            0x61 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::C))),
            0x62 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::D))),
            0x63 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::E))),
            0x64 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::H))),
            0x65 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::L))),
            0x66 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::HLI))),
            0x67 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::A))),
            0x68 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::B))),
            0x69 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::C))),
            0x6A => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D))),
            0x6B => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::E))),
            0x6C => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::H))),
            0x6D => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::L))),
            0x6E => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::HLI))),
            0x6F => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::A))),

            0x70 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::B))),
            0x71 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::C))),
            0x72 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::D))),
            0x73 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::E))),
            0x74 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::H))),
            0x75 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::L))),
            0x77 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::A))),
            0x78 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::B))),
            0x79 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::C))),
            0x7A => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D))),
            0x7B => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::E))),
            0x7C => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::H))),
            0x7D => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::L))),
            0x7E => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::HLI))),
            0x7F => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::A))),

            0x06 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D8))),
            0x16 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D8))),
            0x26 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::D8))),
            0x36 => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::D8))),
            0x0E => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D8))),
            0x1E => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D8))),
            0x2E => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D8))),
            0x3E => Some(InstructionType::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D8))),

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
            0x04 => Some(InstructionType::INC(IncDecTarget::B)),
            0x14 => Some(InstructionType::INC(IncDecTarget::D)),
            0x24 => Some(InstructionType::INC(IncDecTarget::H)),
            0x34 => Some(InstructionType::INC(IncDecTarget::HLI)),

            0x05 => Some(InstructionType::DEC(IncDecTarget::B)),
            0x15 => Some(InstructionType::DEC(IncDecTarget::D)),
            0x25 => Some(InstructionType::DEC(IncDecTarget::H)),
            0x35 => Some(InstructionType::DEC(IncDecTarget::HLI)),

            0x0C => Some(InstructionType::INC(IncDecTarget::C)),
            0x1C => Some(InstructionType::INC(IncDecTarget::E)),
            0x2C => Some(InstructionType::INC(IncDecTarget::L)),
            0x3C => Some(InstructionType::INC(IncDecTarget::A)),

            0x0D => Some(InstructionType::DEC(IncDecTarget::C)),
            0x1D => Some(InstructionType::DEC(IncDecTarget::E)),
            0x2D => Some(InstructionType::DEC(IncDecTarget::L)),
            0x3D => Some(InstructionType::DEC(IncDecTarget::A)),
            
            0x80 => Some(InstructionType::ADD(ArithmeticTarget::B)),
            0x81 => Some(InstructionType::ADD(ArithmeticTarget::C)),
            0x82 => Some(InstructionType::ADD(ArithmeticTarget::D)),
            0x83 => Some(InstructionType::ADD(ArithmeticTarget::E)),
            0x84 => Some(InstructionType::ADD(ArithmeticTarget::H)),
            0x85 => Some(InstructionType::ADD(ArithmeticTarget::L)),
            0x86 => Some(InstructionType::ADD(ArithmeticTarget::HLI)),
            0x87 => Some(InstructionType::ADD(ArithmeticTarget::A)),

            0x88 => Some(InstructionType::ADC(ArithmeticTarget::B)),
            0x89 => Some(InstructionType::ADC(ArithmeticTarget::C)),
            0x8A => Some(InstructionType::ADC(ArithmeticTarget::D)),
            0x8B => Some(InstructionType::ADC(ArithmeticTarget::E)),
            0x8C => Some(InstructionType::ADC(ArithmeticTarget::H)),
            0x8D => Some(InstructionType::ADC(ArithmeticTarget::L)),
            0x8E => Some(InstructionType::ADC(ArithmeticTarget::HLI)),
            0x8F => Some(InstructionType::ADC(ArithmeticTarget::A)),
            
            0x90 => Some(InstructionType::SUB(ArithmeticTarget::B)),
            0x91 => Some(InstructionType::SUB(ArithmeticTarget::C)),
            0x92 => Some(InstructionType::SUB(ArithmeticTarget::D)),
            0x93 => Some(InstructionType::SUB(ArithmeticTarget::E)),
            0x94 => Some(InstructionType::SUB(ArithmeticTarget::H)),
            0x95 => Some(InstructionType::SUB(ArithmeticTarget::L)),
            0x96 => Some(InstructionType::SUB(ArithmeticTarget::HLI)),
            0x97 => Some(InstructionType::SUB(ArithmeticTarget::A)),

            0x98 => Some(InstructionType::SBC(ArithmeticTarget::B)),
            0x99 => Some(InstructionType::SBC(ArithmeticTarget::C)),
            0x9A => Some(InstructionType::SBC(ArithmeticTarget::D)),
            0x9B => Some(InstructionType::SBC(ArithmeticTarget::E)),
            0x9C => Some(InstructionType::SBC(ArithmeticTarget::H)),
            0x9D => Some(InstructionType::SBC(ArithmeticTarget::L)),
            0x9E => Some(InstructionType::SBC(ArithmeticTarget::HLI)),
            0x9F => Some(InstructionType::SBC(ArithmeticTarget::A)),

            0xA0 => Some(InstructionType::AND(ArithmeticTarget::B)),
            0xA1 => Some(InstructionType::AND(ArithmeticTarget::C)),
            0xA2 => Some(InstructionType::AND(ArithmeticTarget::D)),
            0xA3 => Some(InstructionType::AND(ArithmeticTarget::E)),
            0xA4 => Some(InstructionType::AND(ArithmeticTarget::H)),
            0xA5 => Some(InstructionType::AND(ArithmeticTarget::L)),
            0xA6 => Some(InstructionType::AND(ArithmeticTarget::HLI)),
            0xA7 => Some(InstructionType::AND(ArithmeticTarget::A)),

            0xA8 => Some(InstructionType::XOR(ArithmeticTarget::B)),
            0xA9 => Some(InstructionType::XOR(ArithmeticTarget::C)),
            0xAA => Some(InstructionType::XOR(ArithmeticTarget::D)),
            0xAB => Some(InstructionType::XOR(ArithmeticTarget::E)),
            0xAC => Some(InstructionType::XOR(ArithmeticTarget::H)),
            0xAD => Some(InstructionType::XOR(ArithmeticTarget::L)),
            0xAE => Some(InstructionType::XOR(ArithmeticTarget::HLI)),
            0xAF => Some(InstructionType::XOR(ArithmeticTarget::A)),

            0xB0 => Some(InstructionType::OR(ArithmeticTarget::B)),
            0xB1 => Some(InstructionType::OR(ArithmeticTarget::C)),
            0xB2 => Some(InstructionType::OR(ArithmeticTarget::D)),
            0xB3 => Some(InstructionType::OR(ArithmeticTarget::E)),
            0xB4 => Some(InstructionType::OR(ArithmeticTarget::H)),
            0xB5 => Some(InstructionType::OR(ArithmeticTarget::L)),
            0xB6 => Some(InstructionType::OR(ArithmeticTarget::HLI)),
            0xB7 => Some(InstructionType::OR(ArithmeticTarget::A)),

            0xB8 => Some(InstructionType::CP(ArithmeticTarget::B)),
            0xB9 => Some(InstructionType::CP(ArithmeticTarget::C)),
            0xBA => Some(InstructionType::CP(ArithmeticTarget::D)),
            0xBB => Some(InstructionType::CP(ArithmeticTarget::E)),
            0xBC => Some(InstructionType::CP(ArithmeticTarget::H)),
            0xBD => Some(InstructionType::CP(ArithmeticTarget::L)),
            0xBE => Some(InstructionType::CP(ArithmeticTarget::HLI)),
            0xBF => Some(InstructionType::CP(ArithmeticTarget::A)),

            0xC6 => Some(InstructionType::ADD(ArithmeticTarget::D8)),
            0xD6 => Some(InstructionType::SUB(ArithmeticTarget::D8)),
            0xE6 => Some(InstructionType::AND(ArithmeticTarget::D8)),
            0xF6 => Some(InstructionType::OR(ArithmeticTarget::D8)),

            0xCE => Some(InstructionType::ADC(ArithmeticTarget::D8)),
            0xDE => Some(InstructionType::SBC(ArithmeticTarget::D8)),
            0xEE => Some(InstructionType::XOR(ArithmeticTarget::D8)),
            0xFE => Some(InstructionType::CP(ArithmeticTarget::D8)),
            _ => None
        }
    }
}