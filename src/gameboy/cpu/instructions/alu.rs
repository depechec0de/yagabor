use std::io::{Error, ErrorKind};

use crate::gameboy::cpu::cpu::{CPU, ProgramCounter, MachineCycles};

use super::{instructions::{Instruction}, decode::{RegistersIndDir, WordRegister, RegistersIndirect, BitType, RotateDirection, BitTarget, ResSetType}};

impl Instruction {

    pub(super) fn add(&self, cpu: &mut CPU, target: RegistersIndDir) -> Result<MachineCycles, Error> {
        let value = get_arithmetic_target_val(cpu, &target);

        let (new_value, did_overflow) = cpu.regs.a.overflowing_add(value);
        cpu.regs.flags.zero = new_value == 0;
        cpu.regs.flags.subtract = false;
        cpu.regs.flags.carry = did_overflow;
        // Half Carry is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble.
        cpu.regs.flags.half_carry = (cpu.regs.a & 0xF).wrapping_add(value & 0xF) > 0xF;
        cpu.regs.a = new_value;
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
        match target {
            RegistersIndDir::HLI => Ok(MachineCycles::Two),
            RegistersIndDir::D8 => Ok(MachineCycles::Two),
            _ => Ok(MachineCycles::One),
        }
    }

    pub(super) fn addsps8(&self, cpu: &mut CPU) -> Result<MachineCycles, Error> {
        // To preserve the sign
        let value = cpu.read_next_byte(cpu.pc) as i8 as i16 as u16;

        let new_value = cpu.sp.wrapping_add(value);
        cpu.regs.flags.zero = false;
        cpu.regs.flags.subtract = false;
        cpu.regs.flags.carry = (cpu.sp & 0xFF).wrapping_add(value & 0xFF) > 0xFF;
        cpu.regs.flags.half_carry = (cpu.sp & 0xF).wrapping_add(value & 0xF) > 0xF; 
        cpu.sp = new_value;
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
        Ok(MachineCycles::Four)
    }

    pub(super) fn add16(&self, cpu: &mut CPU, target: WordRegister) -> Result<MachineCycles, Error> {
        let value = match target {
            WordRegister::BC => cpu.regs.get_bc(),
            WordRegister::DE => cpu.regs.get_de(),
            WordRegister::HL => cpu.regs.get_hl(),
            WordRegister::SP => cpu.sp,
        };

        let (new_value, did_overflow) = cpu.regs.get_hl().overflowing_add(value);
        cpu.regs.flags.subtract = false;
        cpu.regs.flags.carry = did_overflow;
        // This works for 16 bit
        cpu.regs.flags.half_carry = (cpu.regs.get_hl() & 0xfff).wrapping_add(value & 0xfff) > 0xfff; 
        cpu.regs.set_hl(new_value);
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
        Ok(MachineCycles::Two)
    }

