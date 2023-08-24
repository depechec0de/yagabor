use std::io::{Error, ErrorKind};

use crate::gameboy::cpu::cpu::{MachineCycles, CPU};

use super::{alu::*, decode::{InstructionType, JumpTest, BitTarget, LoadType, RegistersIndDir, RegistersIndirect, WordRegister, LoadIndirectSource, LoadFFType, StackTarget, InstructionSize}};

#[derive(Debug, Clone)]
pub(crate) struct Instruction {
    pub(crate) op: InstructionType,
    pub(crate) size: InstructionSize,
    pub(crate) payload: Option<u16>
}

impl Instruction {

    pub(crate) fn new(op: InstructionType, payload: Option<u16>) -> Instruction {
        let size = op.size();

        Instruction { op, size, payload: payload }
    }

    pub(crate) fn execute(&self, cpu: &mut CPU) -> Result<MachineCycles, Error> {
        match self.op.clone() {
            InstructionType::CALL(test) => self.call(cpu, test),
            InstructionType::RET(test) => self.ret(cpu, test),
            InstructionType::JP(test) => self.jump(cpu, test),
            InstructionType::JR(test) => self.jump_relative(cpu, test),
            InstructionType::JPHL => self.jump_hl(cpu),
            InstructionType::NOP => self.nop(cpu),
            InstructionType::HALT => self.halt(cpu),
            InstructionType::SCF => self.scf(cpu),
            InstructionType::CCF => self.ccf(cpu),
            InstructionType::CPL => self.cpl(cpu),
            InstructionType::ADD(target) => self.add(cpu, target),
            InstructionType::ADC(target) => self.adc(cpu, target),
            InstructionType::INC(target) => self.inc(cpu, target),
            InstructionType::DEC(target) => self.dec(cpu, target),
            InstructionType::ADD16(target) => self.add16(cpu, target),
            InstructionType::INC16(target) => self.inc16(cpu, target),
            InstructionType::DEC16(target) => self.dec16(cpu, target),
            InstructionType::ADDSPS8 => self.addsps8(cpu),
            InstructionType::SUB(target) => self.sub(cpu, target),
            InstructionType::SBC(target) => self.sbc(cpu, target),
            InstructionType::AND(target) => self.and(cpu, target),
            InstructionType::XOR(target) => self.xor(cpu, target),
            InstructionType::OR(target) => self.or(cpu, target),
            InstructionType::CP(target) => self.cp(cpu, target),
            InstructionType::LD(load_type) => self.load(cpu, load_type),
            InstructionType::LDHLSPD8 => self.ldhlspd8(cpu),
            InstructionType::LDSPHL => self.ldsphl(cpu),
            InstructionType::LDSPA16 => self.ldspa16(cpu),
            InstructionType::LDFF(load_type) => self.ldff(cpu, load_type),
            InstructionType::PUSH(target) => self.push(cpu, target),
            InstructionType::POP(target) => self.pop(cpu, target),
            InstructionType::RST(target) => self.rst(cpu, target),
            InstructionType::BIT(bit_type) => self.bit(cpu, bit_type),
            InstructionType::RETI => self.reti(cpu),
            InstructionType::DAA => self.daa(cpu),
            InstructionType::RL(target) => self.rl(cpu, target),
            InstructionType::RLC(target) => self.rlc(cpu, target),
            InstructionType::RR(target) => self.rr(cpu, target),
            InstructionType::RRC(target) => self.rrc(cpu, target),
            InstructionType::RLA => self.rla(cpu),
            InstructionType::RLCA => self.rlca(cpu),
            InstructionType::RRA => self.rra(cpu),
            InstructionType::RRCA => self.rrca(cpu),
            InstructionType::SRA(target) => self.sra(cpu, target),
            InstructionType::SLA(target) => self.sla(cpu, target),
            InstructionType::SRL(target) => self.srl(cpu, target),
            InstructionType::SWAP(target) => self.swap(cpu, target),
            InstructionType::EI => self.ei(cpu),
            InstructionType::DI => self.di(cpu),
            InstructionType::RES(target) => self.res(cpu, target),
            InstructionType::SET(target) => self.set(cpu, target),
            InstructionType::STOP => panic!("STOP instruction"),
        }
    }    

