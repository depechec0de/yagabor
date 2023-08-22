use std::io::{Error, ErrorKind};

use crate::gameboy::{mmu::{MMU, Address}, io::io::{IOEvent}};

use super::{registers::Registers, instructions::*, timers::Timers};

pub(crate) type ProgramCounter = Address;
pub(crate) type StackPointer = Address;
pub(crate) type ClockCycles = u8;

pub(crate) struct CPU{
    pub(super) regs: Registers,
    pub(super) sp: StackPointer,
    pub(super) pc: ProgramCounter,
    pub(super) is_halted: bool,
    pub(super) ime: bool,
    pub(super) mmu: MMU,
    pub(crate) timers: Timers,
}

#[derive(Debug)]
pub(crate) struct ExecResult{
    pub(crate) event: Option<IOEvent>,
    pub(crate) clockcycles: ClockCycles,
}

impl ExecResult {
    pub(crate) fn new(event: Option<IOEvent>, cycles: MachineCycles) -> Self {
        ExecResult { event, clockcycles: cycles as ClockCycles }
    }
}

impl CPU {
    pub fn new(mmu: MMU) -> Self {
        Self { 
            regs: Registers::new(), 
            sp: 0x0, 
            pc: 0x0,  
            is_halted: false,
            ime: true,
            mmu,
            timers: Timers::new(),
        }
    }

    pub(crate) fn step(&mut self) -> Result<ExecResult, Error> {

        let instruction_byte = self.mmu.read_byte(self.pc);
        let byte0 = self.mmu.read_byte(self.pc+1);
        let byte1 = self.mmu.read_byte(self.pc+2);

        if let Some(instruction) = Instruction::parse_instruction(instruction_byte, byte0, byte1) {
            //println!("{:?}", instruction);
            match self.execute(instruction.clone()) {
                Ok(result) => {
                    Ok(result)
                },
                Err(error) => {
                    Err(Error::new(ErrorKind::Other, format!("Error during execution: {}", error)))
                }
            }
        } else {
            //println!("{}", self.mmu);
            Err(Error::new(ErrorKind::Other, format!("Unkown instruction {:x} {:x} found", instruction_byte, byte0)))
        }   
    }   

    pub(super) fn execute(&mut self, instruction: Instruction) -> Result<ExecResult, Error> {

        let inst_type = instruction.op.clone();
        let inst_size = instruction.size_bytes();

        let prev_pc = self.pc;
        self.pc = self.pc.wrapping_add(inst_size as u16);

        match inst_type {
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
            InstructionType::ADDSPS8 => self.addsps8(prev_pc),
            InstructionType::SUB(target) => self.sub(target, prev_pc),
            InstructionType::SBC(target) => self.sbc(target, prev_pc),
            InstructionType::AND(target) => self.and(target, prev_pc),
            InstructionType::XOR(target) => self.xor(target, prev_pc),
            InstructionType::OR(target) => self.or(target, prev_pc),
            InstructionType::CP(target) => self.cp(target, prev_pc),
            InstructionType::LD(load_type) => self.load(prev_pc, load_type),
            InstructionType::LDHLSPD8 => self.ldhlspd8(prev_pc),
            InstructionType::LDSPHL => self.ldsphl(),
            InstructionType::LDSPA16 => self.ldspa16(prev_pc),
            InstructionType::LDFF(load_type) => self.ldff(load_type, prev_pc),
            InstructionType::PUSH(target) => self.push(target),
            InstructionType::POP(target) => self.pop(target),
            InstructionType::RST(target) => self.rst(target, prev_pc),
            InstructionType::BIT(bit_type) => self.bit(bit_type),
            InstructionType::RETI => self.reti(),
            InstructionType::DAA => self.daa(),
            InstructionType::RL(target) => self.rl(target),
            InstructionType::RLC(target) => self.rlc(target),
            InstructionType::RR(target) => self.rr(target),
            InstructionType::RRC(target) => self.rrc(target),
            InstructionType::RLA => self.rla(),
            InstructionType::RLCA => self.rlca(),
            InstructionType::RRA => self.rra(),
            InstructionType::RRCA => self.rrca(),
            InstructionType::SRA(target) => self.sra(target),
            InstructionType::SLA(target) => self.sla(target),
            InstructionType::SRL(target) => self.srl(target),
            InstructionType::SWAP(target) => self.swap(target),
            InstructionType::EI => self.ei(),
            InstructionType::DI => self.di(),
            InstructionType::RES(target) => self.res_set(target, false),
            InstructionType::SET(target) => self.res_set(target, true),
            _ => { Err(Error::new(ErrorKind::Other, format!("Unsupported instruction {:?}", inst_type))) }
        }
    }

