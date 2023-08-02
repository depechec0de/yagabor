mod alu;
mod bus;
mod registers;
mod instructions;
mod tests;

use core::panic;

use crate::rom::ROM;
use instructions::*;

type ProgramCounter = u16;
type StackPointer = u16;
type Address = u16;

pub struct CPU {
    regs: Registers,
    sp: StackPointer,
    pc: ProgramCounter,
    bus: MemoryBus,
    is_halted: bool
}

pub struct Registers {
    a: u8, // Accumulators
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    flags: FlagsRegister,
    h: u8,
    l: u8,
}
#[derive(Clone)]
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

        CPU { regs: Registers::new(), sp: 0b0, pc: 0b0, bus: membus, is_halted: false }
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
            Instruction::NOP => self.nop(),
            Instruction::HALT => self.halt(),
            Instruction::SCF => self.scf(),
            Instruction::CCF => self.ccf(),
            Instruction::CPL => self.cpl(),
            Instruction::ADD(target) => self.add(target),
            Instruction::ADC(target) => self.adc(target),
            Instruction::INC(target) => self.inc(target),
            Instruction::DEC(target) => self.dec(target),
            Instruction::ADD16(target) => self.add16(target),
            Instruction::INC16(target) => self.inc16(target),
            Instruction::DEC16(target) => self.dec16(target),
            Instruction::ADDSP8 => self.addsp8(),
            Instruction::SUB(target) => self.sub(target),
            Instruction::SBC(target) => self.sbc(target),
            Instruction::AND(target) => self.and(target),
            Instruction::XOR(target) => self.xor(target),
            Instruction::OR(target) => self.or(target),
            Instruction::CP(target) => self.cp(target),
            Instruction::LD(load_type) => self.load(load_type),
            Instruction::JP(test) => self.jump(test),
            Instruction::JR(test) => self.jump_relative(test),
            Instruction::JPHL => self.jump_hl(),
            Instruction::PUSH(target) => self.push(target),
            Instruction::POP(target) => self.pop(target),
            Instruction::CALL(test) => self.call(test),
            Instruction::RET(test) => self.ret(test),
            Instruction::RST(target) => todo!(),
            Instruction::RETI => todo!(),
            Instruction::DAA => todo!("daa"), //self.daa(),
            _ => { /* TODO: support more instructions */ self.pc }
        }
    }

    fn jump(&self, test: JumpTest) -> ProgramCounter {
        let should_jump = self.should_jump(test);
     
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

    fn jump_relative(&self, test: JumpTest) -> ProgramCounter {
        let should_jump = self.should_jump(test);
     
        if should_jump {
            let offset = self.read_next_byte();
            self.pc.wrapping_add(offset as u16)
        } else {
            // If we don't jump we need to still move the program
            // counter forward by 2 since the jump instruction is
            // 2 bytes wide (1 byte for tag and 1 bytes for jump offset)
            self.pc.wrapping_add(2)
        }
    }

    fn jump_hl(&mut self) -> ProgramCounter {
        self.pc = self.regs.get_hl();
        self.pc
    }   

    fn should_jump(&self, test: JumpTest) -> bool {
        match test {
            JumpTest::NotZero => !self.regs.flags.zero,
            JumpTest::NotCarry => !self.regs.flags.carry,
            JumpTest::Zero => self.regs.flags.zero,
            JumpTest::Carry => self.regs.flags.carry,
            JumpTest::Always => true
        }
    }

    fn nop(&self) -> ProgramCounter {
        self.pc.wrapping_add(1)
    }

    fn scf(&mut self) -> ProgramCounter {
        self.regs.flags.carry = true;
        self.regs.flags.subtract = false;
        self.regs.flags.half_carry = false;
        self.pc.wrapping_add(1)
    }

    fn cpl(&mut self) -> ProgramCounter {
        self.regs.a = !self.regs.a; 
        self.regs.flags.subtract = true;
        self.regs.flags.half_carry = true;
        self.pc.wrapping_add(1)
    }

    fn ccf(&mut self) -> ProgramCounter {
        self.regs.flags.carry = !self.regs.flags.carry;
        self.regs.flags.subtract = false;
        self.regs.flags.half_carry = false;
        self.pc.wrapping_add(1)
    }

    fn daa(&self) -> ProgramCounter {
        // https://ehaskins.com/2018-01-30%20Z80%20DAA/
        // let correction = 0;
      
        // let setFlagC = 0;
        // if (flagH || (!flagN && (value & 0xf) > 9)) {
        //   correction |= 0x6;
        // }
      
        // if (flagC || (!flagN && value > 0x99)) {
        //   correction |= 0x60;
        //   setFlagC = FLAG_C;
        // }
      
        // value += flagN ? -correction : correction;
      
        // value &= 0xff;
      
        // const setFlagZ = value === 0 ? FLAG_Z : 0;
      
        // regF &= ~(FLAG_H | FLAG_Z | FLAG_C);
        // regF |= setFlagC | setFlagZ;
      
        // return { output, carry, zero };

        self.pc.wrapping_add(1)
    }

    fn halt(&mut self) -> ProgramCounter {
        self.is_halted = true;
        self.pc.wrapping_add(1)
    }

    fn call(&mut self, test: JumpTest) -> ProgramCounter {
        let should_jump = self.should_jump(test);

        let next_pc = self.pc.wrapping_add(3);
        if should_jump {
          self.push_value(next_pc);
          self.read_next_word()
        } else {
          next_pc
        }
    }
    
    fn ret(&mut self, test: JumpTest) -> ProgramCounter {
        let jump_condition = self.should_jump(test);
        self.return_(jump_condition)
    }

    fn return_(&mut self, should_jump: bool) -> u16 {
        if should_jump {
            self.pop_value()
        } else {
            self.pc.wrapping_add(1)
        }
    }

}