    fn jump(&self, cpu: &mut CPU , test: JumpTest) -> Result<MachineCycles, Error> {
        let should_jump = should_jump(cpu, test);
     
        if should_jump {
            // Gameboy is little endian so read pc + 2 as most significant bit
            // and pc + 1 as least significant bit
            let least_significant_byte = cpu.mmu.read_byte(cpu.pc + 1) as u16;
            let most_significant_byte = cpu.mmu.read_byte(cpu.pc + 2) as u16;
            cpu.pc = (most_significant_byte << 8) | least_significant_byte;
    
            Ok(MachineCycles::Four)
        } else {
            cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
            Ok(MachineCycles::Three)
        }
    }
    
    fn jump_relative(&self, cpu: &mut CPU , test: JumpTest) -> Result<MachineCycles, Error> {
        let should_jump = should_jump(cpu, test);
     
        if should_jump {
            let offset: i8 = cpu.read_next_byte(cpu.pc) as i8;
            cpu.pc = cpu.pc.wrapping_add(2i8.wrapping_add(offset) as u16);
    
            Ok(MachineCycles::Three)
        } else {
            cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
            Ok(MachineCycles::Two) 
        }
    }
    
    fn jump_hl(&self, cpu: &mut CPU ) -> Result<MachineCycles, Error> {
        cpu.pc = cpu.regs.get_hl();
        Ok(MachineCycles::One)
    }   
        
    fn nop(&self, cpu: &mut CPU) -> Result<MachineCycles, Error> {
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
        Ok(MachineCycles::One)
    }
    
    fn ei(&self, cpu: &mut CPU ) -> Result<MachineCycles, Error> {
        cpu.ime = true;
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
        Ok(MachineCycles::One)
    }
    
    fn di(&self, cpu: &mut CPU ) -> Result<MachineCycles, Error> {
        cpu.ime = false;
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
        Ok(MachineCycles::One)
    }
    
    fn scf(&self, cpu: &mut CPU ) -> Result<MachineCycles, Error> {
        cpu.regs.flags.carry = true;
        cpu.regs.flags.subtract = false;
        cpu.regs.flags.half_carry = false;
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
        Ok(MachineCycles::One)
    }
    
    fn cpl(&self, cpu: &mut CPU ) -> Result<MachineCycles, Error> {
        cpu.regs.a = !cpu.regs.a; 
        cpu.regs.flags.subtract = true;
        cpu.regs.flags.half_carry = true;
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
        Ok(MachineCycles::One)
    }
    
    fn ccf(&self, cpu: &mut CPU ) -> Result<MachineCycles, Error> {
        cpu.regs.flags.carry = !cpu.regs.flags.carry;
        cpu.regs.flags.subtract = false;
        cpu.regs.flags.half_carry = false;
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
        Ok(MachineCycles::One)
    }
    
    // https://forums.nesdev.org/viewtopic.php?t=15944
    fn daa(&self, cpu: &mut CPU ) -> Result<MachineCycles, Error> {
        if !cpu.regs.flags.subtract {  // after an addition, adjust if (half-)carry occurred or if result is out of bounds
            if cpu.regs.flags.carry || cpu.regs.a > 0x99 { 
                cpu.regs.a = cpu.regs.a.wrapping_add(0x60);
                cpu.regs.flags.carry = true; 
            }
            if cpu.regs.flags.half_carry || (cpu.regs.a & 0x0f) > 0x09 { 
                cpu.regs.a = cpu.regs.a.wrapping_add(0x6); 
            }
        } else {  // after a subtraction, only adjust if (half-)carry occurred
            if cpu.regs.flags.carry { 
                cpu.regs.a = cpu.regs.a.wrapping_sub(0x60);
            }
            if cpu.regs.flags.half_carry { 
                cpu.regs.a = cpu.regs.a.wrapping_sub(0x06);
            }
        }
        // these flags are always updated
        cpu.regs.flags.zero = cpu.regs.a == 0; // the usual z flag
        cpu.regs.flags.half_carry = false; // h flag is always cleared
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
        Ok(MachineCycles::One)
    }
    
