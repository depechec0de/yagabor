mod alu;
pub mod mmu;
mod instructions;
mod tests;
mod registers;

use core::panic;

use self::{mmu::MMU, instructions::*, registers::*};

pub(super) type ProgramCounter = u16;
pub(super) type StackPointer = u16;

pub(crate) struct CPU{
    regs: Registers,
    sp: StackPointer,
    pc: ProgramCounter,
    is_halted: bool,
    mmu: MMU
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

        let next_pc = if let Some(instruction) = self.parse_instruction(instruction_byte) {
            self.execute(instruction)
        } else {
            panic!("Unkown instruction found for: {}", instruction_byte)
        };
        self.pc = next_pc;
    }

    fn parse_instruction(&self, byte: u8) -> Option<Instruction> {
        let prefixed = byte == 0xCB;
        let mut instruction_byte = byte;
        if prefixed {
            instruction_byte = self.mmu.read_byte(self.pc + 1);
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
                InstructionSize::TwoBytes => Some(self.read_next_byte() as u16),
                InstructionSize::ThreeBytes => Some(self.read_next_word()),
            };

            Some(Instruction{ op, size, payload })
        }else{
            None
        }        
    }

    // Returns the next PC to execute
    fn execute(&mut self, instruction: Instruction) -> ProgramCounter {
        let mut jump_pc : Option<ProgramCounter> = None;
        let inst_type = instruction.op;

        match inst_type {
            InstructionType::NOP => self.nop(),
            InstructionType::HALT => self.halt(),
            InstructionType::SCF => self.scf(),
            InstructionType::CCF => self.ccf(),
            InstructionType::CPL => self.cpl(),
            InstructionType::ADD(target) => self.add(target),
            InstructionType::ADC(target) => self.adc(target),
            InstructionType::INC(target) => self.inc(target),
            InstructionType::DEC(target) => self.dec(target),
            InstructionType::ADD16(target) => self.add16(target),
            InstructionType::INC16(target) => self.inc16(target),
            InstructionType::DEC16(target) => self.dec16(target),
            InstructionType::ADDSP8 => self.addsp8(),
            InstructionType::SUB(target) => self.sub(target),
            InstructionType::SBC(target) => self.sbc(target),
            InstructionType::AND(target) => self.and(target),
            InstructionType::XOR(target) => self.xor(target),
            InstructionType::OR(target) => self.or(target),
            InstructionType::CP(target) => self.cp(target),
            InstructionType::LD(load_type) => self.load(load_type),
            InstructionType::LDSIG => self.ldsig(),
            InstructionType::LDSPHL => self.ldsphl(),
            InstructionType::LDFF(load_type) => self.ldff(load_type),
            InstructionType::JP(test) => jump_pc = Some(self.jump(test)),
            InstructionType::JR(test) => jump_pc = Some(self.jump_relative(test)),
            InstructionType::JPHL => jump_pc = Some(self.jump_hl()),
            InstructionType::PUSH(target) => self.push(target),
            InstructionType::POP(target) => self.pop(target),
            InstructionType::CALL(test) => jump_pc = Some(self.call(test)),
            InstructionType::RET(test) => jump_pc = Some(self.ret(test)),
            InstructionType::RST(target) => todo!(),
            InstructionType::BIT(target, source) => self.bit(target, source),
            InstructionType::RETI => todo!(),
            InstructionType::DAA => todo!("daa"), //self.daa(),
            InstructionType::RL(target) => self.rl(target),
            InstructionType::RLA => self.rla(),
            _ => { todo!("Unsupported instruction") }
        }

        match jump_pc {
            None => self.pc.wrapping_add(instruction.size as u16),
            Some(jpc) => jpc
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
            self.pc.wrapping_add(2i8.wrapping_add(offset) as u16)
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

    fn nop(&self) {
    }

    fn scf(&mut self) {
        self.regs.flags.carry = true;
        self.regs.flags.subtract = false;
        self.regs.flags.half_carry = false;
    }

    fn cpl(&mut self) {
        self.regs.a = !self.regs.a; 
        self.regs.flags.subtract = true;
        self.regs.flags.half_carry = true;
    }

    fn ccf(&mut self) {
        self.regs.flags.carry = !self.regs.flags.carry;
        self.regs.flags.subtract = false;
        self.regs.flags.half_carry = false;
    }

    fn daa(&self) {
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
    }

    fn halt(&mut self) {
        self.is_halted = true;
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

    fn return_(&mut self, should_jump: bool) -> ProgramCounter {
        if should_jump {
            self.pop_value()
        } else {
            self.pc.wrapping_add(1)
        }
    }

    fn ldsig(&mut self) {
        let value: i16 = self.read_next_byte() as i16;
        let (new_value, did_overflow) = self.sp.overflowing_add_signed(value);

        self.regs.flags.zero = false;
        self.regs.flags.subtract = false;
        self.regs.flags.carry = did_overflow;
        // TODO: Not sure about half-carry with signed
        self.regs.flags.half_carry = (self.sp & 0xF) + (value as u16 & 0xF) > 0xF;

        self.regs.set_hl(new_value);

        
    }

    fn ldsphl(&mut self) {
        self.sp = self.regs.get_hl();
    }

    fn read_next_byte(&self) -> u8 {
        self.mmu.read_byte(self.pc+1)
    }

    fn read_next_word(&self) -> u16 {
        ((self.mmu.read_byte(self.pc+2) as u16) << 8) | self.mmu.read_byte(self.pc+1) as u16
    }

    fn load(&mut self, load_type: LoadType) {
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
            },
            LoadType::AFromIndirect(target) => {
                match target {
                    LoadIndirectSource::BC => {
                        let addr = self.regs.get_bc();
                        self.regs.a = self.mmu.read_byte(addr);
                    },
                    LoadIndirectSource::DE => {
                        let addr = self.regs.get_de();
                        self.regs.a = self.mmu.read_byte(addr);
                    },
                    LoadIndirectSource::HLInc => {
                        let addr = self.regs.get_hl();
                        self.regs.a = self.mmu.read_byte(addr);
                        let new_value = self.regs.get_hl().wrapping_add(1);
                        self.regs.set_hl(new_value);
                    },
                    LoadIndirectSource::HLDec => {
                        let addr = self.regs.get_hl();
                        self.regs.a = self.mmu.read_byte(addr);
                        let new_value = self.regs.get_hl().wrapping_sub(1);
                        self.regs.set_hl(new_value);
                    }
                }
            },
            LoadType::IndirectFromA(target) => {
                match target {
                    LoadIndirectSource::BC => {
                        let addr = self.regs.get_bc();
                        self.mmu.write_byte(addr, self.regs.a);
                    },
                    LoadIndirectSource::DE => {
                        let addr = self.regs.get_de();
                        self.mmu.write_byte(addr, self.regs.a);
                    },
                    LoadIndirectSource::HLInc => {
                        let addr = self.regs.get_hl();
                        self.mmu.write_byte(addr, self.regs.a);
                        let new_value = self.regs.get_hl().wrapping_add(1);
                        self.regs.set_hl(new_value);
                    },
                    LoadIndirectSource::HLDec => {
                        let addr = self.regs.get_hl();
                        self.mmu.write_byte(addr, self.regs.a);
                        let new_value = self.regs.get_hl().wrapping_sub(1);
                        self.regs.set_hl(new_value);
                    }
                }
            },
            LoadType::AFromDirect => {
                self.regs.a = self.mmu.read_byte(self.read_next_word());
            },
            LoadType::DirectFromA => {
                self.mmu.write_byte(self.read_next_word(), self.regs.a);
            }
        }
    }
    
    fn ldff(&mut self, load_type: LoadFFType) {
        match load_type {
            LoadFFType::AtoFFC => { 
                let addr: u16 = 0xFF00 + self.regs.c as u16;        
                self.mmu.write_byte(addr, self.regs.a);
            },
            LoadFFType::FFCtoA => {
                let addr: u16 = 0xFF00 + self.regs.c as u16;        
                self.regs.a = self.mmu.read_byte(addr);
            },
            LoadFFType::AtoFFa8 => {
                let addr: u16 = 0xFF00 + self.read_next_byte() as u16;        
                self.mmu.write_byte(addr, self.regs.a);
            },
            LoadFFType::FFa8toA => {
                let addr: u16 = 0xFF00 + self.read_next_byte() as u16;        
                self.regs.a = self.mmu.read_byte(addr);
            }
        }
    }

    fn push(&mut self, target: StackTarget) {
        let value = match target {
            StackTarget::BC => self.regs.get_bc(),
            StackTarget::DE => self.regs.get_de(),
            StackTarget::HL => self.regs.get_hl(),
            StackTarget::AF => self.regs.get_af(),
        };
        self.push_value(value);
    }

    fn pop(&mut self, target: StackTarget) {
        let result = self.pop_value();
        match target {
            StackTarget::BC => self.regs.set_bc(result),
            StackTarget::DE => self.regs.set_de(result),
            StackTarget::HL => self.regs.set_hl(result),
            StackTarget::AF => self.regs.set_af(result),
        };
    }

    fn push_value(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.mmu.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.mmu.write_byte(self.sp, (value & 0xFF) as u8);
    }

    fn pop_value(&mut self) -> u16 {
        let lsb = self.mmu.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let msb = self.mmu.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        (msb << 8) | lsb
    }
}