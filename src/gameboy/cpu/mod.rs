pub mod mmu;
mod instructions;
mod tests;

use core::panic;

use self::{mmu::MMU, instructions::*};

pub(super) type ProgramCounter = u16;
pub(super) type StackPointer = u16;
pub(super) type Address = u16;

pub(crate) struct CPU{
    regs: Registers,
    sp: StackPointer,
    pc: ProgramCounter,
    is_halted: bool,
    mmu: MMU
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
    pub fn new(mmu: MMU) -> CPU {
        CPU { 
            regs: Registers::new(), 
            sp: 0b0, 
            pc: 0b0,  
            is_halted: false,
            mmu 
        }
    }

    pub(crate) fn step(&mut self) {
        let mut instruction_byte = self.mmu.read_byte(self.pc);
        println!("instruction byte {:x}", instruction_byte);

        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.mmu.read_byte(self.pc + 1);
        }

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
            println!("{:?} at PC: {}", instruction, self.pc);
            self.execute(instruction)
        } else {
            let description = format!("0x{}{:x}", if prefixed { "cb" } else { "" }, instruction_byte);
            panic!("Unkown instruction found for: {}", description)
        };

        println!("Next pc {}", next_pc);
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
            let least_significant_byte = self.mmu.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.mmu.read_byte(self.pc + 2) as u16;
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
            let offset: i8 = self.read_next_byte() as i8;
            self.pc.wrapping_add(2 + offset as u16)
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
          self.push_value( next_pc);
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

    pub(super) fn read_next_byte(&self) -> u8 {
        self.mmu.read_byte(self.pc+1)
    }

    pub(super) fn read_next_word(&self) -> u16 {
        ((self.mmu.read_byte(self.pc+2) as u16) << 8) | self.mmu.read_byte(self.pc+1) as u16
    }

    pub(super) fn load(&mut self, load_type: LoadType) -> ProgramCounter {
        match load_type {
        LoadType::Byte(target, source) => {
            let source_value = match source {
                LoadByteSource::A   => self.regs.a,
                LoadByteSource::B   => self.regs.b,
                LoadByteSource::C   => self.regs.c,
                LoadByteSource::D   => self.regs.d,
                LoadByteSource::E   => self.regs.e,
                LoadByteSource::H   => self.regs.h,
                LoadByteSource::L   => self.regs.l,
                LoadByteSource::D8  => self.read_next_byte(),
                LoadByteSource::HLI => self.mmu.read_byte(self.regs.get_hl())
            };
            match target {
                LoadByteTarget::A   => self.regs.a = source_value,
                LoadByteTarget::B   => self.regs.b = source_value,
                LoadByteTarget::C   => self.regs.c = source_value,
                LoadByteTarget::D   => self.regs.d = source_value,
                LoadByteTarget::E   => self.regs.e = source_value,
                LoadByteTarget::H   => self.regs.h = source_value,
                LoadByteTarget::L   => self.regs.l = source_value,
                LoadByteTarget::HLI => self.mmu.write_byte(self.regs.get_hl(), source_value)
            };
            match source {
                LoadByteSource::D8  => self.pc.wrapping_add(2),
                _                   => self.pc.wrapping_add(1),
            }
        },
        LoadType::Word(target) => {
            match target {
                WordRegister::BC => {
                    self.regs.set_bc(self.read_next_word());
                },
                WordRegister::DE => {
                    self.regs.set_de(self.read_next_word());
                },
                WordRegister::HL => {
                    self.regs.set_hl(self.read_next_word());
                },
                WordRegister::SP => {
                    self.sp = self.read_next_word();
                }
            }
            self.pc.wrapping_add(3)
        },
        LoadType::IndirectFromA(target) => {
            match target {
                AFromIndirectSource::BC => {
                    let addr = self.regs.get_bc();
                    self.regs.a = self.mmu.read_byte(addr);
                },
                AFromIndirectSource::DE => {
                    let addr = self.regs.get_de();
                    self.regs.a = self.mmu.read_byte(addr);
                },
                AFromIndirectSource::HLInc => {
                    let addr = self.regs.get_hl();
                    self.regs.a = self.mmu.read_byte(addr);
                    let new_value = self.regs.get_hl().wrapping_add(1);
                    self.regs.set_hl(new_value);
                },
                AFromIndirectSource::HLDec => {
                    let addr = self.regs.get_hl();
                    self.regs.a = self.mmu.read_byte(addr);
                    let new_value = self.regs.get_hl().wrapping_sub(1);
                    self.regs.set_hl(new_value);
                }
            }
            self.pc.wrapping_add(1)
        },
        _ => { todo!("todo") }
        }
    }

    pub(super) fn push(&mut self, target: StackTarget) -> ProgramCounter {
        let value = match target {
            StackTarget::BC => self.regs.get_bc(),
            StackTarget::DE => self.regs.get_de(),
            StackTarget::HL => self.regs.get_hl(),
            StackTarget::AF => self.regs.get_af(),
        };
        self.push_value(value);
        self.pc.wrapping_add(1)
    }

    pub(super) fn pop(&mut self, target: StackTarget) -> ProgramCounter {
        let result = self.pop_value();
        match target {
            StackTarget::BC => self.regs.set_bc(result),
            StackTarget::DE => self.regs.set_de(result),
            StackTarget::HL => self.regs.set_hl(result),
            StackTarget::AF => self.regs.set_af(result),
        };
        self.pc.wrapping_add(1)
    }

    pub(super) fn push_value(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.mmu.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);

        self.sp = self.sp.wrapping_sub(1);
        self.mmu.write_byte(self.sp, (value & 0xFF) as u8);
    }

    pub(super) fn pop_value(&mut self) -> u16 {
        let lsb = self.mmu.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let msb = self.mmu.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        (msb << 8) | lsb
    }

    pub(super) fn add(&mut self, target: ArithmeticTarget) -> ProgramCounter {
        let value = self.get_arithmetic_target_val(&target);

        let (new_value, did_overflow) = self.regs.a.overflowing_add(value);
        self.regs.flags.zero = new_value == 0;
        self.regs.flags.subtract = false;
        self.regs.flags.carry = did_overflow;
        // Half Carry is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble.
        self.regs.flags.half_carry = (self.regs.a & 0xF) + (value & 0xF) > 0xF;
        self.regs.a = new_value;

        self.arithmetic_pc_increment(&target)
    }

    pub(super) fn addsp8(&mut self) -> ProgramCounter {
        let value = self.read_next_byte() as u16;

        let (new_value, did_overflow) = self.sp.overflowing_add(value);
        self.regs.flags.zero = false;
        self.regs.flags.subtract = false;
        self.regs.flags.carry = did_overflow;
        self.regs.flags.half_carry = (self.sp & 0xF) + (value & 0xF) > 0xF;
        self.sp = new_value;

        self.pc.wrapping_add(2)
    }

    pub(super) fn add16(&mut self, target: WordRegister) -> ProgramCounter {
        let value = match target {
            WordRegister::BC => self.regs.get_bc(),
            WordRegister::DE => self.regs.get_de(),
            WordRegister::HL => self.regs.get_hl(),
            WordRegister::SP => self.sp,
        };

        let (new_value, did_overflow) = self.regs.get_hl().overflowing_add(value);
        self.regs.flags.subtract = false;
        self.regs.flags.carry = did_overflow;
        self.regs.flags.half_carry = (self.regs.get_hl() & 0xF) + (value & 0xF) > 0xF;
        self.regs.set_hl(new_value);

        self.pc.wrapping_add(1)
    }

    pub(super) fn adc(&mut self, target: ArithmeticTarget) -> ProgramCounter {
        let value = self.get_arithmetic_target_val(&target);

        let (new_value1, did_overflow1) = self.regs.a.overflowing_add(value);

        let (new_value2, did_overflow2) = new_value1.overflowing_add(self.regs.flags.carry as u8);

        self.regs.flags.zero = new_value2 == 0;
        self.regs.flags.subtract = false;
        self.regs.flags.half_carry = ((self.regs.a & 0xF) + (value & 0xF) > 0xF) || ((new_value1 & 0xF) + (self.regs.flags.carry as u8) > 0xF);
        self.regs.flags.carry = did_overflow1 || did_overflow2;      
        self.regs.a = new_value2;

        self.arithmetic_pc_increment(&target)
    }

    pub(super) fn sub(&mut self, target: ArithmeticTarget) -> ProgramCounter {
        let value = self.get_arithmetic_target_val(&target);

        let (new_value, did_overflow) = self.regs.a.overflowing_sub(value);
        self.regs.flags.zero = new_value == 0;
        self.regs.flags.subtract = true;
        self.regs.flags.carry = did_overflow;
        let (new_value_low, _) = (self.regs.a & 0xF).overflowing_sub(value & 0xF);
        self.regs.flags.half_carry = (new_value_low & 0x10) == 0x10;
        self.regs.a = new_value;

        self.arithmetic_pc_increment(&target)
    }

    pub(super) fn sbc(&mut self, target: ArithmeticTarget) -> ProgramCounter {
        let value = self.get_arithmetic_target_val(&target);

        let (new_value1, did_overflow1) = self.regs.a.overflowing_sub(self.regs.flags.carry as u8);
        let (new_value2, did_overflow2) = new_value1.overflowing_sub(value);

        self.regs.flags.zero = new_value2 == 0;
        self.regs.flags.subtract = true;
        self.regs.flags.carry = did_overflow1 || did_overflow2;
        let (new_value_low, _) = (new_value2 & 0xF).overflowing_sub(value & 0xF);
        self.regs.flags.half_carry = (new_value_low & 0x10) == 0x10;
        self.regs.a = new_value2;

        self.arithmetic_pc_increment(&target)
    }

    pub(super) fn and(&mut self, target: ArithmeticTarget) -> ProgramCounter {
        let value = self.get_arithmetic_target_val(&target);

        self.regs.a = self.regs.a & value;
        self.regs.flags.zero = self.regs.a == 0;
        self.regs.flags.subtract = false;
        self.regs.flags.half_carry = true;
        self.regs.flags.carry = false;

        self.arithmetic_pc_increment(&target)
    }

    pub(super) fn xor(&mut self, target: ArithmeticTarget) -> ProgramCounter {
        let value = self.get_arithmetic_target_val(&target);

        self.regs.a = self.regs.a ^ value;
        self.regs.flags.zero = self.regs.a == 0;
        self.regs.flags.subtract = false;
        self.regs.flags.half_carry = false;
        self.regs.flags.carry = false;

        self.arithmetic_pc_increment(&target)
    }

    pub(super) fn or(&mut self, target: ArithmeticTarget) -> ProgramCounter {
        let value = self.get_arithmetic_target_val(&target);

        self.regs.a = self.regs.a | value;
        self.regs.flags.zero = self.regs.a == 0;
        self.regs.flags.subtract = false;
        self.regs.flags.half_carry = false;
        self.regs.flags.carry = false;

        self.arithmetic_pc_increment(&target)
    }

    pub(super) fn cp(&mut self, target: ArithmeticTarget) -> ProgramCounter {
        let value = self.get_arithmetic_target_val(&target);

        let (result, did_overflow) = self.regs.a.overflowing_sub(value);
        self.regs.flags.zero = result == 0;
        self.regs.flags.subtract = true;
        let (new_value_low, _) = (self.regs.a & 0xF).overflowing_sub(value & 0xF);
        self.regs.flags.half_carry = (new_value_low & 0x10) == 0x10;
        self.regs.flags.carry = did_overflow;

        self.arithmetic_pc_increment(&target)
    }

    pub(super) fn inc(&mut self, target: IncDecTarget) -> ProgramCounter {
        let value = match target {
            IncDecTarget::A => self.regs.a = self.regs.a.wrapping_add(1),
            IncDecTarget::B => self.regs.b = self.regs.b.wrapping_add(1),
            IncDecTarget::C => self.regs.c = self.regs.c.wrapping_add(1),
            IncDecTarget::D => self.regs.d = self.regs.d.wrapping_add(1),
            IncDecTarget::E => self.regs.e = self.regs.e.wrapping_add(1),
            IncDecTarget::H => self.regs.h = self.regs.h.wrapping_add(1),
            IncDecTarget::L => self.regs.l = self.regs.l.wrapping_add(1),
            IncDecTarget::HLI => {
                let new_val = self.mmu.read_byte(self.regs.get_hl()).wrapping_add(1);
                self.mmu.write_byte(self.regs.get_hl(), new_val);
            }
        };
        self.pc.wrapping_add(1)
    }

    pub(super) fn dec(&mut self, target: IncDecTarget) -> ProgramCounter {
        let value = match target {
            IncDecTarget::A => self.regs.a = self.regs.a.wrapping_sub(1),
            IncDecTarget::B => self.regs.b = self.regs.b.wrapping_sub(1),
            IncDecTarget::C => self.regs.c = self.regs.c.wrapping_sub(1),
            IncDecTarget::D => self.regs.d = self.regs.d.wrapping_sub(1),
            IncDecTarget::E => self.regs.e = self.regs.e.wrapping_sub(1),
            IncDecTarget::H => self.regs.h = self.regs.h.wrapping_sub(1),
            IncDecTarget::L => self.regs.l = self.regs.l.wrapping_sub(1),
            IncDecTarget::HLI => {
                let new_val = self.mmu.read_byte(self.regs.get_hl()).wrapping_sub(1);
                self.mmu.write_byte(self.regs.get_hl(), new_val);
            }
        };
        self.pc.wrapping_add(1)
    }

    pub(super) fn inc16(&mut self, target: WordRegister) -> ProgramCounter {
        let value = match target {
            WordRegister::BC => self.regs.set_bc(self.regs.get_bc().wrapping_add(1)),
            WordRegister::DE => self.regs.set_de(self.regs.get_de().wrapping_add(1)),
            WordRegister::HL => self.regs.set_hl(self.regs.get_hl().wrapping_add(1)),
            WordRegister::SP => self.sp = self.sp.wrapping_add(1),
        };
        self.pc.wrapping_add(1)
    }

    pub(super) fn dec16(&mut self, target: WordRegister) -> ProgramCounter {
        let value = match target {
            WordRegister::BC => self.regs.set_bc(self.regs.get_bc().wrapping_sub(1)),
            WordRegister::DE => self.regs.set_de(self.regs.get_de().wrapping_sub(1)),
            WordRegister::HL => self.regs.set_hl(self.regs.get_hl().wrapping_sub(1)),
            WordRegister::SP => self.sp = self.sp.wrapping_sub(1),
        };
        self.pc.wrapping_add(1)
    }

    pub(super) fn bit(&mut self, target:BitTarget, source: BitSource) -> ProgramCounter {
        let i = get_position_by_bittarget(target);
        let value = self.get_bitsource_val(source);
        let bit_value = get_bit_val(i, value);

        self.regs.flags.zero = !bit_value;

        self.pc.wrapping_add(2)
    }

    fn get_bitsource_val(&self, source: BitSource) -> u8 {
        match source {
            BitSource::A => self.regs.a,
            BitSource::B => self.regs.b,
            BitSource::C => self.regs.c,
            BitSource::D => self.regs.d,
            BitSource::E => self.regs.e,
            BitSource::H => self.regs.h,
            BitSource::L => self.regs.l,
            BitSource::HLI => self.mmu.read_byte(self.regs.get_hl()),
        }
    }
    
    fn arithmetic_pc_increment(&self, target: &ArithmeticTarget) -> ProgramCounter {
        let is_d8: ProgramCounter = match target {
            ArithmeticTarget::D8 => 1,
            _ => 0
        }; 
        self.pc.wrapping_add(1+is_d8)
    }

    fn get_arithmetic_target_val(&self, target: &ArithmeticTarget) -> u8 {
        match target {
            ArithmeticTarget::A     => self.regs.a,
            ArithmeticTarget::B     => self.regs.b,
            ArithmeticTarget::C     => self.regs.c,
            ArithmeticTarget::D     => self.regs.d,
            ArithmeticTarget::E     => self.regs.e,
            ArithmeticTarget::H     => self.regs.h,
            ArithmeticTarget::L     => self.regs.l,
            ArithmeticTarget::HLI   => self.mmu.read_byte(self.regs.get_hl()),
            ArithmeticTarget::D8    => self.read_next_byte()
        }
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

fn get_position_by_bittarget(target:BitTarget) -> u8 {
    match target {
        BitTarget::Zero => 0,
        BitTarget::One => 1,
        BitTarget::Two => 2,
        BitTarget::Three => 3,
        BitTarget::Four => 4,
        BitTarget::Five => 5,
        BitTarget::Six => 6,
        BitTarget::Seven => 7,
    }
}

fn get_bit_val(position:u8, value:u8) -> bool {
    let mask = 1 << position;
    (mask & value) > 0
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