    fn call(&self, cpu: &mut CPU , test: JumpTest) -> Result<MachineCycles, Error> {
        let should_jump = should_jump(cpu, test);

        if should_jump {
            let jump_addr = cpu.read_next_word(cpu.pc);
            cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
            cpu.push_stack(cpu.pc);
            cpu.pc = jump_addr;
            Ok(MachineCycles::Six)
        } else {
            cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
            Ok(MachineCycles::Three)
        }
    }
    
    fn ret(&self, cpu: &mut CPU , test: JumpTest) -> Result<MachineCycles, Error> {
        let jump_condition = should_jump(cpu, test);
        if jump_condition {
            cpu.pc = cpu.pop_stack();
            Ok(MachineCycles::Five) 
        } else {
            cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
            Ok(MachineCycles::Two) 
        }
    }
    
    fn reti(&self, cpu: &mut CPU ) -> Result<MachineCycles, Error> {
        cpu.pc = cpu.pop_stack();
        cpu.ime = true;
        Ok(MachineCycles::Four)
    }
    
    fn rst(&self, cpu: &mut CPU , target: BitTarget) -> Result<MachineCycles, Error> {
        cpu.push_stack(cpu.pc.wrapping_add(u16::from(self.op.size())));
    
        let address: u16 = match target {
            BitTarget::Zero => 0x0000,
            BitTarget::One => 0x0008,
            BitTarget::Two => 0x0010,
            BitTarget::Three => 0x0018,
            BitTarget::Four => 0x0020,
            BitTarget::Five => 0x0028,
            BitTarget::Six => 0x0030,
            BitTarget::Seven => 0x0038,
        };
        cpu.pc = address; 
    
        Ok(MachineCycles::Four)
    }
    
    fn ldhlspd8(&self, cpu: &mut CPU ) -> Result<MachineCycles, Error> {
        let value = cpu.read_next_byte(cpu.pc) as i8 as u16;
        let new_value = cpu.sp.wrapping_add(value);
    
        cpu.regs.flags.zero = false;
        cpu.regs.flags.subtract = false;
        cpu.regs.flags.carry = (cpu.sp & 0xFF).wrapping_add(value as u16 & 0xFF) > 0xFF;
        cpu.regs.flags.half_carry = (cpu.sp & 0xF) + (value as u16 & 0xF) > 0xF;
    
        cpu.regs.set_hl(new_value);
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
        Ok(MachineCycles::Three)        
    }
    
    fn ldsphl(&self, cpu: &mut CPU ) -> Result<MachineCycles, Error> {
        cpu.sp = cpu.regs.get_hl();
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
        Ok(MachineCycles::Two) 
    }
    
    fn ldspa16(&self, cpu: &mut CPU) -> Result<MachineCycles, Error> {
        let address = cpu.read_next_word(cpu.pc);
    
        let lsb = (cpu.sp & 0x00FF) as u8;
        let msb = ((cpu.sp & 0xFF00) >> 8) as u8;

        cpu.mmu.write_byte(address, lsb);
        cpu.mmu.write_byte(address.wrapping_add(1), msb);
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
        Ok(MachineCycles::Five) 
    }
    
