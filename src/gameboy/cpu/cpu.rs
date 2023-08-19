use core::panic;
use std::io::{Error, ErrorKind};

use crate::gameboy::{gameboy::ExecuteOutput, interrupts::Interruption};

use super::{registers::Registers, mmu::MMU, instructions::*};

pub(super) type ProgramCounter = u16;
pub(super) type StackPointer = u16;
pub(crate) type Address = u16;

pub(crate) struct CPU{
    pub(super) regs: Registers,
    pub(super) sp: StackPointer,
    pub(super) pc: ProgramCounter,
    pub(super) is_halted: bool,
    pub(super) mmu: MMU,
    pub(super) ime: bool,
}

impl CPU {
    pub fn new(mmu: MMU) -> Self {
        Self { 
            regs: Registers::new(), 
            sp: 0b0, 
            pc: 0b0,  
            is_halted: false,
            mmu,
            ime: true
        }
    }

    pub(crate) fn step(&mut self) -> Result<ExecuteOutput, Error> {

        let instruction_byte = self.mmu.read_byte(self.pc);
        let byte0 = self.mmu.read_byte(self.pc+1);
        let byte1 = self.mmu.read_byte(self.pc+2);

        if let Some(instruction) = Instruction::parse_instruction(instruction_byte, byte0, byte1) {
            //println!("{:?}", instruction);
            match self.execute(instruction.clone()) {
                Ok(result) => {
                    //println!("pc {:#04x} | {:x} ({:?} cycles) {:?} {:?} SP:{:x}", self.pc, instruction_byte, u64::from(cycles.clone()) , instruction, self.regs, self.sp);
                    // if self.pc > 0xFF {
                        // println!("A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:02X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}", 
                        //         self.regs.a, u8::from(self.regs.flags.clone()), self.regs.b, self.regs.c, self.regs.d, self.regs.e, self.regs.h, self.regs.l, self.sp, self.pc, self.mmu.read_byte(self.pc), self.mmu.read_byte(self.pc+1), self.mmu.read_byte(self.pc+2), self.mmu.read_byte(self.pc+3) );                        
                    // }

                    
                    
                    Ok(result)
                },
                Err(error) => {
                    //println!("{}", self.mmu);
                    Err(Error::new(ErrorKind::Other, format!("Error during execution: {}", error)))
                }
            }
        } else {
            //println!("{}", self.mmu);
            Err(Error::new(ErrorKind::Other, format!("Unkown instruction {:x} {:x} found", instruction_byte, byte0)))
        }   
    }   

    // Returns the next PC to execute and the cycles consumed
    pub(super) fn execute(&mut self, instruction: Instruction) -> Result<ExecuteOutput, Error> {

        let inst_type = instruction.op.clone();
        let inst_size = instruction.size_bytes();

        let prev_pc = self.pc;
        self.pc = self.pc.wrapping_add(inst_size as u16);

        let exec_result = match inst_type {
            // This instructions never return and change directly the PC
            InstructionType::CALL(test) => self.call(test, prev_pc),
            InstructionType::RET(test) => self.ret(test),
            InstructionType::JP(test) => self.jump(test, prev_pc),
            InstructionType::JR(test) => self.jump_relative(test, prev_pc),
            InstructionType::JPHL => self.jump_hl(),
            InstructionType::NOP => self.nop(),
            InstructionType::HALT => self.halt(),
            InstructionType::SCF => self.scf(),
            InstructionType::CCF => self.ccf(),
            InstructionType::CPL => self.cpl(),
            InstructionType::ADD(target) => self.add(target, prev_pc),
            InstructionType::ADC(target) => self.adc(target, prev_pc),
            InstructionType::INC(target) => self.inc(target),
            InstructionType::DEC(target) => self.dec(target),
            InstructionType::ADD16(target) => self.add16(target),
            InstructionType::INC16(target) => self.inc16(target),
            InstructionType::DEC16(target) => self.dec16(target),
            InstructionType::ADDSP8 => self.addsp8(prev_pc),
            InstructionType::SUB(target) => self.sub(target, prev_pc),
            InstructionType::SBC(target) => self.sbc(target, prev_pc),
            InstructionType::AND(target) => self.and(target, prev_pc),
            InstructionType::XOR(target) => self.xor(target, prev_pc),
            InstructionType::OR(target) => self.or(target, prev_pc),
            InstructionType::CP(target) => self.cp(target, prev_pc),
            InstructionType::LD(load_type) => self.load(prev_pc, load_type),
            InstructionType::LDSIG => self.ldsig(prev_pc),
            InstructionType::LDSPHL => self.ldsphl(),
            InstructionType::LDSPA16 => self.ldspa16(),
            InstructionType::LDFF(load_type) => self.ldff(load_type, prev_pc),
            InstructionType::PUSH(target) => self.push(target),
            InstructionType::POP(target) => self.pop(target),
            //InstructionType::RST(target) => todo!(),
            InstructionType::BIT(bit_type) => self.bit(bit_type),
            //InstructionType::RETI => todo!(),
            InstructionType::DAA => self.daa(),
            InstructionType::RL(target) => self.rl(target),
            InstructionType::RLC(target) => self.rlc(target),
            InstructionType::RR(target) => self.rr(target),
            InstructionType::RRC(target) => self.rrc(target),
            InstructionType::RLA => self.rla(),
            InstructionType::RLCA => self.rlca(),
            InstructionType::RRA => self.rra(),
            InstructionType::RRCA => self.rrca(),
            InstructionType::SRA(target) => self.sla(target),
            InstructionType::SLA(target) => self.sra(target),
            InstructionType::SRL(target) => self.srl(target),
            InstructionType::SWAP(target) => self.swap(target),
            InstructionType::EI => self.ei(),
            InstructionType::DI => self.di(),
            InstructionType::RES(target) => self.res_set(target, false),
            InstructionType::SET(target) => self.res_set(target, true),
            _ => { Err(Error::new(ErrorKind::Other, "Unsupported instruction")) }
        };

        exec_result
        
    }