    pub(crate) fn disable_interrupts(&mut self) {
        self.ime = false;
    }

    fn jump(&mut self, test: JumpTest, current_pc: ProgramCounter) -> Result<ExecResult, Error> {
        let should_jump = self.should_jump(test);
     
        if should_jump {
            // Gameboy is little endian so read pc + 2 as most significant bit
            // and pc + 1 as least significant bit
            let least_significant_byte = self.mmu.read_byte(current_pc + 1) as u16;
            let most_significant_byte = self.mmu.read_byte(current_pc + 2) as u16;
            self.pc = (most_significant_byte << 8) | least_significant_byte;

            Ok(ExecResult::new(None, MachineCycles::Four))
        } else {
            Ok(ExecResult::new(None, MachineCycles::Three))
        }
    }

    fn jump_relative(&mut self, test: JumpTest, current_pc: ProgramCounter) -> Result<ExecResult, Error> {
        let should_jump = self.should_jump(test);
     
        if should_jump {
            let offset: i8 = self.read_next_byte(current_pc) as i8;
            self.pc = current_pc.wrapping_add(2i8.wrapping_add(offset) as u16);

            Ok(ExecResult::new(None, MachineCycles::Three))
        } else {
            Ok(ExecResult::new(None, MachineCycles::Two))
        }
    }