    pub(super) fn adc(&self, cpu: &mut CPU, target: RegistersIndDir) -> Result<MachineCycles, Error> {
        let value = get_arithmetic_target_val(cpu, &target);

        let (new_value1, did_overflow1) = cpu.regs.a.overflowing_add(value);
        let (new_value2, did_overflow2) = new_value1.overflowing_add(cpu.regs.flags.carry as u8);

        cpu.regs.flags.zero = new_value2 == 0;
        cpu.regs.flags.subtract = false;
        cpu.regs.flags.half_carry = ((cpu.regs.a & 0xF) + (value & 0xF) > 0xF) || ((new_value1 & 0xF) + (cpu.regs.flags.carry as u8) > 0xF);
        cpu.regs.flags.carry = did_overflow1 || did_overflow2;      
        cpu.regs.a = new_value2;
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));

        match target {
            RegistersIndDir::HLI => Ok(MachineCycles::Two),
            RegistersIndDir::D8 => Ok(MachineCycles::Two),
            _ => Ok(MachineCycles::One),
        }
    }

    pub(super) fn sub(&self, cpu: &mut CPU, target: RegistersIndDir) -> Result<MachineCycles, Error> {
        let value = get_arithmetic_target_val(cpu, &target);

        let (new_value, did_overflow) = cpu.regs.a.overflowing_sub(value);
        cpu.regs.flags.zero = new_value == 0;
        cpu.regs.flags.subtract = true;
        cpu.regs.flags.carry = did_overflow;
        let (new_value_low, _) = (cpu.regs.a & 0xF).overflowing_sub(value & 0xF);
        cpu.regs.flags.half_carry = (new_value_low & 0x10) == 0x10;
        cpu.regs.a = new_value;
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));

        match target {
            RegistersIndDir::HLI => Ok(MachineCycles::Two),
            RegistersIndDir::D8 => Ok(MachineCycles::Two),
            _ => Ok(MachineCycles::One),
        }
    }

    pub(super) fn sbc(&self, cpu: &mut CPU, target: RegistersIndDir) -> Result<MachineCycles, Error> {

        let value = get_arithmetic_target_val(cpu, &target);

        let (new_value1, did_overflow1) = cpu.regs.a.overflowing_sub(cpu.regs.flags.carry as u8);
        let (new_value2, did_overflow2) = new_value1.overflowing_sub(value);

        let half_carry_step1 = ((cpu.regs.a & 0xF).wrapping_sub(cpu.regs.flags.carry as u8 & 0xF) & 0x10) == 0x10;
        let half_carry_step2 = ((new_value1 & 0xF).wrapping_sub(value & 0xF) & 0x10) == 0x10;

        cpu.regs.flags.half_carry = half_carry_step1 || half_carry_step2;
        cpu.regs.flags.zero = new_value2 == 0;
        cpu.regs.flags.subtract = true;
        cpu.regs.flags.carry = did_overflow1 || did_overflow2;
        
        cpu.regs.a = new_value2;
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));

        match target {
            RegistersIndDir::HLI => Ok(MachineCycles::Two),
            RegistersIndDir::D8 => Ok(MachineCycles::Two),
            _ => Ok(MachineCycles::One),
        }
    }

    pub(super) fn and(&self, cpu: &mut CPU, target: RegistersIndDir) -> Result<MachineCycles, Error> {
        let value = get_arithmetic_target_val(cpu, &target);

        cpu.regs.a = cpu.regs.a & value;
        cpu.regs.flags.zero = cpu.regs.a == 0;
        cpu.regs.flags.subtract = false;
        cpu.regs.flags.half_carry = true;
        cpu.regs.flags.carry = false;
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));

        match target {
            RegistersIndDir::HLI => Ok(MachineCycles::Two),
            RegistersIndDir::D8 => Ok(MachineCycles::Two),
            _ => Ok(MachineCycles::One),
        }
    }

    pub(super) fn xor(&self, cpu: &mut CPU, target: RegistersIndDir) -> Result<MachineCycles, Error> {
        let value = get_arithmetic_target_val(cpu, &target);

        cpu.regs.a = cpu.regs.a ^ value;
        cpu.regs.flags.zero = cpu.regs.a == 0;
        cpu.regs.flags.subtract = false;
        cpu.regs.flags.half_carry = false;
        cpu.regs.flags.carry = false;
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));

        match target {
            RegistersIndDir::HLI => Ok(MachineCycles::Two),
            RegistersIndDir::D8 => Ok(MachineCycles::Two),
            _ => Ok(MachineCycles::One),
        }
    }

    pub(super) fn or(&self, cpu: &mut CPU, target: RegistersIndDir) -> Result<MachineCycles, Error> {
        let value = get_arithmetic_target_val(cpu, &target);

        cpu.regs.a = cpu.regs.a | value;
        cpu.regs.flags.zero = cpu.regs.a == 0;
        cpu.regs.flags.subtract = false;
        cpu.regs.flags.half_carry = false;
        cpu.regs.flags.carry = false;
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));

        match target {
            RegistersIndDir::HLI => Ok(MachineCycles::Two),
            RegistersIndDir::D8 => Ok(MachineCycles::Two),
            _ => Ok(MachineCycles::One),
        }
    }

    pub(super) fn cp(&self, cpu: &mut CPU, target: RegistersIndDir) -> Result<MachineCycles, Error> {
        let value = get_arithmetic_target_val(cpu, &target);

        let (result, did_overflow) = cpu.regs.a.overflowing_sub(value);
        cpu.regs.flags.zero = result == 0;
        cpu.regs.flags.subtract = true;
        let (new_value_low, _) = (cpu.regs.a & 0xF).overflowing_sub(value & 0xF);
        cpu.regs.flags.half_carry = (new_value_low & 0x10) == 0x10;
        cpu.regs.flags.carry = did_overflow;
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));

        match target {
            RegistersIndDir::HLI => Ok(MachineCycles::Two),
            RegistersIndDir::D8 => Ok(MachineCycles::Two),
            _ => Ok(MachineCycles::One),
        }
    }

    pub(super) fn inc(&self, cpu: &mut CPU, target: RegistersIndirect) -> Result<MachineCycles, Error> {
        cpu.regs.flags.subtract = false;

        match target {
            RegistersIndirect::A => { 
                cpu.regs.flags.half_carry = (cpu.regs.a & 0xF).wrapping_add(0b1 & 0xF) > 0xF;
                cpu.regs.flags.zero = cpu.regs.a.wrapping_add(1) == 0;
                cpu.regs.a = cpu.regs.a.wrapping_add(1);
            },
            RegistersIndirect::B => { 
                cpu.regs.flags.half_carry = (cpu.regs.b & 0xF).wrapping_add(0b1 & 0xF) > 0xF;
                cpu.regs.flags.zero = cpu.regs.b.wrapping_add(1) == 0;
                cpu.regs.b = cpu.regs.b.wrapping_add(1);
            },
            RegistersIndirect::C => { 
                cpu.regs.flags.half_carry = (cpu.regs.c & 0xF).wrapping_add(0b1 & 0xF) > 0xF;
                cpu.regs.flags.zero = cpu.regs.c.wrapping_add(1) == 0;
                cpu.regs.c = cpu.regs.c.wrapping_add(1);
            },
            RegistersIndirect::D => { 
                cpu.regs.flags.half_carry = (cpu.regs.d & 0xF).wrapping_add(0b1 & 0xF) > 0xF;
                cpu.regs.flags.zero = cpu.regs.d.wrapping_add(1) == 0;
                cpu.regs.d = cpu.regs.d.wrapping_add(1);
            },
            RegistersIndirect::E => { 
                cpu.regs.flags.half_carry = (cpu.regs.e & 0xF).wrapping_add(0b1 & 0xF) > 0xF;
                cpu.regs.flags.zero = cpu.regs.e.wrapping_add(1) == 0;
                cpu.regs.e = cpu.regs.e.wrapping_add(1);
            },
            RegistersIndirect::H => { 
                cpu.regs.flags.half_carry = (cpu.regs.h & 0xF).wrapping_add(0b1 & 0xF) > 0xF;
                cpu.regs.flags.zero = cpu.regs.h.wrapping_add(1) == 0;
                cpu.regs.h = cpu.regs.h.wrapping_add(1);
            },
            RegistersIndirect::L => { 
                cpu.regs.flags.half_carry = (cpu.regs.l & 0xF).wrapping_add(0b1 & 0xF) > 0xF;
                cpu.regs.flags.zero = cpu.regs.l.wrapping_add(1) == 0;
                cpu.regs.l = cpu.regs.l.wrapping_add(1);
            },
            RegistersIndirect::HLI => {
                let old_val = cpu.mmu.read_byte(cpu.regs.get_hl());
                cpu.regs.flags.half_carry = (old_val & 0xF).wrapping_add(0b1 & 0xF) > 0xF;
                let new_val = old_val.wrapping_add(1);
                cpu.regs.flags.zero = new_val == 0;
                cpu.mmu.write_byte(cpu.regs.get_hl(), new_val);
            }
        };
  
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));

        match target {
            RegistersIndirect::HLI => Ok(MachineCycles::Three),
            _ => Ok(MachineCycles::One),
        }
    }

    pub(super) fn dec(&self, cpu: &mut CPU, target: RegistersIndirect) -> Result<MachineCycles, Error> {
        cpu.regs.flags.subtract = true;

        match target {
            RegistersIndirect::A => { 
                cpu.regs.flags.half_carry = (cpu.regs.a & 0xF).wrapping_sub(0b1 & 0xF) > 0xF;
                cpu.regs.flags.zero = cpu.regs.a.wrapping_sub(1) == 0;
                cpu.regs.a = cpu.regs.a.wrapping_sub(1);
            },
            RegistersIndirect::B => { 
                cpu.regs.flags.half_carry = (cpu.regs.b & 0xF).wrapping_sub(0b1 & 0xF) > 0xF;
                cpu.regs.flags.zero = cpu.regs.b.wrapping_sub(1) == 0;
                cpu.regs.b = cpu.regs.b.wrapping_sub(1);
            },
            RegistersIndirect::C => { 
                cpu.regs.flags.half_carry = (cpu.regs.c & 0xF).wrapping_sub(0b1 & 0xF) > 0xF;
                cpu.regs.flags.zero = cpu.regs.c.wrapping_sub(1) == 0;
                cpu.regs.c = cpu.regs.c.wrapping_sub(1);
            },
            RegistersIndirect::D => { 
                cpu.regs.flags.half_carry = (cpu.regs.d & 0xF).wrapping_sub(0b1 & 0xF) > 0xF;
                cpu.regs.flags.zero = cpu.regs.d.wrapping_sub(1) == 0;
                cpu.regs.d = cpu.regs.d.wrapping_sub(1);
            },
            RegistersIndirect::E => { 
                cpu.regs.flags.half_carry = (cpu.regs.e & 0xF).wrapping_sub(0b1 & 0xF) > 0xF;
                cpu.regs.flags.zero = cpu.regs.e.wrapping_sub(1) == 0;
                cpu.regs.e = cpu.regs.e.wrapping_sub(1);
            },
            RegistersIndirect::H => { 
                cpu.regs.flags.half_carry = (cpu.regs.h & 0xF).wrapping_sub(0b1 & 0xF) > 0xF;
                cpu.regs.flags.zero = cpu.regs.h.wrapping_sub(1) == 0;
                cpu.regs.h = cpu.regs.h.wrapping_sub(1);
            },
            RegistersIndirect::L => { 
                cpu.regs.flags.half_carry = (cpu.regs.l & 0xF).wrapping_sub(0b1 & 0xF) > 0xF;
                cpu.regs.flags.zero = cpu.regs.l.wrapping_sub(1) == 0;
                cpu.regs.l = cpu.regs.l.wrapping_sub(1);
            },
            RegistersIndirect::HLI => {
                let old_val = cpu.mmu.read_byte(cpu.regs.get_hl());
                cpu.regs.flags.half_carry = (old_val & 0xF).wrapping_sub(0b1 & 0xF) > 0xF;
                let new_val = old_val.wrapping_sub(1);
                cpu.regs.flags.zero = new_val == 0;
                cpu.mmu.write_byte(cpu.regs.get_hl(), new_val);
            }
        };
  
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));

        match target {
            RegistersIndirect::HLI => Ok(MachineCycles::Three),
            _ => Ok(MachineCycles::One),
        }
    }

    pub(super) fn inc16(&self, cpu: &mut CPU, target: WordRegister) -> Result<MachineCycles, Error> {
        match target {
            WordRegister::BC => cpu.regs.set_bc(cpu.regs.get_bc().wrapping_add(1)),
            WordRegister::DE => cpu.regs.set_de(cpu.regs.get_de().wrapping_add(1)),
            WordRegister::HL => cpu.regs.set_hl(cpu.regs.get_hl().wrapping_add(1)),
            WordRegister::SP => cpu.sp = cpu.sp.wrapping_add(1),
        };
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
        Ok(MachineCycles::Two)
    }

    pub(super) fn dec16(&self, cpu: &mut CPU, target: WordRegister) -> Result<MachineCycles, Error> {
        match target {
            WordRegister::BC => cpu.regs.set_bc(cpu.regs.get_bc().wrapping_sub(1)),
            WordRegister::DE => cpu.regs.set_de(cpu.regs.get_de().wrapping_sub(1)),
            WordRegister::HL => cpu.regs.set_hl(cpu.regs.get_hl().wrapping_sub(1)),
            WordRegister::SP => cpu.sp = cpu.sp.wrapping_sub(1),
        };
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
        Ok(MachineCycles::Two)
    }

    pub(super) fn bit(&self, cpu: &mut CPU, bit_type: BitType) -> Result<MachineCycles, Error> {
        let BitType::Registers(t, s) = bit_type;
        let target = t;
        let source = s;

        let i = get_position_by_bittarget(target);
        let value = get_register_indirect_val(cpu, source.clone());
        let bit_value = get_bit_val(i, value);

        cpu.regs.flags.zero = !bit_value;
        cpu.regs.flags.subtract = false;
        cpu.regs.flags.half_carry = true;
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));

        match source {
            RegistersIndirect::HLI => Ok(MachineCycles::Three),
            _ => Ok(MachineCycles::Two),
        }             
    }

    // RLA, RRA, ... are legacy instructions made for compatibility with 8080
    // No zero flag is set
    pub(super) fn rla(&self, cpu: &mut CPU) -> Result<MachineCycles, Error> {
        bitwise_rotate(cpu, &RegistersIndirect::A, RotateDirection::Left, false);
        cpu.regs.flags.zero = false;
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
        Ok(MachineCycles::One)
    }

    pub(super) fn rlca(&self, cpu: &mut CPU) -> Result<MachineCycles, Error> {
        bitwise_rotate(cpu, &RegistersIndirect::A, RotateDirection::Left, true);
        cpu.regs.flags.zero = false;
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
        Ok(MachineCycles::One)
    }

    pub(super) fn rra(&self, cpu: &mut CPU) -> Result<MachineCycles, Error> {
        bitwise_rotate(cpu, &RegistersIndirect::A, RotateDirection::Right, false);
        cpu.regs.flags.zero = false;
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
        Ok(MachineCycles::One)
    }

    pub(super) fn rrca(&self, cpu: &mut CPU) -> Result<MachineCycles, Error> {
        bitwise_rotate(cpu, &RegistersIndirect::A, RotateDirection::Right, true);
        cpu.regs.flags.zero = false;
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
        Ok(MachineCycles::One)
    }

    pub(super) fn sla(&self, cpu: &mut CPU, target: RegistersIndirect) -> Result<MachineCycles, Error> {
        bitwise_rotate(cpu, &target, RotateDirection::Left, true);
        res_set(cpu, ResSetType::Registers(BitTarget::Zero, target.clone()), false);
        set_flag_zero(cpu, &target);
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
        
        match target {
            RegistersIndirect::HLI => Ok(MachineCycles::Four),
            _ => Ok(MachineCycles::Two),
        }
    }

    pub(super) fn sra(&self, cpu: &mut CPU, target: RegistersIndirect) -> Result<MachineCycles, Error> {
        let value = get_register_indirect_val(cpu, target.clone());
        let bit7 = get_bit_val(7, value);

        bitwise_rotate(cpu, &target, RotateDirection::Right, true);
        res_set(cpu, ResSetType::Registers(BitTarget::Seven, target.clone()), bit7);
        set_flag_zero(cpu, &target);
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));

        match target {
            RegistersIndirect::HLI => Ok(MachineCycles::Four),
            _ => Ok(MachineCycles::Two),
        }
    }

    pub(super) fn srl(&self, cpu: &mut CPU, target: RegistersIndirect) -> Result<MachineCycles, Error> {
        bitwise_rotate(cpu, &target, RotateDirection::Right, true);
        res_set(cpu, ResSetType::Registers(BitTarget::Seven, target.clone()), false);
        set_flag_zero(cpu, &target);
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
        
        match target {
            RegistersIndirect::HLI => Ok(MachineCycles::Four),
            _ => Ok(MachineCycles::Two),
        }
    }

    pub(super) fn rr(&self, cpu: &mut CPU, target: RegistersIndirect) -> Result<MachineCycles, Error> {
        bitwise_rotate(cpu, &target, RotateDirection::Right, false);
        set_flag_zero(cpu, &target);
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
        
        match target {
            RegistersIndirect::HLI => Ok(MachineCycles::Four),
            _ => Ok(MachineCycles::Two),
        }
    }

    pub(super) fn rrc(&self, cpu: &mut CPU, target: RegistersIndirect) -> Result<MachineCycles, Error> {
        bitwise_rotate(cpu, &target, RotateDirection::Right, true);
        set_flag_zero(cpu, &target);
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));

        match target {
            RegistersIndirect::HLI => Ok(MachineCycles::Four),
            _ => Ok(MachineCycles::Two),
        }
    }

    pub(super) fn rl(&self, cpu: &mut CPU, target: RegistersIndirect) -> Result<MachineCycles, Error> {
        bitwise_rotate(cpu, &target, RotateDirection::Left, false);
        set_flag_zero(cpu, &target);
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));

        match target {
            RegistersIndirect::HLI => Ok(MachineCycles::Four),
            _ => Ok(MachineCycles::Two),
        }
    }

    pub(super) fn rlc(&self, cpu: &mut CPU, target: RegistersIndirect) -> Result<MachineCycles, Error> {
        bitwise_rotate(cpu, &target, RotateDirection::Left, true);
        set_flag_zero(cpu, &target);
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));

        match target {
            RegistersIndirect::HLI => Ok(MachineCycles::Four),
            _ => Ok(MachineCycles::Two),
        }
    }

    pub(super) fn swap(&self, cpu: &mut CPU, target: RegistersIndirect) -> Result<MachineCycles, Error> {
        let value = get_register_indirect_val(cpu, target.clone());

        let low = value & 0x0F;
        let high = value & 0xF0;

        let new_value = (low << 4).wrapping_add(high >> 4);

        match target {
            RegistersIndirect::A => { cpu.regs.a = new_value; },
            RegistersIndirect::B => { cpu.regs.b = new_value; },
            RegistersIndirect::C => { cpu.regs.c = new_value; },
            RegistersIndirect::D => { cpu.regs.d = new_value; },
            RegistersIndirect::E => { cpu.regs.e = new_value; },
            RegistersIndirect::H => { cpu.regs.h = new_value; },
            RegistersIndirect::L => { cpu.regs.l = new_value; },
            RegistersIndirect::HLI => {
                cpu.mmu.write_byte(cpu.regs.get_hl(), new_value);
            }
        };

        cpu.regs.flags.subtract = false;
        cpu.regs.flags.half_carry = false;
        cpu.regs.flags.carry = false;
        set_flag_zero(cpu, &target);
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
        Ok(MachineCycles::Two)
    }

    pub(super) fn res(&self, cpu: &mut CPU, target: ResSetType) -> Result<MachineCycles, Error> {

        let ResSetType::Registers(bt, register) = target.clone();

        res_set(cpu, target, false);
        
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
        
        match register {
            RegistersIndirect::HLI => Ok(MachineCycles::Four),
            _ => Ok(MachineCycles::Two),
        }
    }

    pub(super) fn set(&self, cpu: &mut CPU, target: ResSetType) -> Result<MachineCycles, Error> {

        let ResSetType::Registers(bt, register) = target.clone();

        res_set(cpu, target, true);
        
        cpu.pc = cpu.pc.wrapping_add(u16::from(self.op.size()));
        
        match register {
            RegistersIndirect::HLI => Ok(MachineCycles::Four),
            _ => Ok(MachineCycles::Two),
        }
    }
}

