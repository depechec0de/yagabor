mod registers;
mod instructions;
mod tests;

use registers::Registers;
use instructions::{Instruction, ArithmeticTarget};
pub struct CPU {
    regs: Registers,
    flags: u8,
    sp: u16,
    pc: u16
}

impl CPU {
    pub fn new() -> CPU {
        CPU { regs: Registers::new(), flags: 0b0, sp: 0b0, pc: 0b0 }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
          Instruction::ADD8(target) => {
            match target {
              ArithmeticTarget::C => {
                // TODO: implement ADD on register C
              }
              _ => { /* TODO: support more targets */ }
            }
          }
          _ => { /* TODO: support more instructions */ }
        }
      }
}