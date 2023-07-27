pub enum Instruction {
    // 8-bit arithmetic and logical instructions
    ADD(ArithmeticTarget),
    // 8-bit load instructions
    LOAD(LoadTarget),
    LD(LoadType),
    // Jump instructions
    JP(JumpTest),
}

pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
}

pub enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}
pub enum LoadTarget {
    A, B, C, D, E, H, L,
}
pub enum LoadByteTarget {
    A, B, C, D, E, H, L, HLI
}
pub enum LoadByteSource {
    A, B, C, D, E, H, L, D8, HLI
}
pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
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
            _ => /* TODO: Add mapping for rest of instructions */ None
        }
    }
    
    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x80 => Some(Instruction::ADD(ArithmeticTarget::A)),
            
            _ => /* TODO: Add mapping for rest of instructions */ None
        }
    }
}