// RR r and RL r instructions
// If is_rc is true we consider the RLC and RRC instructions, otherwise the RL and RR
fn bitwise_rotate(cpu: &mut CPU, target: &RegistersIndirect, direction: RotateDirection, is_rc: bool) {
    cpu.regs.flags.subtract = false;
    cpu.regs.flags.half_carry = false;

    match direction {
        RotateDirection::Left => shift_left_register(cpu, &target, is_rc),
        RotateDirection::Right => shift_right_register(cpu, &target, is_rc),
    }
}

pub(super) fn res_set(cpu: &mut CPU, target: ResSetType, value: bool) {
    let ResSetType::Registers(bt, register) = target;
    
    let i = get_position_by_bittarget(bt);

    match register {
        RegistersIndirect::A => cpu.regs.a = set_bit_val(i, value, cpu.regs.a),
        RegistersIndirect::B => cpu.regs.b = set_bit_val(i, value, cpu.regs.b),
        RegistersIndirect::C => cpu.regs.c = set_bit_val(i, value, cpu.regs.c),
        RegistersIndirect::D => cpu.regs.d = set_bit_val(i, value, cpu.regs.d),
        RegistersIndirect::E => cpu.regs.e = set_bit_val(i, value, cpu.regs.e),
        RegistersIndirect::H   => cpu.regs.h = set_bit_val(i, value, cpu.regs.h),
        RegistersIndirect::L   => cpu.regs.l = set_bit_val(i, value, cpu.regs.l),
        RegistersIndirect::HLI => {
            let new_value = set_bit_val(i, value, cpu.mmu.read_byte(cpu.regs.get_hl()));
            cpu.mmu.write_byte(cpu.regs.get_hl(), new_value);
        }
    };
}

