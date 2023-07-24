struct Registers {
    a: u8, // Accumulators
    b: u8, // Flags
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
  }

impl Registers {
    pub fn new() -> Registers {
        Registers { a: 0b0, b: 0b0, c: 0b0, d: 0b0, e: 0b0, f: 0b0, h: 0b0, l: 0b0 }
    }
}
pub struct CPU {
    regs: Registers,
    sp: u16,
    pc: u16
}

impl CPU {
    pub fn new() -> CPU {
        CPU { regs: Registers::new(), sp: 0b0, pc: 0b0 }
    }
}