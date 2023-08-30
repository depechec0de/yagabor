use std::io::{Error, ErrorKind};

use crate::gameboy::gameboy::GameBoy;
use crate::gameboy::io::interrupts::{Interruption, Interrupts};
use crate::gameboy::io::io::{SERIAL_CONTROL_ADDRESS, SERIAL_DATA_ADDRESS, SerialTransferMode};
use crate::gameboy::io::timers::Timers;
use crate::gameboy::{mmu::{MMU, Address}, io::io::{INTERRUPT_FLAG_ADDRESS}};

use super::instructions::decode::{InstructionType, InstructionSize};
use super::instructions::instructions::{Instruction};
use super::{registers::Registers};

pub(crate) type ProgramCounter = Address;
pub(crate) type StackPointer = Address;
pub(crate) type ClockCycles = u16;

pub(crate) struct CPU{
    pub(crate) regs: Registers,
    pub(crate) sp: StackPointer,
    pub(crate) pc: ProgramCounter,
    pub(crate) is_halted: bool,
    pub(crate) ime: bool,
}

impl CPU {
    pub fn new() -> Self {
        Self { 
            regs: Registers::new(), 
            sp: 0x0, 
            pc: 0x0,  
            is_halted: false,
            ime: true,
        }
    }

    pub(crate) fn step(gb: &mut GameBoy) -> Result<ClockCycles, Error> {
        let mut mcycles = MachineCycles::One;

        CPU::handle_interrupts(gb);
        
        if !gb.cpu.is_halted {
            let instruction = CPU::fetch_decode(gb)?;
            //println!("{} {:?}", gb, instruction);
            mcycles = instruction.execute(gb)?;           
        }

        Timers::tick(gb, u8::from(mcycles.clone()));        

        Ok(ClockCycles::from(mcycles))
    }   

    pub(super) fn fetch_decode(gb: &GameBoy) -> Result<Instruction, Error> {
        let instruction_byte = MMU::read_byte(gb, gb.cpu.pc);
        let byte0 = MMU::read_byte(gb, gb.cpu.pc+1);
        let byte1 = MMU::read_byte(gb, gb.cpu.pc+2);

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

    pub(crate) fn send_serial(gb: &mut GameBoy) -> Option<u8> {
        let serial_transfer = SerialTransferMode::parse_from_byte(MMU::read_byte(&gb, SERIAL_CONTROL_ADDRESS));
        let serial_data = MMU::read_byte(&gb, SERIAL_DATA_ADDRESS);
        
        match serial_transfer {
            SerialTransferMode::TransferInternalClock => Some(serial_data),
            SerialTransferMode::TransferExternalClock => Some(serial_data),
            _ => None
        }
    }

    pub(crate) fn handle_interrupts(gb: &mut GameBoy) {
        if Interrupts::some_interrupt_enabled(gb) {
            if gb.cpu.ime {
                let interrupt = Interrupts::interrupt_to_handle(gb).unwrap();
                gb.cpu.is_halted = false;
                gb.cpu.ime = false;
                CPU::push_stack(gb, gb.cpu.pc);
                gb.cpu.pc = interrupt.handler();
            }else{
                gb.cpu.is_halted = false;
            }
        }
    }

    pub(crate) fn push_stack(gb: &mut GameBoy, value: u16) {
        gb.cpu.sp = gb.cpu.sp.wrapping_sub(1);
        MMU::write_byte(gb, gb.cpu.sp, ((value & 0xFF00) >> 8) as u8);
        gb.cpu.sp = gb.cpu.sp.wrapping_sub(1);
        MMU::write_byte(gb, gb.cpu.sp, (value & 0xFF) as u8);
    }
    
    pub(crate) fn pop_stack(gb: &mut GameBoy) -> u16 {
        let lsb = MMU::read_byte(&gb, gb.cpu.sp) as u16;
        gb.cpu.sp = gb.cpu.sp.wrapping_add(1);
    
        let msb = MMU::read_byte(&gb, gb.cpu.sp) as u16;
        gb.cpu.sp = gb.cpu.sp.wrapping_add(1);
    
        (msb << 8) | lsb
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