fn get_register_indirect_val(cpu: &CPU, source: RegistersIndirect) -> u8 {
    match source {
        RegistersIndirect::A => cpu.regs.a,
        RegistersIndirect::B => cpu.regs.b,
        RegistersIndirect::C => cpu.regs.c,
        RegistersIndirect::D => cpu.regs.d,
        RegistersIndirect::E => cpu.regs.e,
        RegistersIndirect::H => cpu.regs.h,
        RegistersIndirect::L => cpu.regs.l,
        RegistersIndirect::HLI => cpu.mmu.read_byte(cpu.regs.get_hl()),
    }
}

fn get_arithmetic_target_val(cpu: &CPU, target: &RegistersIndDir) -> u8 {
    match target {
        RegistersIndDir::A     => cpu.regs.a,
        RegistersIndDir::B     => cpu.regs.b,
        RegistersIndDir::C     => cpu.regs.c,
        RegistersIndDir::D     => cpu.regs.d,
        RegistersIndDir::E     => cpu.regs.e,
        RegistersIndDir::H     => cpu.regs.h,
        RegistersIndDir::L     => cpu.regs.l,
        RegistersIndDir::HLI   => cpu.mmu.read_byte(cpu.regs.get_hl()),
        RegistersIndDir::D8    => cpu.read_next_byte(cpu.pc)
    }
}