    fn jump(&mut self, test: JumpTest, current_pc: ProgramCounter) -> Result<ExecuteOutput, Error> {
        let should_jump = self.should_jump(test);
     
        if should_jump {
            // Gameboy is little endian so read pc + 2 as most significant bit
            // and pc + 1 as least significant bit
            let least_significant_byte = self.mmu.read_byte(current_pc + 1) as u16;
            let most_significant_byte = self.mmu.read_byte(current_pc + 2) as u16;
            self.pc = (most_significant_byte << 8) | least_significant_byte;

            Ok(ExecuteOutput::new(ClockCycles::Four, None))
        } else {
            Ok(ExecuteOutput::new(ClockCycles::Three, None))
        }
    }

    fn jump_relative(&mut self, test: JumpTest, current_pc: ProgramCounter) -> Result<ExecuteOutput, Error> {
        let should_jump = self.should_jump(test);
     
        if should_jump {
            let offset: i8 = self.read_next_byte(current_pc) as i8;
            self.pc = current_pc.wrapping_add(2i8.wrapping_add(offset) as u16);

            Ok(ExecuteOutput::new(ClockCycles::Three, None))
        } else {
            Ok(ExecuteOutput::new(ClockCycles::Two, None))
        }
    }

    fn jump_hl(&mut self) -> Result<ExecuteOutput, Error> {
        self.pc = self.regs.get_hl();

        Ok(ExecuteOutput::new(ClockCycles::One, None))
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

    fn nop(&self) -> Result<ExecuteOutput, Error> {
        Ok(ExecuteOutput::new(ClockCycles::One, None))
    }

    fn ei(&mut self) -> Result<ExecuteOutput, Error> {
        self.ime = true;
        Ok(ExecuteOutput::new(ClockCycles::One, None))
    }

    fn di(&mut self) -> Result<ExecuteOutput, Error> {
        self.ime = false;
        Ok(ExecuteOutput::new(ClockCycles::One, None))
    }

    fn scf(&mut self) -> Result<ExecuteOutput, Error> {
        self.regs.flags.carry = true;
        self.regs.flags.subtract = false;
        self.regs.flags.half_carry = false;

        Ok(ExecuteOutput::new(ClockCycles::One, None))
    }

    fn cpl(&mut self) -> Result<ExecuteOutput, Error> {
        self.regs.a = !self.regs.a; 
        self.regs.flags.subtract = true;
        self.regs.flags.half_carry = true;

        Ok(ExecuteOutput::new(ClockCycles::One, None))
    }

    fn ccf(&mut self) -> Result<ExecuteOutput, Error> {
        self.regs.flags.carry = !self.regs.flags.carry;
        self.regs.flags.subtract = false;
        self.regs.flags.half_carry = false;

        Ok(ExecuteOutput::new(ClockCycles::One, None))
    }

    // https://forums.nesdev.org/viewtopic.php?t=15944
    fn daa(&mut self) -> Result<ExecuteOutput, Error> {
        if !self.regs.flags.subtract {  // after an addition, adjust if (half-)carry occurred or if result is out of bounds
            if self.regs.flags.carry || self.regs.a > 0x99 { 
                self.regs.a = self.regs.a.wrapping_add(0x60);
                self.regs.flags.carry = true; 
            }
            if self.regs.flags.half_carry || (self.regs.a & 0x0f) > 0x09 { 
                self.regs.a = self.regs.a.wrapping_add(0x6); 
            }
        } else {  // after a subtraction, only adjust if (half-)carry occurred
            if self.regs.flags.carry { 
                self.regs.a = self.regs.a.wrapping_sub(0x60);
            }
            if self.regs.flags.half_carry { 
                self.regs.a = self.regs.a.wrapping_sub(0x06);
            }
        }
        // these flags are always updated
        self.regs.flags.zero = self.regs.a == 0; // the usual z flag
        self.regs.flags.half_carry = false; // h flag is always cleared

        Ok(ExecuteOutput::new(ClockCycles::One, None))
    }

    fn halt(&mut self) -> Result<ExecuteOutput, Error> {
        self.is_halted = true;

        Ok(ExecuteOutput::new(ClockCycles::One, None))
    }

    fn call(&mut self, test: JumpTest, current_pc: ProgramCounter) -> Result<ExecuteOutput, Error> {
        let should_jump = self.should_jump(test);

        if should_jump {
            self.push_value(current_pc.wrapping_add(3));
            self.pc = self.read_next_word(current_pc);

            Ok(ExecuteOutput::new(ClockCycles::Six, None))
        } else {
            Ok(ExecuteOutput::new(ClockCycles::Three, None))
        }
    }
    
    fn ret(&mut self, test: JumpTest) -> Result<ExecuteOutput, Error> {
        let jump_condition = self.should_jump(test);
        self.return_(jump_condition)
    }

    fn return_(&mut self, should_jump: bool) -> Result<ExecuteOutput, Error> {
        if should_jump {
            self.pc = self.pop_value();

            Ok(ExecuteOutput::new(ClockCycles::Five, None))
        } else {
            Ok(ExecuteOutput::new(ClockCycles::Two, None))
        }
    }

    fn ldsig(&mut self, current_pc: ProgramCounter) -> Result<ExecuteOutput, Error> {
        let value: i16 = self.read_next_byte(current_pc) as i16;
        let (new_value, did_overflow) = self.sp.overflowing_add_signed(value);

        self.regs.flags.zero = false;
        self.regs.flags.subtract = false;
        self.regs.flags.carry = did_overflow;
        // TODO: Not sure about half-carry with signed
        self.regs.flags.half_carry = (self.sp & 0xF) + (value as u16 & 0xF) > 0xF;

        self.regs.set_hl(new_value);

        Ok(ExecuteOutput::new(ClockCycles::Three, None))        
    }

    fn ldsphl(&mut self) -> Result<ExecuteOutput, Error> {
        self.sp = self.regs.get_hl();

        Ok(ExecuteOutput::new(ClockCycles::Two, None))
    }

    fn ldspa16(&mut self) -> Result<ExecuteOutput, Error> {
        let address = self.read_next_word(self.pc);

        let lsb = (self.sp & 0x00FF) as u8;
        let msb = ((self.sp & 0xFF00) >> 8) as u8;

        self.mmu.write_byte(address, lsb);
        self.mmu.write_byte(address.wrapping_add(1), msb);

        Ok(ExecuteOutput::new(ClockCycles::Five, None))
    }

    pub(super) fn read_next_byte(&self, address: Address) -> u8 {
        self.mmu.read_byte(address+1)
    }

    pub(super) fn read_next_word(&self, address: Address) -> u16 {
        ((self.mmu.read_byte(address+2) as u16) << 8) | self.mmu.read_byte(address+1) as u16
    }

    fn load(&mut self, current_pc: ProgramCounter, load_type: LoadType) -> Result<ExecuteOutput, Error> {
        match &load_type {
            LoadType::Byte(target, source) => {
                let source_value = match source {
                    RegistersIndDir::A   => self.regs.a,
                    RegistersIndDir::B   => self.regs.b,
                    RegistersIndDir::C   => self.regs.c,
                    RegistersIndDir::D   => self.regs.d,
                    RegistersIndDir::E   => self.regs.e,
                    RegistersIndDir::H   => self.regs.h,
                    RegistersIndDir::L   => self.regs.l,
                    RegistersIndDir::D8  => self.read_next_byte(current_pc),
                    RegistersIndDir::HLI => self.mmu.read_byte(self.regs.get_hl())
                };
                match target {
                    RegistersIndirect::A   => self.regs.a = source_value,
                    RegistersIndirect::B   => self.regs.b = source_value,
                    RegistersIndirect::C   => self.regs.c = source_value,
                    RegistersIndirect::D   => self.regs.d = source_value,
                    RegistersIndirect::E   => self.regs.e = source_value,
                    RegistersIndirect::H   => self.regs.h = source_value,
                    RegistersIndirect::L   => self.regs.l = source_value,
                    RegistersIndirect::HLI => { self.mmu.write_byte(self.regs.get_hl(), source_value); }
                };

            },
            LoadType::Word(target) => {
                match target {
                    WordRegister::BC => {
                        self.regs.set_bc(self.read_next_word(current_pc));
                    },
                    WordRegister::DE => {
                        self.regs.set_de(self.read_next_word(current_pc));
                    },
                    WordRegister::HL => {
                        self.regs.set_hl(self.read_next_word(current_pc));
                    },
                    WordRegister::SP => {
                        self.sp = self.read_next_word(current_pc);
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
                self.regs.a = self.mmu.read_byte(self.read_next_word(current_pc));
            },
            LoadType::DirectFromA => {
                self.mmu.write_byte(self.read_next_word(current_pc), self.regs.a);
            }
        }

        match load_type {
            LoadType::Byte(_,RegistersIndDir::HLI) => Ok(ExecuteOutput::new(ClockCycles::Two, None)),
            LoadType::Byte(RegistersIndirect::HLI, _) => Ok(ExecuteOutput::new(ClockCycles::Two, None)),
            LoadType::AFromIndirect(_) => Ok(ExecuteOutput::new(ClockCycles::Two, None)),
            LoadType::IndirectFromA(_) => Ok(ExecuteOutput::new(ClockCycles::Two, None)),
            LoadType::Word(_) => Ok(ExecuteOutput::new(ClockCycles::Three, None)),
            LoadType::AFromDirect => Ok(ExecuteOutput::new(ClockCycles::Four, None)),
            LoadType::DirectFromA => Ok(ExecuteOutput::new(ClockCycles::Four, None)),
            _ => Ok(ExecuteOutput::new(ClockCycles::One, None)),
        }
    }
    
    fn ldff(&mut self, load_type: LoadFFType, current_pc: Address) -> Result<ExecuteOutput, Error> {
        match load_type {
            LoadFFType::AtoFFC => { 
                let addr: u16 = 0xFF00 + self.regs.c as u16;        
                self.mmu.write_byte(addr, self.regs.a);
                Ok(ExecuteOutput::new(ClockCycles::Two, None))
            },
            LoadFFType::FFCtoA => {
                let addr: u16 = 0xFF00 + self.regs.c as u16;        
                self.regs.a = self.mmu.read_byte(addr);
                Ok(ExecuteOutput::new(ClockCycles::Two, None))
            },
            LoadFFType::AtoFFa8 => {
                let addr: u16 = 0xFF00 + self.read_next_byte(current_pc) as u16;        
                self.mmu.write_byte(addr, self.regs.a);
                Ok(ExecuteOutput::new(ClockCycles::Three, None))
            },
            LoadFFType::FFa8toA => {
                let addr: u16 = 0xFF00 + self.read_next_byte(current_pc) as u16;        
                self.regs.a = self.mmu.read_byte(addr);
                Ok(ExecuteOutput::new(ClockCycles::Three, None))
            }
        }
    }

    pub(super) fn push(&mut self, target: StackTarget) -> Result<ExecuteOutput, Error> {
        let value = match target {
            StackTarget::BC => self.regs.get_bc(),
            StackTarget::DE => self.regs.get_de(),
            StackTarget::HL => self.regs.get_hl(),
            StackTarget::AF => self.regs.get_af(),
        };
        self.push_value(value);

        Ok(ExecuteOutput::new(ClockCycles::Four, None))
    }

    pub(super) fn pop(&mut self, target: StackTarget) -> Result<ExecuteOutput, Error> {
        let result = self.pop_value();
        match target {
            StackTarget::BC => self.regs.set_bc(result),
            StackTarget::DE => self.regs.set_de(result),
            StackTarget::HL => self.regs.set_hl(result),
            StackTarget::AF => self.regs.set_af(result),
        };

        Ok(ExecuteOutput::new(ClockCycles::Three, None))
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

    pub(crate) fn interrupts_enabled(&self) -> bool {
        self.ime
    }
}

// We use machine cycles for reference, but in the translation we multiply by 4
#[derive(Debug, Clone)]
pub(crate) enum ClockCycles {
    One, Two, Three, Four, Five, Six
}

impl std::convert::From<ClockCycles> for u64  {
    fn from(cycles: ClockCycles) -> u64 {
        let machine_cycles = match cycles {
            ClockCycles::One => 1,
            ClockCycles::Two => 2,
            ClockCycles::Three => 3,
            ClockCycles::Four => 4,
            ClockCycles::Five => 5,
            ClockCycles::Six => 6
        };
        machine_cycles*4
    }
}