#[derive(Debug)]
pub(super) struct Registers {
    pub(super) a: u8, // Accumulators
    pub(super) b: u8,
    pub(super) c: u8,
    pub(super) d: u8,
    pub(super) e: u8,
    pub(super) flags: FlagsRegister,
    pub(super) h: u8,
    pub(super) l: u8,
}
#[derive(Clone, Debug)]
pub(super) struct FlagsRegister {
    pub(super) zero: bool,
    pub(super) subtract: bool,
    pub(super) half_carry: bool,
    pub(super) carry: bool
}

impl Registers {
    pub(super) fn new() -> Registers {
        Registers { a: 0b0, 
                    b: 0b0, 
                    c: 0b0, 
                    d: 0b0, 
                    e: 0b0, 
                    flags: FlagsRegister {  zero: false, 
                                            subtract: false, 
                                            half_carry: false, 
                                            carry: false }, 
                    h: 0b0, 
                    l: 0b0 
                }
    }

    pub(super) fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }
    
    pub(super) fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub(super) fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub(super) fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub(super) fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub(super) fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

    pub(super) fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | u8::from(self.flags.clone()) as u16
    }

    pub(super) fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.flags = FlagsRegister::from((value & 0xFF) as u8);
    }
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl std::convert::From<FlagsRegister> for u8  {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
        (if flag.subtract   { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}