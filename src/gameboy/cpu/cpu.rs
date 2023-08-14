use core::panic;
use std::io::{Error, ErrorKind};

use crate::gameboy::ClockCycles;

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
    pub fn new(mmu: MMU) -> CPU {
        CPU { 
            regs: Registers::new(), 
            sp: 0b0, 
            pc: 0b0,  
            is_halted: false,
            mmu,
            ime: true
        }
    }

    pub(crate) fn step(&mut self) -> Result<ClockCycles, Error> {
        let instruction_byte = self.mmu.read_byte(self.pc);
        let byte0 = self.mmu.read_byte(self.pc+1);
        let byte1 = self.mmu.read_byte(self.pc+2);

        if let Some(instruction) = Instruction::parse_instruction(instruction_byte, byte0, byte1) {
            println!("{:?}", instruction);
            match self.execute(instruction.clone()) {
                Ok(cycles) => {
                    //println!("pc {:#04x} | {:x} ({:?} cycles) {:?} {:?} SP:{:x}", self.pc, instruction_byte, u64::from(cycles.clone()) , instruction, self.regs, self.sp);
                    //if self.pc > 0xFF {
                        println!("A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:02X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}", 
                                self.regs.a, u8::from(self.regs.flags.clone()), self.regs.b, self.regs.c, self.regs.d, self.regs.e, self.regs.h, self.regs.l, self.sp, self.pc, self.mmu.read_byte(self.pc), self.mmu.read_byte(self.pc+1), self.mmu.read_byte(self.pc+2), self.mmu.read_byte(self.pc+3) );
                    //}
                    Ok(cycles)
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
    pub(super) fn execute(&mut self, instruction: Instruction) -> Result<ClockCycles, Error> {

        let inst_type = instruction.op.clone();
        let inst_size = instruction.size_bytes();

        let prev_pc = self.pc;
        self.pc = self.pc.wrapping_add(inst_size as u16);

        let executed_cycles = match inst_type {
            // This instructions never return and change directly the PC
            InstructionType::CALL(test) => Ok(self.call(test, prev_pc)),
            InstructionType::RET(test) => Ok(self.ret(test)),
            InstructionType::JP(test) => Ok(self.jump(test, prev_pc)),
            InstructionType::JR(test) => Ok(self.jump_relative(test, prev_pc)),
            InstructionType::JPHL => Ok(self.jump_hl()),
            InstructionType::NOP => Ok(self.nop()),
            InstructionType::HALT => Ok(self.halt()),
            InstructionType::SCF => Ok(self.scf()),
            InstructionType::CCF => Ok(self.ccf()),
            InstructionType::CPL => Ok(self.cpl()),
            InstructionType::ADD(target) => Ok(self.add(target, prev_pc)),
            InstructionType::ADC(target) => Ok(self.adc(target, prev_pc)),
            InstructionType::INC(target) => Ok(self.inc(target)),
            InstructionType::DEC(target) => Ok(self.dec(target)),
            InstructionType::ADD16(target) => Ok(self.add16(target)),
            InstructionType::INC16(target) => Ok(self.inc16(target)),
            InstructionType::DEC16(target) => Ok(self.dec16(target)),
            InstructionType::ADDSP8 => Ok(self.addsp8(prev_pc)),
            InstructionType::SUB(target) => Ok(self.sub(target, prev_pc)),
            InstructionType::SBC(target) => Ok(self.sbc(target, prev_pc)),
            InstructionType::AND(target) => Ok(self.and(target, prev_pc)),
            InstructionType::XOR(target) => Ok(self.xor(target, prev_pc)),
            InstructionType::OR(target) => Ok(self.or(target, prev_pc)),
            InstructionType::CP(target) => Ok(self.cp(target, prev_pc)),
            InstructionType::LD(load_type) => Ok(self.load(prev_pc, load_type)),
            InstructionType::LDSIG => Ok(self.ldsig(prev_pc)),
            InstructionType::LDSPHL => Ok(self.ldsphl()),
            InstructionType::LDSPA16 => Ok(self.ldspa16()),
            InstructionType::LDFF(load_type) => Ok(self.ldff(load_type, prev_pc)),
            InstructionType::PUSH(target) => Ok(self.push(target)),
            InstructionType::POP(target) => Ok(self.pop(target)),
            //InstructionType::RST(target) => todo!(),
            InstructionType::BIT(bit_type) => Ok(self.bit(bit_type)),
            //InstructionType::RETI => todo!(),
            //InstructionType::DAA => self.daa(),
            InstructionType::RL(target) => {
                Ok(self.bitwise_rotate(target, RotateDirection::Left, false)) 
            },
            InstructionType::RLC(target) => {
                Ok(self.bitwise_rotate(target, RotateDirection::Left, true)) 
            },
            InstructionType::RLA => Ok(self.rla()),
            InstructionType::RLCA => Ok(self.rlca()),
            InstructionType::SRA(target) => Ok(self.sla(target)),
            InstructionType::SLA(target) => Ok(self.sra(target)),
            InstructionType::SRL(target) => Ok(self.srl(target)),
            InstructionType::SWAP(target) => Ok(self.swap(target)),
            InstructionType::EI => Ok(self.ei()),
            InstructionType::DI => Ok(self.di()),
            InstructionType::RES(target) => Ok(self.res_set(target, false)),
            InstructionType::SET(target) => Ok(self.res_set(target, true)),
            _ => { Err(Error::new(ErrorKind::Other, "Unsupported instruction")) }
        };

        executed_cycles
    }

    fn jump(&mut self, test: JumpTest, current_pc: ProgramCounter) -> ClockCycles {
        let should_jump = self.should_jump(test);
     
        if should_jump {
            // Gameboy is little endian so read pc + 2 as most significant bit
            // and pc + 1 as least significant bit
            let least_significant_byte = self.mmu.read_byte(current_pc + 1) as u16;
            let most_significant_byte = self.mmu.read_byte(current_pc + 2) as u16;
            self.pc = (most_significant_byte << 8) | least_significant_byte;

            ClockCycles::Four
        } else {
            ClockCycles::Three
        }
    }

    fn jump_relative(&mut self, test: JumpTest, current_pc: ProgramCounter) -> ClockCycles {
        let should_jump = self.should_jump(test);
     
        if should_jump {
            let offset: i8 = self.read_next_byte(current_pc) as i8;
            self.pc = current_pc.wrapping_add(2i8.wrapping_add(offset) as u16);

            ClockCycles::Three
        } else {
            ClockCycles::Two
        }
    }

    fn jump_hl(&mut self) -> ClockCycles {
        self.pc = self.regs.get_hl();

        ClockCycles::One
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

    fn nop(&self) -> ClockCycles {
        ClockCycles::One
    }

    fn ei(&mut self) -> ClockCycles {
        self.ime = true;
        ClockCycles::One
    }

    fn di(&mut self) -> ClockCycles {
        self.ime = false;
        ClockCycles::One
    }

    fn scf(&mut self) -> ClockCycles {
        self.regs.flags.carry = true;
        self.regs.flags.subtract = false;
        self.regs.flags.half_carry = false;

        ClockCycles::One
    }

    fn cpl(&mut self) -> ClockCycles {
        self.regs.a = !self.regs.a; 
        self.regs.flags.subtract = true;
        self.regs.flags.half_carry = true;

        ClockCycles::One
    }

    fn ccf(&mut self) -> ClockCycles {
        self.regs.flags.carry = !self.regs.flags.carry;
        self.regs.flags.subtract = false;
        self.regs.flags.half_carry = false;

        ClockCycles::One
    }

    fn daa(&self) -> ClockCycles {
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

        ClockCycles::One
    }

    fn halt(&mut self) -> ClockCycles {
        self.is_halted = true;

        ClockCycles::One
    }

    fn call(&mut self, test: JumpTest, current_pc: ProgramCounter) -> ClockCycles {
        let should_jump = self.should_jump(test);

        if should_jump {
            self.push_value(current_pc.wrapping_add(3));
            self.pc = self.read_next_word(current_pc);

            ClockCycles::Six
        } else {
            ClockCycles::Three
        }
    }
    
    fn ret(&mut self, test: JumpTest) -> ClockCycles {
        let jump_condition = self.should_jump(test);
        self.return_(jump_condition)
    }

    fn return_(&mut self, should_jump: bool) -> ClockCycles {
        if should_jump {
            self.pc = self.pop_value();

            ClockCycles::Five
        } else {
            ClockCycles::Two
        }
    }

    fn ldsig(&mut self, current_pc: ProgramCounter) -> ClockCycles {
        let value: i16 = self.read_next_byte(current_pc) as i16;
        let (new_value, did_overflow) = self.sp.overflowing_add_signed(value);

        self.regs.flags.zero = false;
        self.regs.flags.subtract = false;
        self.regs.flags.carry = did_overflow;
        // TODO: Not sure about half-carry with signed
        self.regs.flags.half_carry = (self.sp & 0xF) + (value as u16 & 0xF) > 0xF;

        self.regs.set_hl(new_value);

        ClockCycles::Three        
    }

    fn ldsphl(&mut self) -> ClockCycles {
        self.sp = self.regs.get_hl();

        ClockCycles::Two
    }

    fn ldspa16(&mut self) -> ClockCycles {
        let address = self.read_next_word(self.pc);

        let lsb = (self.sp & 0x00FF) as u8;
        let msb = ((self.sp & 0xFF00) >> 8) as u8;

        self.mmu.write_byte(address, lsb);
        self.mmu.write_byte(address.wrapping_add(1), msb);

        ClockCycles::Five
    }

    pub(super) fn read_next_byte(&self, address: Address) -> u8 {
        self.mmu.read_byte(address+1)
    }

    pub(super) fn read_next_word(&self, address: Address) -> u16 {
        ((self.mmu.read_byte(address+2) as u16) << 8) | self.mmu.read_byte(address+1) as u16
    }

    fn load(&mut self, current_pc: ProgramCounter, load_type: LoadType) -> ClockCycles {
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
                    RegistersIndirect::HLI => self.mmu.write_byte(self.regs.get_hl(), source_value)
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
            LoadType::Byte(_,RegistersIndDir::HLI) => ClockCycles::Two,
            LoadType::Byte(RegistersIndirect::HLI, _) => ClockCycles::Two,
            LoadType::AFromIndirect(_) => ClockCycles::Two,
            LoadType::IndirectFromA(_) => ClockCycles::Two,
            LoadType::Word(_) => ClockCycles::Three,
            LoadType::AFromDirect => ClockCycles::Four,
            LoadType::DirectFromA => ClockCycles::Four,
            _ => ClockCycles::One,
        }
    }
    
    fn ldff(&mut self, load_type: LoadFFType, current_pc: Address) -> ClockCycles {
        match load_type {
            LoadFFType::AtoFFC => { 
                let addr: u16 = 0xFF00 + self.regs.c as u16;        
                self.mmu.write_byte(addr, self.regs.a);
                ClockCycles::Two
            },
            LoadFFType::FFCtoA => {
                let addr: u16 = 0xFF00 + self.regs.c as u16;        
                self.regs.a = self.mmu.read_byte(addr);
                ClockCycles::Two
            },
            LoadFFType::AtoFFa8 => {
                let addr: u16 = 0xFF00 + self.read_next_byte(current_pc) as u16;        
                self.mmu.write_byte(addr, self.regs.a);
                ClockCycles::Three
            },
            LoadFFType::FFa8toA => {
                let addr: u16 = 0xFF00 + self.read_next_byte(current_pc) as u16;        
                self.regs.a = self.mmu.read_byte(addr);
                ClockCycles::Three
            }
        }
    }

    pub(super) fn push(&mut self, target: StackTarget) -> ClockCycles {
        let value = match target {
            StackTarget::BC => self.regs.get_bc(),
            StackTarget::DE => self.regs.get_de(),
            StackTarget::HL => self.regs.get_hl(),
            StackTarget::AF => self.regs.get_af(),
        };
        self.push_value(value);

        ClockCycles::Four
    }

    pub(super) fn pop(&mut self, target: StackTarget) -> ClockCycles {
        let result = self.pop_value();
        match target {
            StackTarget::BC => self.regs.set_bc(result),
            StackTarget::DE => self.regs.set_de(result),
            StackTarget::HL => self.regs.set_hl(result),
            StackTarget::AF => self.regs.set_af(result),
        };

        ClockCycles::Three
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