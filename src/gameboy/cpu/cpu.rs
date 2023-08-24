use std::io::{Error, ErrorKind};
use std::fmt;

use crate::gameboy::io::io::{SERIAL_CONTROL_ADDRESS, SERIAL_DATA_ADDRESS};
use crate::gameboy::serial::SerialTransferMode;
use crate::gameboy::{mmu::{MMU, Address}, io::io::{IOEvent, INTERRUPT_FLAG_ADDRESS}};

use super::instructions::decode::{InstructionType, InstructionSize};
use super::instructions::instructions::{Instruction};
use super::{registers::Registers, timers::Timers};

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

    pub(crate) fn step(&mut self) -> Result<MachineCycles, Error> {
        if !self.is_halted {
            //self.handle_interrupts();
            let instruction = self.fetch_decode()?;
            //println!("{:?} {}", instruction, self);
            println!("{}", self);
            let result = instruction.execute(self)?;
            Ok(result)
        }else{
            Ok(MachineCycles::One)
        }
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

    pub(crate) fn output_event(&mut self) -> Option<IOEvent> {
        let serial_transfer = self.mmu.read_byte(SERIAL_CONTROL_ADDRESS);
        let serial_data = self.mmu.read_byte(SERIAL_DATA_ADDRESS);

        if serial_data != 0 {
            println!("serial {:x} {}", serial_transfer, serial_data as char);
            Some(IOEvent::SerialOutput(serial_data))
        }else{
            None
        }

    }

    pub(crate) fn handle_interrupts(&mut self) -> MachineCycles {
        if self.ime {
            match self.mmu.io.interrupts.interrupt_to_handle() {
                Some(interrupt) => {
                    self.is_halted = false;
                    self.ime = false;
                    self.push_stack(self.pc);
                    self.pc = interrupt.handler();
                    return MachineCycles::Five
                },
                _ => MachineCycles::Zero
            }
        }else{
            MachineCycles::Zero
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

    pub(crate) fn timer_interrupt(&mut self) {
        let prev_interruption_flag = self.mmu.read_byte(INTERRUPT_FLAG_ADDRESS);
        self.mmu.write_byte(INTERRUPT_FLAG_ADDRESS, prev_interruption_flag | 0b00000100);       
    }

    pub(super) fn read_next_byte(&self, address: Address) -> u8 {
        self.mmu.read_byte(address+1)
    }
    
    pub(super) fn read_next_word(&self, address: Address) -> u16 {
        ((self.mmu.read_byte(address+2) as u16) << 8) | (self.mmu.read_byte(address+1) as u16)
    }

    pub(crate) fn timer_tick(&mut self, cycles: u16) {
        self.timers.div_counter += cycles;

        if self.timers.div_counter >= 256 {
            self.timers.div_counter -= 256;
            self.timers.div = self.timers.div.wrapping_add(1);
        }

        if self.timers.is_enabled() {
            self.timers.counter += cycles;

            while self.timers.counter >= self.timers.get_frecuency() {
                let (new_tima, tima_overflow) = self.timers.tima.overflowing_add(1);
                self.timers.tima = new_tima;
                if tima_overflow {
                    self.timer_interrupt();
                    self.timers.tima = self.timers.tma;
                }

                self.timers.counter -= self.timers.get_frecuency();
            }
        }
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

impl std::convert::From<MachineCycles> for ClockCycles  {
    fn from(cycles: MachineCycles) -> ClockCycles {
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