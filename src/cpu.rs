mod registers;
mod instructions;
mod tests;

use crate::rom::ROM;
use instructions::*;

type ProgramCounter = u16;
type StackPointer = u16;
type Address = u16;
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
        let data = [0; 0xFFFF];

        MemoryBus { memory: data }
    }

    fn read_byte(&self, address: Address) -> u8 {
        self.memory[address as usize]
    }

    fn write_byte(&mut self, address: Address, byte: u8) {
        self.memory[address as usize] = byte;
    }
}

impl CPU {
    pub fn new(boot: ROM) -> CPU {
        let mut membus = MemoryBus::new();

        // Loading the boot ROM data into memory
        for addr in 0..boot.size() {
            membus.write_byte(addr, boot.read_byte(addr))
        } 

        CPU { regs: Registers::new(), sp: 0b0, pc: 0b0, bus: membus }
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
            },
            Instruction::LD(load_type) => {
                match load_type {
                  LoadType::Byte(target, source) => {
                    let source_value = match source {
                      LoadByteSource::A => self.regs.a,
                      LoadByteSource::D8 => self.read_next_byte(),
                      LoadByteSource::HLI => self.bus.read_byte(self.regs.get_hl()),
                      _ => { panic!("TODO: implement other sources") }
                    };
                    match target {
                      LoadByteTarget::A => self.regs.a = source_value,
                      LoadByteTarget::HLI => self.bus.write_byte(self.regs.get_hl(), source_value),
                      _ => { panic!("TODO: implement other targets") }
                    };
                    match source {
                      LoadByteSource::D8  => self.pc.wrapping_add(2),
                      _                   => self.pc.wrapping_add(1),
                    }
                  }
                  _ => { panic!("TODO: implement other load types") }
                }
            },
            Instruction::JP(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.regs.f.zero,
                    JumpTest::NotCarry => !self.regs.f.carry,
                    JumpTest::Zero => self.regs.f.zero,
                    JumpTest::Carry => self.regs.f.carry,
                    JumpTest::Always => true
                };
                self.jump(jump_condition)
            }
            _ => { /* TODO: support more instructions */ self.pc }
        }

        

    }

    fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc+1)
    }

    fn jump(&self, should_jump: bool) -> Address {
        if should_jump {
            // Gameboy is little endian so read pc + 2 as most significant bit
            // and pc + 1 as least significant bit
            let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            (most_significant_byte << 8) | least_significant_byte
        } else {
            // If we don't jump we need to still move the program
            // counter forward by 3 since the jump instruction is
            // 3 bytes wide (1 byte for tag and 2 bytes for jump address)
            self.pc.wrapping_add(3)
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