use std::io::{Error, ErrorKind};
use std::fmt;

use crate::gameboy::io::interrupts::Interruption;
use crate::gameboy::io::io::{SERIAL_CONTROL_ADDRESS, SERIAL_DATA_ADDRESS};
use crate::gameboy::serial::SerialTransferMode;
use crate::gameboy::{mmu::{MMU, Address}, io::io::{IOEvent, INTERRUPT_FLAG_ADDRESS}};

use super::instructions::decode::{InstructionType, InstructionSize};
use super::instructions::instructions::{Instruction};
use super::{registers::Registers};

pub(crate) type ProgramCounter = Address;
pub(crate) type StackPointer = Address;
pub(crate) type ClockCycles = u16;

pub(crate) struct CPU{
    pub(super) regs: Registers,
    pub(super) sp: StackPointer,
    pub(super) pc: ProgramCounter,
    pub(super) is_halted: bool,
    pub(super) ime: bool,
    pub(super) mmu: MMU,
    pub(super) div_counter: u8,
    // Because the max frecuency is 1024 => 10 bits
    pub(super) tima_counter: u16, 
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
            div_counter: 0,
            tima_counter: 0
        }
    }

    pub(crate) fn step(&mut self) -> Result<ClockCycles, Error> {
        let mut mcycles = MachineCycles::One;

        self.handle_interrupts();
        
        if !self.is_halted {
            let instruction = self.fetch_decode()?;
            //println!("{:?} {}", instruction, self);
            //println!("{}", self);
            mcycles = instruction.execute(self)?;           
        }

        self.tick_timers(u8::from(mcycles.clone()));        

        Ok(ClockCycles::from(mcycles))
    }   

    pub(super) fn fetch_decode(&self) -> Result<Instruction, Error> {
        let instruction_byte = self.mmu.read_byte(self.pc);
        let byte0 = self.mmu.read_byte(self.pc+1);
        let byte1 = self.mmu.read_byte(self.pc+2);

        let prefixed = instruction_byte == 0xCB;
        let mut instruction_byte = instruction_byte;
        if prefixed {
            instruction_byte = byte0;
        }

        let inst_type: Option<InstructionType>;

        if prefixed {
            inst_type = InstructionType::from_byte_prefixed(instruction_byte)
        } else {
            inst_type = InstructionType::from_byte_not_prefixed(instruction_byte)
        }

        if let Some(op) = inst_type {
            let payload = match op.size() {
                InstructionSize::OneByte => None,
                InstructionSize::TwoBytes => Some(byte0 as u16),
                InstructionSize::ThreeBytes => Some(((byte0 as u16) << 8) | byte1 as u16),
            };

            Ok(Instruction::new(op, payload))
        }else{
            Err(Error::new(ErrorKind::Other, format!("Unkown instruction {:x} {:x} found", instruction_byte, byte0)))
        }        
    }

    pub(crate) fn send_serial(&mut self) -> Option<u8> {
        let serial_transfer = SerialTransferMode::parse_from_byte(self.mmu.read_byte(SERIAL_CONTROL_ADDRESS));
        let serial_data = self.mmu.read_byte(SERIAL_DATA_ADDRESS);

        match serial_transfer {
            SerialTransferMode::TransferInternalClock => Some(serial_data),
            SerialTransferMode::TransferExternalClock => Some(serial_data),
            _ => None
        }
    }

    pub(crate) fn handle_interrupts(&mut self) {
        if self.mmu.io.interrupts.some_interrupt_enabled() {
            if self.ime {
                let interrupt = self.mmu.io.interrupts.interrupt_to_handle().unwrap();
                self.is_halted = false;
                self.ime = false;
                self.push_stack(self.pc);
                self.pc = interrupt.handler();
            }else{
                self.is_halted = false;
            }
        }
    }

    pub(crate) fn tick_timers(&mut self, cycles: u8) {

        let (new_div, div_overflow) = self.div_counter.overflowing_add(cycles);

        if div_overflow {
            self.div_counter = new_div;
            self.mmu.io.tick_div();
        }

        if self.timer_enabled() {
            self.tima_counter = self.tima_counter.wrapping_add(cycles as u16);

            while self.tima_counter >= self.timer_frecuency() {
                let tima_overflow = self.mmu.io.tick_tima();
                if tima_overflow {
                    self.mmu.io.interrupts.turnon(Interruption::Timer);
                    self.mmu.io.reset_tima();
                }
                self.tima_counter -= self.timer_frecuency();
            }
        }
    }
    
    pub(crate) fn timer_enabled(&self) -> bool {
        // if bit 2 is high, timer is enabled 
        self.mmu.io.get_timers_tac() & 0b00000100 > 0
    }

    pub(crate) fn timer_frecuency(&self) -> u16 {
        let tac = self.mmu.io.get_timers_tac();
        if tac & 0b00000011 == 0 {
            1024
        }else if tac & 0b00000011 == 1 {
            16
        }else if tac & 0b00000011 == 2 {
            64
        }else {
            256
        }
    }

    pub(crate) fn push_stack(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.mmu.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.mmu.write_byte(self.sp, (value & 0xFF) as u8);
    }
    
    pub(crate) fn pop_stack(&mut self) -> u16 {
        let lsb = self.mmu.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
    
        let msb = self.mmu.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
    
        (msb << 8) | lsb
    }

    pub(super) fn read_next_byte(&self, address: Address) -> u8 {
        self.mmu.read_byte(address+1)
    }
    
    pub(super) fn read_next_word(&self, address: Address) -> u16 {
        ((self.mmu.read_byte(address+2) as u16) << 8) | (self.mmu.read_byte(address+1) as u16)
    }

    pub(crate) fn ack_sent_serial(&mut self){
        self.mmu.io.interrupts.turnon(Interruption::Serial);
        self.mmu.io.serial_control_clear();
    }
    
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A:{:02X} \
            F:{:02X} \
            B:{:02X} \
            C:{:02X} \
            D:{:02X} \
            E:{:02X} \
            H:{:02X} \
            L:{:02X} \
            SP:{:04X} \
            PC:{:04X} \
            PCMEM:{:02X},{:02X},{:02X},{:02X}", 
            self.regs.a, 
            u8::from(self.regs.flags.clone()), 
            self.regs.b, 
            self.regs.c, 
            self.regs.d, 
            self.regs.e, 
            self.regs.h, 
            self.regs.l, 
            self.sp, 
            self.pc, 
            self.mmu.read_byte(self.pc), 
            self.mmu.read_byte(self.pc+1), 
            self.mmu.read_byte(self.pc+2), 
            self.mmu.read_byte(self.pc+3) )
    }
}

  

// We use machine cycles for reference, but in the translation we multiply by 4
#[derive(Debug, Clone)]
pub(crate) enum MachineCycles {
    Zero, One, Two, Three, Four, Five, Six
}

impl std::convert::From<MachineCycles> for u8  {
    fn from(cycles: MachineCycles) -> u8 {
        let machine_cycles = match cycles {
            MachineCycles::Zero => 0,
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

impl std::convert::From<MachineCycles> for ClockCycles  {
    fn from(cycles: MachineCycles) -> ClockCycles {
        u8::from(cycles) as ClockCycles
    }
}