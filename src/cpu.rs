mod registers;
mod instructions;
mod tests;

use instructions::{Instruction, ArithmeticTarget};

type ProgramCounter = u16;
type StackPointer = u16;
pub struct CPU {
    regs: Registers,
    sp: StackPointer,
    pc: ProgramCounter,
    bus: MemoryBus
}

pub struct Registers {
    a: u8, // Accumulators
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagsRegister,
    h: u8,
    l: u8,
}
struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool
}

struct MemoryBus {
    memory: [u8; 0xFFFF]
}

impl MemoryBus {
    pub fn new() -> MemoryBus {
        MemoryBus { memory: [0; 0xFFFF] }
    }
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}

impl CPU {
    pub fn new() -> CPU {
        CPU { regs: Registers::new(), sp: 0b0, pc: 0b0, bus: MemoryBus::new() }
    }

    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
    if prefixed {
        instruction_byte = self.bus.read_byte(self.pc + 1);
    }

    let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
        self.execute(instruction)
    } else {
        let description = format!("0x{}{:x}", if prefixed { "cb" } else { "" }, instruction_byte);
        panic!("Unkown instruction found for: {}", description)
    };

    self.pc = next_pc;
    }

    // Returns the next PC to execute
    fn execute(&mut self, instruction: Instruction) -> ProgramCounter {
    match instruction {
        Instruction::ADD(target) => {
        match target {
            ArithmeticTarget::B => {
            let value = self.regs.b;
            let new_value = self.add(value);
            self.regs.a = new_value;
            self.pc.wrapping_add(1)
            }
            _ => { /* TODO: support more targets */ self.pc }
        }
        }
        _ => { /* TODO: support more instructions */ self.pc }
    }

    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.regs.a.overflowing_add(value);
        self.regs.f.zero = new_value == 0;
        self.regs.f.subtract = false;
        self.regs.f.carry = did_overflow;
        // Half Carry is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble.
        self.regs.f.half_carry = (self.regs.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }
}