    fn load(&self, cpu: &mut CPU , load_type: LoadType) -> Result<MachineCycles, Error> {
         
        match &load_type {
            LoadType::Byte(target, source) => {
                let source_value = match source {
                    RegistersIndDir::A   => cpu.regs.a,
                    RegistersIndDir::B   => cpu.regs.b,
                    RegistersIndDir::C   => cpu.regs.c,
                    RegistersIndDir::D   => cpu.regs.d,
                    RegistersIndDir::E   => cpu.regs.e,
                    RegistersIndDir::H   => cpu.regs.h,
                    RegistersIndDir::L   => cpu.regs.l,
                    RegistersIndDir::D8  => cpu.read_next_byte(cpu.pc),
                    RegistersIndDir::HLI => cpu.mmu.read_byte(cpu.regs.get_hl())
                };
                match target {
                    RegistersIndirect::A   => cpu.regs.a = source_value,
                    RegistersIndirect::B   => cpu.regs.b = source_value,
                    RegistersIndirect::C   => cpu.regs.c = source_value,
                    RegistersIndirect::D   => cpu.regs.d = source_value,
                    RegistersIndirect::E   => cpu.regs.e = source_value,
                    RegistersIndirect::H   => cpu.regs.h = source_value,
                    RegistersIndirect::L   => cpu.regs.l = source_value,
                    RegistersIndirect::HLI => cpu.mmu.write_byte(cpu.regs.get_hl(), source_value)
                };
    
            },
            LoadType::Word(target) => {
                match target {
                    WordRegister::BC => {
                        cpu.regs.set_bc(cpu.read_next_word(cpu.pc));
                    },
                    WordRegister::DE => {
                        cpu.regs.set_de(cpu.read_next_word(cpu.pc));
                    },
                    WordRegister::HL => {
                        cpu.regs.set_hl(cpu.read_next_word(cpu.pc));
                    },
                    WordRegister::SP => {
                        cpu.sp = cpu.read_next_word(cpu.pc);
                    }
                }
            },
            LoadType::AFromIndirect(target) => {
                match target {
                    LoadIndirectSource::BC => {
                        let addr = cpu.regs.get_bc();
                        cpu.regs.a = cpu.mmu.read_byte(addr);
                    },
                    LoadIndirectSource::DE => {
                        let addr = cpu.regs.get_de();
                        cpu.regs.a = cpu.mmu.read_byte(addr);
                    },
                    LoadIndirectSource::HLInc => {
                        let addr = cpu.regs.get_hl();
                        cpu.regs.a = cpu.mmu.read_byte(addr);
                        let new_value = cpu.regs.get_hl().wrapping_add(1);
                        cpu.regs.set_hl(new_value);
                    },
                    LoadIndirectSource::HLDec => {
                        let addr = cpu.regs.get_hl();
                        cpu.regs.a = cpu.mmu.read_byte(addr);
                        let new_value = cpu.regs.get_hl().wrapping_sub(1);
                        cpu.regs.set_hl(new_value);
                    }
                }
            },
            LoadType::IndirectFromA(target) => {
                match target {
                    LoadIndirectSource::BC => {
                        let addr = cpu.regs.get_bc();
                        cpu.mmu.write_byte(addr, cpu.regs.a);
                    },
                    LoadIndirectSource::DE => {
                        let addr = cpu.regs.get_de();
                        cpu.mmu.write_byte(addr, cpu.regs.a);
                    },
                    LoadIndirectSource::HLInc => {
                        let addr = cpu.regs.get_hl();
                        cpu.mmu.write_byte(addr, cpu.regs.a);
                        let new_value = cpu.regs.get_hl().wrapping_add(1);
                        cpu.regs.set_hl(new_value);
                    },
                    LoadIndirectSource::HLDec => {
                        let addr = cpu.regs.get_hl();
                        cpu.mmu.write_byte(addr, cpu.regs.a);
                        let new_value = cpu.regs.get_hl().wrapping_sub(1);
                        cpu.regs.set_hl(new_value);
                    }
                }
            },
            LoadType::AFromDirect => {
                let address = cpu.read_next_word(cpu.pc);
                cpu.regs.a = cpu.mmu.read_byte(address);
            },
            LoadType::DirectFromA => {
                cpu.mmu.write_byte(cpu.read_next_word(cpu.pc), cpu.regs.a);
            }
        }

        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
    
        // Result
        match load_type {
            LoadType::Byte(_,RegistersIndDir::HLI) => Ok(MachineCycles::Two) ,
            LoadType::Byte(RegistersIndirect::HLI, _) => Ok(MachineCycles::Two) ,
            LoadType::AFromIndirect(_) => Ok(MachineCycles::Two) ,
            LoadType::IndirectFromA(_) => Ok(MachineCycles::Two) ,
            LoadType::Word(_) => Ok(MachineCycles::Three),
            LoadType::AFromDirect => Ok(MachineCycles::Four),
            LoadType::DirectFromA => Ok(MachineCycles::Four),
            _ => Ok(MachineCycles::One),
        }
    }
    