fn shift_left_register(cpu: &mut CPU, target: &RegistersIndirect, is_rlc: bool) {
    let new_bit0;
    let prev_bit7;

    match target {
        RegistersIndirect::A => prev_bit7 = get_bit_val(7,cpu.regs.a),
        RegistersIndirect::B => prev_bit7 = get_bit_val(7,cpu.regs.b),
        RegistersIndirect::C => prev_bit7 = get_bit_val(7,cpu.regs.c),
        RegistersIndirect::D => prev_bit7 = get_bit_val(7,cpu.regs.d),
        RegistersIndirect::E => prev_bit7 = get_bit_val(7,cpu.regs.e),
        RegistersIndirect::H   => prev_bit7 = get_bit_val(7,cpu.regs.h),
        RegistersIndirect::L   => prev_bit7 = get_bit_val(7,cpu.regs.l),
        RegistersIndirect::HLI => { 
            let hl_value = cpu.mmu.read_byte(cpu.regs.get_hl());
            prev_bit7 = get_bit_val(7,hl_value);
        }
    };

    if is_rlc {
        new_bit0 = prev_bit7;
        cpu.regs.flags.carry = prev_bit7;
    }else{
        new_bit0 = cpu.regs.flags.carry;
        cpu.regs.flags.carry = prev_bit7;
    }

    match target {
        RegistersIndirect::A => { cpu.regs.a = (cpu.regs.a << 1).wrapping_add(new_bit0 as u8); },
        RegistersIndirect::B => { cpu.regs.b = (cpu.regs.b << 1).wrapping_add(new_bit0 as u8); },
        RegistersIndirect::C => { cpu.regs.c = (cpu.regs.c << 1).wrapping_add(new_bit0 as u8); },
        RegistersIndirect::D => { cpu.regs.d = (cpu.regs.d << 1).wrapping_add(new_bit0 as u8); },
        RegistersIndirect::E => { cpu.regs.e = (cpu.regs.e << 1).wrapping_add(new_bit0 as u8); },
        RegistersIndirect::H => { cpu.regs.h = (cpu.regs.h << 1).wrapping_add(new_bit0 as u8); },
        RegistersIndirect::L => { cpu.regs.l = (cpu.regs.l << 1).wrapping_add(new_bit0 as u8); },
        RegistersIndirect::HLI => {
            let new_val = (cpu.mmu.read_byte(cpu.regs.get_hl()) << 1).wrapping_add(new_bit0 as u8);
            cpu.mmu.write_byte(cpu.regs.get_hl(), new_val);
        }
    };

}