    fn jump_hl(&mut self) -> Result<ExecResult, Error> {
        self.pc = self.regs.get_hl();

        Ok(ExecResult::new(None, MachineCycles::One))
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

    fn nop(&self) -> Result<ExecResult, Error> {
        Ok(ExecResult::new(None, MachineCycles::One))
    }

    fn ei(&mut self) -> Result<ExecResult, Error> {
        self.ime = true;
        Ok(ExecResult::new(None, MachineCycles::One))
    }

    fn di(&mut self) -> Result<ExecResult, Error> {
        self.ime = false;
        Ok(ExecResult::new(None, MachineCycles::One))
    }

    fn scf(&mut self) -> Result<ExecResult, Error> {
        self.regs.flags.carry = true;
        self.regs.flags.subtract = false;
        self.regs.flags.half_carry = false;

        Ok(ExecResult::new(None, MachineCycles::One))
    }

    fn cpl(&mut self) -> Result<ExecResult, Error> {
        self.regs.a = !self.regs.a; 
        self.regs.flags.subtract = true;
        self.regs.flags.half_carry = true;

        Ok(ExecResult::new(None, MachineCycles::One))
    }

    fn ccf(&mut self) -> Result<ExecResult, Error> {
        self.regs.flags.carry = !self.regs.flags.carry;
        self.regs.flags.subtract = false;
        self.regs.flags.half_carry = false;

        Ok(ExecResult::new(None, MachineCycles::One))
    }

    // https://forums.nesdev.org/viewtopic.php?t=15944
    fn daa(&mut self) -> Result<ExecResult, Error> {
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

        Ok(ExecResult::new(None, MachineCycles::One))
    }

    fn halt(&mut self) -> Result<ExecResult, Error> {
        self.is_halted = true;

        Ok(ExecResult::new(None, MachineCycles::One))
    }

    fn call(&mut self, test: JumpTest, current_pc: ProgramCounter) -> Result<ExecResult, Error> {
        let should_jump = self.should_jump(test);

        if should_jump {
            self.call_func(current_pc.wrapping_add(3), self.read_next_word(current_pc));

            Ok(ExecResult::new(None, MachineCycles::Six))
        } else {
            Ok(ExecResult::new(None, MachineCycles::Three))
        }
    }

    fn call_func(&mut self, current_pc: ProgramCounter, jump_addr: ProgramCounter){
        self.push_value(current_pc);
        self.pc = jump_addr;
    }
    
    fn ret(&mut self, test: JumpTest) -> Result<ExecResult, Error> {
        let jump_condition = self.should_jump(test);
        self.return_(jump_condition)
    }

    fn reti(&mut self) -> Result<ExecResult, Error> {
        self.pc = self.pop_value();
        self.ime = true;

        Ok(ExecResult::new(None, MachineCycles::Four))
    }

    fn rst(&mut self, target: BitTarget, current_pc: Address) -> Result<ExecResult, Error> {
        self.push_value(current_pc.wrapping_add(1));

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

        self.pc = address; 

        Ok(ExecResult::new(None, MachineCycles::Four))
    }

    fn return_(&mut self, should_jump: bool) -> Result<ExecResult, Error> {
        if should_jump {
            self.pc = self.pop_value();

            Ok(ExecResult::new(None, MachineCycles::Five))
        } else {
            Ok(ExecResult::new(None, MachineCycles::Two))
        }
    }

    fn ldhlspd8(&mut self, current_pc: ProgramCounter) -> Result<ExecResult, Error> {
        let value = self.read_next_byte(current_pc) as i8 as u16;
        let new_value = self.sp.wrapping_add(value);

        self.regs.flags.zero = false;
        self.regs.flags.subtract = false;
        self.regs.flags.carry = (self.sp & 0xFF).wrapping_add(value as u16 & 0xFF) > 0xFF;
        self.regs.flags.half_carry = (self.sp & 0xF) + (value as u16 & 0xF) > 0xF;

        self.regs.set_hl(new_value);

        Ok(ExecResult::new(None, MachineCycles::Three))        
    }

    fn ldsphl(&mut self) -> Result<ExecResult, Error> {
        self.sp = self.regs.get_hl();

        Ok(ExecResult::new(None, MachineCycles::Two))
    }

    fn ldspa16(&mut self, current_pc: Address) -> Result<ExecResult, Error> {
        let event: Option<IOEvent>;

        let address = self.read_next_word(current_pc);

        let lsb = (self.sp & 0x00FF) as u8;
        let msb = ((self.sp & 0xFF00) >> 8) as u8;

        self.mmu.write_byte(address, lsb);
        event = self.mmu.write_byte(address.wrapping_add(1), msb);

        Ok(ExecResult::new(event, MachineCycles::Five))
    }

    pub(super) fn read_next_byte(&self, address: Address) -> u8 {
        self.mmu.read_byte(address+1)
    }

    pub(super) fn read_next_word(&self, address: Address) -> u16 {
        ((self.mmu.read_byte(address+2) as u16) << 8) | (self.mmu.read_byte(address+1) as u16)
    }

    fn load(&mut self, current_pc: ProgramCounter, load_type: LoadType) -> Result<ExecResult, Error> {

        let mut event: Option<IOEvent> = None;

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
                    RegistersIndirect::HLI => { event = self.mmu.write_byte(self.regs.get_hl(), source_value); }
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
                        event = self.mmu.write_byte(addr, self.regs.a);
                    },
                    LoadIndirectSource::DE => {
                        let addr = self.regs.get_de();
                        event = self.mmu.write_byte(addr, self.regs.a);
                    },
                    LoadIndirectSource::HLInc => {
                        let addr = self.regs.get_hl();
                        event = self.mmu.write_byte(addr, self.regs.a);
                        let new_value = self.regs.get_hl().wrapping_add(1);
                        self.regs.set_hl(new_value);
                    },
                    LoadIndirectSource::HLDec => {
                        let addr = self.regs.get_hl();
                        event = self.mmu.write_byte(addr, self.regs.a);
                        let new_value = self.regs.get_hl().wrapping_sub(1);
                        self.regs.set_hl(new_value);
                    }
                }
            },
            LoadType::AFromDirect => {
                let address = self.read_next_word(current_pc);
                self.regs.a = self.mmu.read_byte(address);
            },
            LoadType::DirectFromA => {
                event = self.mmu.write_byte(self.read_next_word(current_pc), self.regs.a);
            }
        }

        // Result
        match load_type {
            LoadType::Byte(_,RegistersIndDir::HLI) => Ok(ExecResult::new(event, MachineCycles::Two)),
            LoadType::Byte(RegistersIndirect::HLI, _) => Ok(ExecResult::new(event, MachineCycles::Two)),
            LoadType::AFromIndirect(_) => Ok(ExecResult::new(event, MachineCycles::Two)),
            LoadType::IndirectFromA(_) => Ok(ExecResult::new(event, MachineCycles::Two)),
            LoadType::Word(_) => Ok(ExecResult::new(event, MachineCycles::Three)),
            LoadType::AFromDirect => Ok(ExecResult::new(event, MachineCycles::Four)),
            LoadType::DirectFromA => Ok(ExecResult::new(event, MachineCycles::Four)),
            _ => Ok(ExecResult::new(event, MachineCycles::One)),
        }
    }
    
    fn ldff(&mut self, load_type: LoadFFType, current_pc: Address) -> Result<ExecResult, Error> {

        let mut event: Option<IOEvent> = None;

        match load_type {
            LoadFFType::AtoFFC => { 
                let addr: u16 = 0xFF00 + self.regs.c as u16;        
                event = self.mmu.write_byte(addr, self.regs.a);
                Ok(ExecResult::new(event, MachineCycles::Two))
            },
            LoadFFType::FFCtoA => {
                let addr: u16 = 0xFF00 + self.regs.c as u16;        
                self.regs.a = self.mmu.read_byte(addr);
                Ok(ExecResult::new(event, MachineCycles::Two))
            },
            LoadFFType::AtoFFa8 => {
                let addr: u16 = 0xFF00 + self.read_next_byte(current_pc) as u16;        
                event = self.mmu.write_byte(addr, self.regs.a);
                Ok(ExecResult::new(event, MachineCycles::Three))
            },
            LoadFFType::FFa8toA => {
                let addr: u16 = 0xFF00 + self.read_next_byte(current_pc) as u16;        
                self.regs.a = self.mmu.read_byte(addr);
                Ok(ExecResult::new(event, MachineCycles::Three))
            }
        }
    }

    pub(super) fn push(&mut self, target: StackTarget) -> Result<ExecResult, Error> {
        let value = match target {
            StackTarget::BC => self.regs.get_bc(),
            StackTarget::DE => self.regs.get_de(),
            StackTarget::HL => self.regs.get_hl(),
            StackTarget::AF => self.regs.get_af(),
        };
        self.push_value(value);

        Ok(ExecResult::new(None, MachineCycles::Four))
    }

    pub(super) fn pop(&mut self, target: StackTarget) -> Result<ExecResult, Error> {
        let result = self.pop_value();
        match target {
            StackTarget::BC => self.regs.set_bc(result),
            StackTarget::DE => self.regs.set_de(result),
            StackTarget::HL => self.regs.set_hl(result),
            StackTarget::AF => self.regs.set_af(result),
        };

        Ok(ExecResult::new(None, MachineCycles::Three))
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

    pub(crate) fn handle_interrupts(&mut self) -> MachineCycles {
        if self.ime {
            if let Some(interrupt) = self.mmu.io.interrupts.interrupt_to_handle(){
                self.ime = false;
                self.call_func(self.pc, interrupt.handler());
            }
        }
        // TODO: Check this value
        MachineCycles::Six
    }

    pub(crate) fn handle_event(&mut self, event: IOEvent) -> Option<IOEvent> {
        match event {
            IOEvent::TimerControl(value) => {
                self.timers.tac = value;
                None
            },
            IOEvent::TimerTMA(value) => {
                self.timers.tma = value;
                None
            },
            IOEvent::TimerTIMA(value) => {
                self.timers.tima = value;
                None
            },
            IOEvent::TimerDIV(_) => {
                // Any write resets this - PanDocs
                self.timers.div = 0;
                None
            },
            // A non handled IOEvent is an external event, e.g. serial output
            _ => Some(event)
        }
    }
}

// We use machine cycles for reference, but in the translation we multiply by 4
#[derive(Debug, Clone)]
pub(crate) enum MachineCycles {
    One, Two, Three, Four, Five, Six
}

impl std::convert::From<MachineCycles> for ClockCycles  {
    fn from(cycles: MachineCycles) -> ClockCycles {
        let machine_cycles = match cycles {
            MachineCycles::One => 1,
            MachineCycles::Two => 2,
            MachineCycles::Three => 3,
            MachineCycles::Four => 4,
            MachineCycles::Five => 5,
            MachineCycles::Six => 6
        };
        machine_cycles*4
    }
}