    fn ldff(&self, cpu: &mut CPU , load_type: LoadFFType) -> Result<MachineCycles, Error> {

        match load_type {
            LoadFFType::AtoFFC => { 
                let addr: u16 = 0xFF00 + cpu.regs.c as u16;       
                cpu.mmu.write_byte(addr, cpu.regs.a);
                cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
                Ok(MachineCycles::Two) 
            },
            LoadFFType::FFCtoA => {
                let addr: u16 = 0xFF00 + cpu.regs.c as u16;        
                cpu.regs.a = cpu.mmu.read_byte(addr);
                cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
                Ok(MachineCycles::Two) 
            },
            LoadFFType::AtoFFa8 => {
                let addr: u16 = 0xFF00 + cpu.read_next_byte(cpu.pc) as u16;        
                cpu.mmu.write_byte(addr, cpu.regs.a);
                cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
                Ok(MachineCycles::Three)
            },
            LoadFFType::FFa8toA => {
                let addr: u16 = 0xFF00 + cpu.read_next_byte(cpu.pc) as u16;        
                cpu.regs.a = cpu.mmu.read_byte(addr);
                cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
                Ok(MachineCycles::Three)
            }
        }
    }
    
    pub(crate) fn push(&self, cpu: &mut CPU , target: StackTarget) -> Result<MachineCycles, Error> {
        let value = match target {
            StackTarget::BC => cpu.regs.get_bc(),
            StackTarget::DE => cpu.regs.get_de(),
            StackTarget::HL => cpu.regs.get_hl(),
            StackTarget::AF => cpu.regs.get_af(),
        };
        cpu.push_stack(value);
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
        Ok(MachineCycles::Four)
    }
    
    pub(crate) fn pop(&self, cpu: &mut CPU , target: StackTarget) -> Result<MachineCycles, Error> {
        let result = cpu.pop_stack();
        match target {
            StackTarget::BC => cpu.regs.set_bc(result),
            StackTarget::DE => cpu.regs.set_de(result),
            StackTarget::HL => cpu.regs.set_hl(result),
            StackTarget::AF => cpu.regs.set_af(result),
        };
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
        Ok(MachineCycles::Three)
    }
    
    fn halt(&self, cpu: &mut CPU ) -> Result<MachineCycles, Error> {
        if cpu.mmu.io.interrupts.some_interrupt_enabled() {
            if !cpu.ime {
                // Halt bug, no PC increment
            }else{
                // We ignore the halting
                cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
            }
        }else{
            cpu.is_halted = true;
            cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
        }
        
        Ok(MachineCycles::One)
    }

}

fn should_jump(cpu: &CPU, test: JumpTest) -> bool {
    match test {
        JumpTest::NotZero => !cpu.regs.flags.zero,
        JumpTest::NotCarry => !cpu.regs.flags.carry,
        JumpTest::Zero => cpu.regs.flags.zero,
        JumpTest::Carry => cpu.regs.flags.carry,
        JumpTest::Always => true
    }
}