fn shift_right_register(cpu: &mut CPU, target: &RegistersIndirect, is_rrc: bool) {
    let new_bit7;
    let prev_bit0;

    match target {
        RegistersIndirect::A => prev_bit0 = get_bit_val(0,cpu.regs.a),
        RegistersIndirect::B => prev_bit0 = get_bit_val(0,cpu.regs.b),
        RegistersIndirect::C => prev_bit0 = get_bit_val(0,cpu.regs.c),
        RegistersIndirect::D => prev_bit0 = get_bit_val(0,cpu.regs.d),
        RegistersIndirect::E => prev_bit0 = get_bit_val(0,cpu.regs.e),
        RegistersIndirect::H   => prev_bit0 = get_bit_val(0,cpu.regs.h),
        RegistersIndirect::L   => prev_bit0 = get_bit_val(0,cpu.regs.l),
        RegistersIndirect::HLI => prev_bit0 = get_bit_val(0,cpu.mmu.read_byte(cpu.regs.get_hl()))
    };

    if is_rrc {
        new_bit7 = prev_bit0;
        cpu.regs.flags.carry = prev_bit0;
    }else{
        new_bit7 = cpu.regs.flags.carry;
        cpu.regs.flags.carry = prev_bit0;
    }
    
    match target {
        RegistersIndirect::A => { cpu.regs.a = (cpu.regs.a >> 1).wrapping_add((new_bit7 as u8) << 7); },
        RegistersIndirect::B => { cpu.regs.b = (cpu.regs.b >> 1).wrapping_add((new_bit7 as u8) << 7); },
        RegistersIndirect::C => { cpu.regs.c = (cpu.regs.c >> 1).wrapping_add((new_bit7 as u8) << 7); },
        RegistersIndirect::D => { cpu.regs.d = (cpu.regs.d >> 1).wrapping_add((new_bit7 as u8) << 7); },
        RegistersIndirect::E => { cpu.regs.e = (cpu.regs.e >> 1).wrapping_add((new_bit7 as u8) << 7); },
        RegistersIndirect::H => { cpu.regs.h = (cpu.regs.h >> 1).wrapping_add((new_bit7 as u8) << 7); },
        RegistersIndirect::L => { cpu.regs.l = (cpu.regs.l >> 1).wrapping_add((new_bit7 as u8) << 7); },
        RegistersIndirect::HLI => {
            let new_val = (cpu.mmu.read_byte(cpu.regs.get_hl()) >> 1).wrapping_add((new_bit7 as u8) << 7);
            cpu.mmu.write_byte(cpu.regs.get_hl(), new_val);
        }
    };

}

