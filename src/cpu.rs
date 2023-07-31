mod registers;
mod instructions;
mod tests;

use crate::rom::ROM;
use instructions::*;

type ProgramCounter = u16;
type StackPointer = u16;
type Address = u16;
pub struct CPU {
    regs: Registers,
    sp: StackPointer,
    pc: ProgramCounter,
    bus: MemoryBus
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
struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool
}

struct MemoryBus {
    memory: [u8; 0xFFFF]
}

impl MemoryBus {
    pub fn new() -> MemoryBus {
        let data = [0; 0xFFFF];

        MemoryBus { memory: data }
    }

    fn read_byte(&self, address: Address) -> u8 {
        self.memory[address as usize]
    }

    fn write_byte(&mut self, address: Address, byte: u8) {
        self.memory[address as usize] = byte;
    }
}

impl CPU {
    pub fn new(boot: ROM) -> CPU {
        let mut membus = MemoryBus::new();

        // Loading the boot ROM data into memory
        for addr in 0..boot.size() {
            membus.write_byte(addr, boot.read_byte(addr))
        } 

        CPU { regs: Registers::new(), sp: 0b0, pc: 0b0, bus: membus }
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
            Instruction::ADD(target) => self.add(target),
            Instruction::ADC(target) => self.adc(target),
            Instruction::INC(target) => self.inc(target),
            Instruction::DEC(target) => self.dec(target),
            Instruction::SUB(target) => self.sub(target),
            Instruction::SBC(target) => self.sbc(target),
            Instruction::AND(target) => self.and(target),
            Instruction::XOR(target) => self.xor(target),
            Instruction::OR(target) => self.or(target),
            Instruction::CP(target) => self.cp(target),
            Instruction::LD(load_type) => self.load(load_type),
            Instruction::JP(test) => self.jump(test),
            _ => { /* TODO: support more instructions */ self.pc }
        }

        

    }

    fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc+1)
    }

    fn load(&mut self, load_type: LoadType) -> ProgramCounter {
        match load_type {
          LoadType::Byte(target, source) => {
            let source_value = match source {
              LoadByteSource::A => self.regs.a,
              LoadByteSource::D8 => self.read_next_byte(),
              LoadByteSource::HLI => self.bus.read_byte(self.regs.get_hl()),
              _ => { panic!("TODO: implement other sources") }
            };
            match target {
              LoadByteTarget::A => self.regs.a = source_value,
              LoadByteTarget::HLI => self.bus.write_byte(self.regs.get_hl(), source_value),
              _ => { panic!("TODO: implement other targets") }
            };
            match source {
              LoadByteSource::D8  => self.pc.wrapping_add(2),
              _                   => self.pc.wrapping_add(1),
            }
          }
          _ => { panic!("TODO: implement other load types") }
        }
    }

    fn jump(&self, test: JumpTest) -> ProgramCounter {
        let should_jump = match test {
            JumpTest::NotZero => !self.regs.flags.zero,
            JumpTest::NotCarry => !self.regs.flags.carry,
            JumpTest::Zero => self.regs.flags.zero,
            JumpTest::Carry => self.regs.flags.carry,
            JumpTest::Always => true
        };
     
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

    fn nop(&self) -> ProgramCounter {
        self.pc.wrapping_add(1)
    }

    fn add(&mut self, target: ArithmeticTarget) -> ProgramCounter {
        let value = self.get_arithmetic_target_val(target);

        let (new_value, did_overflow) = self.regs.a.overflowing_add(value);
        self.regs.flags.zero = new_value == 0;
        self.regs.flags.subtract = false;
        self.regs.flags.carry = did_overflow;
        // Half Carry is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble.
        self.regs.flags.half_carry = (self.regs.a & 0xF) + (value & 0xF) > 0xF;
        self.regs.a = new_value;
        self.pc.wrapping_add(1)
    }

    fn adc(&mut self, target: ArithmeticTarget) -> ProgramCounter {
        let value = self.get_arithmetic_target_val(target);

        let (new_value1, did_overflow1) = self.regs.a.overflowing_add(value);

        let (new_value2, did_overflow2) = new_value1.overflowing_add(self.regs.flags.carry as u8);

        self.regs.flags.zero = new_value2 == 0;
        self.regs.flags.subtract = false;
        self.regs.flags.half_carry = ((self.regs.a & 0xF) + (value & 0xF) > 0xF) || ((new_value1 & 0xF) + (self.regs.flags.carry as u8) > 0xF);
        self.regs.flags.carry = did_overflow1 || did_overflow2;      
        self.regs.a = new_value2;
        self.pc.wrapping_add(1)
    }

    fn sub(&mut self, target: ArithmeticTarget) -> ProgramCounter {
        let value = self.get_arithmetic_target_val(target);

        let (new_value, did_overflow) = self.regs.a.overflowing_sub(value);
        self.regs.flags.zero = new_value == 0;
        self.regs.flags.subtract = true;
        self.regs.flags.carry = did_overflow;
        let (new_value_low, _) = (self.regs.a & 0xF).overflowing_sub(value & 0xF);
        self.regs.flags.half_carry = (new_value_low & 0x10) == 0x10;
        self.regs.a = new_value;
        self.pc.wrapping_add(1)
    }

    fn sbc(&mut self, target: ArithmeticTarget) -> ProgramCounter {
        let value = self.get_arithmetic_target_val(target);

        let (new_value1, did_overflow1) = self.regs.a.overflowing_sub(self.regs.flags.carry as u8);
        let (new_value2, did_overflow2) = new_value1.overflowing_sub(value);

        self.regs.flags.zero = new_value2 == 0;
        self.regs.flags.subtract = true;
        self.regs.flags.carry = did_overflow1 || did_overflow2;
        let (new_value_low, _) = (new_value2 & 0xF).overflowing_sub(value & 0xF);
        self.regs.flags.half_carry = (new_value_low & 0x10) == 0x10;
        self.regs.a = new_value2;
        self.pc.wrapping_add(1)
    }

    fn and(&mut self, target: ArithmeticTarget) -> ProgramCounter {
        let value = self.get_arithmetic_target_val(target);

        self.regs.a = self.regs.a & value;
        self.regs.flags.zero = self.regs.a == 0;
        self.regs.flags.subtract = false;
        self.regs.flags.half_carry = true;
        self.regs.flags.carry = false;

        self.pc.wrapping_add(1)
    }

    fn xor(&mut self, target: ArithmeticTarget) -> ProgramCounter {
        let value = self.get_arithmetic_target_val(target);

        self.regs.a = self.regs.a ^ value;
        self.regs.flags.zero = self.regs.a == 0;
        self.regs.flags.subtract = false;
        self.regs.flags.half_carry = false;
        self.regs.flags.carry = false;

        self.pc.wrapping_add(1)
    }

    fn or(&mut self, target: ArithmeticTarget) -> ProgramCounter {
        let value = self.get_arithmetic_target_val(target);

        self.regs.a = self.regs.a | value;
        self.regs.flags.zero = self.regs.a == 0;
        self.regs.flags.subtract = false;
        self.regs.flags.half_carry = false;
        self.regs.flags.carry = false;

        self.pc.wrapping_add(1)
    }

    fn cp(&mut self, target: ArithmeticTarget) -> ProgramCounter {
        let value = self.get_arithmetic_target_val(target);

        let (result, did_overflow) = self.regs.a.overflowing_sub(value);
        self.regs.flags.zero = result == 0;
        self.regs.flags.subtract = true;
        let (new_value_low, _) = (self.regs.a & 0xF).overflowing_sub(value & 0xF);
        self.regs.flags.half_carry = (new_value_low & 0x10) == 0x10;
        self.regs.flags.carry = did_overflow;

        self.pc.wrapping_add(1)
    }

    fn get_arithmetic_target_val(&self, target: ArithmeticTarget) -> u8 {
        match target {
            ArithmeticTarget::A => self.regs.a,
            ArithmeticTarget::B => self.regs.b,
            ArithmeticTarget::C => self.regs.c,
            ArithmeticTarget::D => self.regs.d,
            ArithmeticTarget::E => self.regs.e,
            ArithmeticTarget::H => self.regs.h,
            ArithmeticTarget::L => self.regs.l,
            ArithmeticTarget::HLI => self.bus.read_byte(self.regs.get_hl()),
        }
    }

    fn inc(&mut self, target: IncDecTarget) -> ProgramCounter {
        let value = match target {
            IncDecTarget::BC => self.regs.set_bc(self.regs.get_bc().wrapping_add(1)),
            IncDecTarget::DE => self.regs.set_de(self.regs.get_de().wrapping_add(1)),
            IncDecTarget::HL => self.regs.set_hl(self.regs.get_hl().wrapping_add(1)),
            IncDecTarget::SP => self.sp = self.sp.wrapping_add(1),
        };
        self.pc.wrapping_add(1)
    }

    fn dec(&mut self, target: IncDecTarget) -> ProgramCounter {
        let value = match target {
            IncDecTarget::BC => self.regs.set_bc(self.regs.get_bc().wrapping_sub(1)),
            IncDecTarget::DE => self.regs.set_de(self.regs.get_de().wrapping_sub(1)),
            IncDecTarget::HL => self.regs.set_hl(self.regs.get_hl().wrapping_sub(1)),
            IncDecTarget::SP => self.sp = self.sp.wrapping_sub(1),
        };
        self.pc.wrapping_add(1)
    }

}