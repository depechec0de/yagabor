mod alu;
mod bus;
mod gpu;
mod instructions;
mod tests;

use core::panic;

use crate::gameboy::rom::ROM;
use instructions::*;

use self::bus::MemoryBus;

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

impl CPU {
    pub fn new() -> CPU {
        let mut membus = MemoryBus::new();

        CPU { regs: Registers::new(), sp: 0b0, pc: 0b0, bus: membus, is_halted: false }
    }

    fn load_boot(&mut self, boot: ROM){
        // TODO: map instead of copy
        // Loading the boot ROM data into memory
        for addr in 0..boot.size() {
            self.bus.write_byte(addr, boot.read_byte(addr))
        } 
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
            Instruction::LDSIG => self.ldsig(),
            Instruction::LDSPHL => self.ldsphl(),
            Instruction::JP(test) => self.jump(test),
            Instruction::JR(test) => self.jump_relative(test),
            Instruction::JPHL => self.jump_hl(),
            Instruction::PUSH(target) => self.push(target),
            Instruction::POP(target) => self.pop(target),
            Instruction::CALL(test) => self.call(test),
            Instruction::RET(test) => self.ret(test),
            Instruction::RST(target) => todo!(),
            Instruction::BIT(target, source) => self.bit(target, source),
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

    fn ldsig(&mut self) -> ProgramCounter {
        let value: i16 = self.read_next_byte() as i16;
        let (new_value, did_overflow) = self.sp.overflowing_add_signed(value);

        self.regs.flags.zero = false;
        self.regs.flags.subtract = false;
        self.regs.flags.carry = did_overflow;
        // TODO: Not sure about half-carry with signed
        self.regs.flags.half_carry = (self.sp & 0xF) + (value as u16 & 0xF) > 0xF;

        self.regs.set_hl(new_value);

        self.pc.wrapping_add(2)
    }

    fn ldsphl(&mut self) -> ProgramCounter {
        self.sp = self.regs.get_hl();
        self.pc.wrapping_add(1)
    }

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