fn set_flag_zero(cpu: &mut CPU, target: &RegistersIndirect) {
    match target {
        RegistersIndirect::A => { cpu.regs.flags.zero = cpu.regs.a == 0; },
        RegistersIndirect::B => { cpu.regs.flags.zero = cpu.regs.b == 0; },
        RegistersIndirect::C => { cpu.regs.flags.zero = cpu.regs.c == 0; },
        RegistersIndirect::D => { cpu.regs.flags.zero = cpu.regs.d == 0; },
        RegistersIndirect::E => { cpu.regs.flags.zero = cpu.regs.e == 0; },
        RegistersIndirect::H => { cpu.regs.flags.zero = cpu.regs.h == 0; },
        RegistersIndirect::L => { cpu.regs.flags.zero = cpu.regs.l == 0; },
        RegistersIndirect::HLI => { cpu.regs.flags.zero = cpu.mmu.read_byte(cpu.regs.get_hl()) == 0; }
    };
}

fn get_position_by_bittarget(target:BitTarget) -> usize {
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

fn get_bit_val(position: usize, value:u8) -> bool {
    let mask = 1 << position;
    (mask & value) > 0
}

fn set_bit_val(position: usize, switch: bool, source: u8) -> u8 {
    if switch {
        source | 1 << position
    }else{
        source & !(1 << position)
    }
}