use std::io::Error;

use crate::{cpu::cpu::MachineCycles, gameboy::GameBoy, mmu::MMU};

use super::decode::{RegistersIndDir, WordRegister, RegistersIndirect, BitType, RotateDirection, BitTarget, ResSetType, Instruction};

impl Instruction {

    pub(super) fn add(&self, gb: &mut GameBoy, target: RegistersIndDir) -> Result<MachineCycles, Error> {
        let value = get_arithmetic_target_val(gb, &target);

        let (new_value, did_overflow) = gb.cpu.regs.a.overflowing_add(value);
        gb.cpu.regs.flags.zero = new_value == 0;
        gb.cpu.regs.flags.subtract = false;
        gb.cpu.regs.flags.carry = did_overflow;
        // Half Carry is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble.
        gb.cpu.regs.flags.half_carry = (gb.cpu.regs.a & 0xF).wrapping_add(value & 0xF) > 0xF;
        gb.cpu.regs.a = new_value;
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
        match target {
            RegistersIndDir::HLI => Ok(MachineCycles::Two),
            RegistersIndDir::D8 => Ok(MachineCycles::Two),
            _ => Ok(MachineCycles::One),
        }
    }

    pub(super) fn addsps8(&self, gb: &mut GameBoy) -> Result<MachineCycles, Error> {
        // To preserve the sign
        let value = MMU::read_next_byte(gb, gb.cpu.pc) as i8 as i16 as u16;

        let new_value = gb.cpu.sp.wrapping_add(value);
        gb.cpu.regs.flags.zero = false;
        gb.cpu.regs.flags.subtract = false;
        gb.cpu.regs.flags.carry = (gb.cpu.sp & 0xFF).wrapping_add(value & 0xFF) > 0xFF;
        gb.cpu.regs.flags.half_carry = (gb.cpu.sp & 0xF).wrapping_add(value & 0xF) > 0xF; 
        gb.cpu.sp = new_value;
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
        Ok(MachineCycles::Four)
    }

    pub(super) fn add16(&self, gb: &mut GameBoy, target: WordRegister) -> Result<MachineCycles, Error> {
        let value = match target {
            WordRegister::BC => gb.cpu.regs.get_bc(),
            WordRegister::DE => gb.cpu.regs.get_de(),
            WordRegister::HL => gb.cpu.regs.get_hl(),
            WordRegister::SP => gb.cpu.sp,
        };

        let (new_value, did_overflow) = gb.cpu.regs.get_hl().overflowing_add(value);
        gb.cpu.regs.flags.subtract = false;
        gb.cpu.regs.flags.carry = did_overflow;
        // This works for 16 bit
        gb.cpu.regs.flags.half_carry = (gb.cpu.regs.get_hl() & 0xfff).wrapping_add(value & 0xfff) > 0xfff; 
        gb.cpu.regs.set_hl(new_value);
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
        Ok(MachineCycles::Two)
    }

    pub(super) fn adc(&self, gb: &mut GameBoy, target: RegistersIndDir) -> Result<MachineCycles, Error> {
        let value = get_arithmetic_target_val(gb, &target);

        let (new_value1, did_overflow1) = gb.cpu.regs.a.overflowing_add(value);
        let (new_value2, did_overflow2) = new_value1.overflowing_add(gb.cpu.regs.flags.carry as u8);

        gb.cpu.regs.flags.zero = new_value2 == 0;
        gb.cpu.regs.flags.subtract = false;
        gb.cpu.regs.flags.half_carry = ((gb.cpu.regs.a & 0xF) + (value & 0xF) > 0xF) || ((new_value1 & 0xF) + (gb.cpu.regs.flags.carry as u8) > 0xF);
        gb.cpu.regs.flags.carry = did_overflow1 || did_overflow2;      
        gb.cpu.regs.a = new_value2;
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));

        match target {
            RegistersIndDir::HLI => Ok(MachineCycles::Two),
            RegistersIndDir::D8 => Ok(MachineCycles::Two),
            _ => Ok(MachineCycles::One),
        }
    }

    pub(super) fn sub(&self, gb: &mut GameBoy, target: RegistersIndDir) -> Result<MachineCycles, Error> {
        let value = get_arithmetic_target_val(gb, &target);

        let (new_value, did_overflow) = gb.cpu.regs.a.overflowing_sub(value);
        gb.cpu.regs.flags.zero = new_value == 0;
        gb.cpu.regs.flags.subtract = true;
        gb.cpu.regs.flags.carry = did_overflow;
        let (new_value_low, _) = (gb.cpu.regs.a & 0xF).overflowing_sub(value & 0xF);
        gb.cpu.regs.flags.half_carry = (new_value_low & 0x10) == 0x10;
        gb.cpu.regs.a = new_value;
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));

        match target {
            RegistersIndDir::HLI => Ok(MachineCycles::Two),
            RegistersIndDir::D8 => Ok(MachineCycles::Two),
            _ => Ok(MachineCycles::One),
        }
    }

    pub(super) fn sbc(&self, gb: &mut GameBoy, target: RegistersIndDir) -> Result<MachineCycles, Error> {

        let value = get_arithmetic_target_val(gb, &target);

        let (new_value1, did_overflow1) = gb.cpu.regs.a.overflowing_sub(gb.cpu.regs.flags.carry as u8);
        let (new_value2, did_overflow2) = new_value1.overflowing_sub(value);

        let half_carry_step1 = ((gb.cpu.regs.a & 0xF).wrapping_sub(gb.cpu.regs.flags.carry as u8 & 0xF) & 0x10) == 0x10;
        let half_carry_step2 = ((new_value1 & 0xF).wrapping_sub(value & 0xF) & 0x10) == 0x10;

        gb.cpu.regs.flags.half_carry = half_carry_step1 || half_carry_step2;
        gb.cpu.regs.flags.zero = new_value2 == 0;
        gb.cpu.regs.flags.subtract = true;
        gb.cpu.regs.flags.carry = did_overflow1 || did_overflow2;
        
        gb.cpu.regs.a = new_value2;
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));

        match target {
            RegistersIndDir::HLI => Ok(MachineCycles::Two),
            RegistersIndDir::D8 => Ok(MachineCycles::Two),
            _ => Ok(MachineCycles::One),
        }
    }

    pub(super) fn and(&self, gb: &mut GameBoy, target: RegistersIndDir) -> Result<MachineCycles, Error> {
        let value = get_arithmetic_target_val(gb, &target);

        gb.cpu.regs.a = gb.cpu.regs.a & value;
        gb.cpu.regs.flags.zero = gb.cpu.regs.a == 0;
        gb.cpu.regs.flags.subtract = false;
        gb.cpu.regs.flags.half_carry = true;
        gb.cpu.regs.flags.carry = false;
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));

        match target {
            RegistersIndDir::HLI => Ok(MachineCycles::Two),
            RegistersIndDir::D8 => Ok(MachineCycles::Two),
            _ => Ok(MachineCycles::One),
        }
    }

    pub(super) fn xor(&self, gb: &mut GameBoy, target: RegistersIndDir) -> Result<MachineCycles, Error> {
        let value = get_arithmetic_target_val(gb, &target);

        gb.cpu.regs.a = gb.cpu.regs.a ^ value;
        gb.cpu.regs.flags.zero = gb.cpu.regs.a == 0;
        gb.cpu.regs.flags.subtract = false;
        gb.cpu.regs.flags.half_carry = false;
        gb.cpu.regs.flags.carry = false;
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));

        match target {
            RegistersIndDir::HLI => Ok(MachineCycles::Two),
            RegistersIndDir::D8 => Ok(MachineCycles::Two),
            _ => Ok(MachineCycles::One),
        }
    }

    pub(super) fn or(&self, gb: &mut GameBoy, target: RegistersIndDir) -> Result<MachineCycles, Error> {
        let value = get_arithmetic_target_val(gb, &target);

        gb.cpu.regs.a = gb.cpu.regs.a | value;
        gb.cpu.regs.flags.zero = gb.cpu.regs.a == 0;
        gb.cpu.regs.flags.subtract = false;
        gb.cpu.regs.flags.half_carry = false;
        gb.cpu.regs.flags.carry = false;
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));

        match target {
            RegistersIndDir::HLI => Ok(MachineCycles::Two),
            RegistersIndDir::D8 => Ok(MachineCycles::Two),
            _ => Ok(MachineCycles::One),
        }
    }

    pub(super) fn cp(&self, gb: &mut GameBoy, target: RegistersIndDir) -> Result<MachineCycles, Error> {
        let value = get_arithmetic_target_val(gb, &target);

        let (result, did_overflow) = gb.cpu.regs.a.overflowing_sub(value);
        gb.cpu.regs.flags.zero = result == 0;
        gb.cpu.regs.flags.subtract = true;
        let (new_value_low, _) = (gb.cpu.regs.a & 0xF).overflowing_sub(value & 0xF);
        gb.cpu.regs.flags.half_carry = (new_value_low & 0x10) == 0x10;
        gb.cpu.regs.flags.carry = did_overflow;
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));

        match target {
            RegistersIndDir::HLI => Ok(MachineCycles::Two),
            RegistersIndDir::D8 => Ok(MachineCycles::Two),
            _ => Ok(MachineCycles::One),
        }
    }

    pub(super) fn inc(&self, gb: &mut GameBoy, target: RegistersIndirect) -> Result<MachineCycles, Error> {
        gb.cpu.regs.flags.subtract = false;

        match target {
            RegistersIndirect::A => { 
                gb.cpu.regs.flags.half_carry = (gb.cpu.regs.a & 0xF).wrapping_add(0b1 & 0xF) > 0xF;
                gb.cpu.regs.flags.zero = gb.cpu.regs.a.wrapping_add(1) == 0;
                gb.cpu.regs.a = gb.cpu.regs.a.wrapping_add(1);
            },
            RegistersIndirect::B => { 
                gb.cpu.regs.flags.half_carry = (gb.cpu.regs.b & 0xF).wrapping_add(0b1 & 0xF) > 0xF;
                gb.cpu.regs.flags.zero = gb.cpu.regs.b.wrapping_add(1) == 0;
                gb.cpu.regs.b = gb.cpu.regs.b.wrapping_add(1);
            },
            RegistersIndirect::C => { 
                gb.cpu.regs.flags.half_carry = (gb.cpu.regs.c & 0xF).wrapping_add(0b1 & 0xF) > 0xF;
                gb.cpu.regs.flags.zero = gb.cpu.regs.c.wrapping_add(1) == 0;
                gb.cpu.regs.c = gb.cpu.regs.c.wrapping_add(1);
            },
            RegistersIndirect::D => { 
                gb.cpu.regs.flags.half_carry = (gb.cpu.regs.d & 0xF).wrapping_add(0b1 & 0xF) > 0xF;
                gb.cpu.regs.flags.zero = gb.cpu.regs.d.wrapping_add(1) == 0;
                gb.cpu.regs.d = gb.cpu.regs.d.wrapping_add(1);
            },
            RegistersIndirect::E => { 
                gb.cpu.regs.flags.half_carry = (gb.cpu.regs.e & 0xF).wrapping_add(0b1 & 0xF) > 0xF;
                gb.cpu.regs.flags.zero = gb.cpu.regs.e.wrapping_add(1) == 0;
                gb.cpu.regs.e = gb.cpu.regs.e.wrapping_add(1);
            },
            RegistersIndirect::H => { 
                gb.cpu.regs.flags.half_carry = (gb.cpu.regs.h & 0xF).wrapping_add(0b1 & 0xF) > 0xF;
                gb.cpu.regs.flags.zero = gb.cpu.regs.h.wrapping_add(1) == 0;
                gb.cpu.regs.h = gb.cpu.regs.h.wrapping_add(1);
            },
            RegistersIndirect::L => { 
                gb.cpu.regs.flags.half_carry = (gb.cpu.regs.l & 0xF).wrapping_add(0b1 & 0xF) > 0xF;
                gb.cpu.regs.flags.zero = gb.cpu.regs.l.wrapping_add(1) == 0;
                gb.cpu.regs.l = gb.cpu.regs.l.wrapping_add(1);
            },
            RegistersIndirect::HLI => {
                let old_val = MMU::read_byte(gb, gb.cpu.regs.get_hl());
                gb.cpu.regs.flags.half_carry = (old_val & 0xF).wrapping_add(0b1 & 0xF) > 0xF;
                let new_val = old_val.wrapping_add(1);
                gb.cpu.regs.flags.zero = new_val == 0;
                MMU::write_byte(gb, gb.cpu.regs.get_hl(), new_val);
            }
        };
  
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));

        match target {
            RegistersIndirect::HLI => Ok(MachineCycles::Three),
            _ => Ok(MachineCycles::One),
        }
    }

    pub(super) fn dec(&self, gb: &mut GameBoy, target: RegistersIndirect) -> Result<MachineCycles, Error> {
        gb.cpu.regs.flags.subtract = true;

        match target {
            RegistersIndirect::A => { 
                gb.cpu.regs.flags.half_carry = (gb.cpu.regs.a & 0xF).wrapping_sub(0b1 & 0xF) > 0xF;
                gb.cpu.regs.flags.zero = gb.cpu.regs.a.wrapping_sub(1) == 0;
                gb.cpu.regs.a = gb.cpu.regs.a.wrapping_sub(1);
            },
            RegistersIndirect::B => { 
                gb.cpu.regs.flags.half_carry = (gb.cpu.regs.b & 0xF).wrapping_sub(0b1 & 0xF) > 0xF;
                gb.cpu.regs.flags.zero = gb.cpu.regs.b.wrapping_sub(1) == 0;
                gb.cpu.regs.b = gb.cpu.regs.b.wrapping_sub(1);
            },
            RegistersIndirect::C => { 
                gb.cpu.regs.flags.half_carry = (gb.cpu.regs.c & 0xF).wrapping_sub(0b1 & 0xF) > 0xF;
                gb.cpu.regs.flags.zero = gb.cpu.regs.c.wrapping_sub(1) == 0;
                gb.cpu.regs.c = gb.cpu.regs.c.wrapping_sub(1);
            },
            RegistersIndirect::D => { 
                gb.cpu.regs.flags.half_carry = (gb.cpu.regs.d & 0xF).wrapping_sub(0b1 & 0xF) > 0xF;
                gb.cpu.regs.flags.zero = gb.cpu.regs.d.wrapping_sub(1) == 0;
                gb.cpu.regs.d = gb.cpu.regs.d.wrapping_sub(1);
            },
            RegistersIndirect::E => { 
                gb.cpu.regs.flags.half_carry = (gb.cpu.regs.e & 0xF).wrapping_sub(0b1 & 0xF) > 0xF;
                gb.cpu.regs.flags.zero = gb.cpu.regs.e.wrapping_sub(1) == 0;
                gb.cpu.regs.e = gb.cpu.regs.e.wrapping_sub(1);
            },
            RegistersIndirect::H => { 
                gb.cpu.regs.flags.half_carry = (gb.cpu.regs.h & 0xF).wrapping_sub(0b1 & 0xF) > 0xF;
                gb.cpu.regs.flags.zero = gb.cpu.regs.h.wrapping_sub(1) == 0;
                gb.cpu.regs.h = gb.cpu.regs.h.wrapping_sub(1);
            },
            RegistersIndirect::L => { 
                gb.cpu.regs.flags.half_carry = (gb.cpu.regs.l & 0xF).wrapping_sub(0b1 & 0xF) > 0xF;
                gb.cpu.regs.flags.zero = gb.cpu.regs.l.wrapping_sub(1) == 0;
                gb.cpu.regs.l = gb.cpu.regs.l.wrapping_sub(1);
            },
            RegistersIndirect::HLI => {
                let old_val = MMU::read_byte(gb, gb.cpu.regs.get_hl());
                gb.cpu.regs.flags.half_carry = (old_val & 0xF).wrapping_sub(0b1 & 0xF) > 0xF;
                let new_val = old_val.wrapping_sub(1);
                gb.cpu.regs.flags.zero = new_val == 0;
                MMU::write_byte(gb, gb.cpu.regs.get_hl(), new_val);
            }
        };
  
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));

        match target {
            RegistersIndirect::HLI => Ok(MachineCycles::Three),
            _ => Ok(MachineCycles::One),
        }
    }

    pub(super) fn inc16(&self, gb: &mut GameBoy, target: WordRegister) -> Result<MachineCycles, Error> {
        match target {
            WordRegister::BC => gb.cpu.regs.set_bc(gb.cpu.regs.get_bc().wrapping_add(1)),
            WordRegister::DE => gb.cpu.regs.set_de(gb.cpu.regs.get_de().wrapping_add(1)),
            WordRegister::HL => gb.cpu.regs.set_hl(gb.cpu.regs.get_hl().wrapping_add(1)),
            WordRegister::SP => gb.cpu.sp = gb.cpu.sp.wrapping_add(1),
        };
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
        Ok(MachineCycles::Two)
    }

    pub(super) fn dec16(&self, gb: &mut GameBoy, target: WordRegister) -> Result<MachineCycles, Error> {
        match target {
            WordRegister::BC => gb.cpu.regs.set_bc(gb.cpu.regs.get_bc().wrapping_sub(1)),
            WordRegister::DE => gb.cpu.regs.set_de(gb.cpu.regs.get_de().wrapping_sub(1)),
            WordRegister::HL => gb.cpu.regs.set_hl(gb.cpu.regs.get_hl().wrapping_sub(1)),
            WordRegister::SP => gb.cpu.sp = gb.cpu.sp.wrapping_sub(1),
        };
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
        Ok(MachineCycles::Two)
    }

    pub(super) fn bit(&self, gb: &mut GameBoy, bit_type: BitType) -> Result<MachineCycles, Error> {
        let BitType::Registers(t, s) = bit_type;
        let target = t;
        let source = s;

        let i = get_position_by_bittarget(target);
        let value = get_register_indirect_val(gb, source.clone());
        let bit_value = get_bit_val(i, value);

        gb.cpu.regs.flags.zero = !bit_value;
        gb.cpu.regs.flags.subtract = false;
        gb.cpu.regs.flags.half_carry = true;
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));

        match source {
            RegistersIndirect::HLI => Ok(MachineCycles::Three),
            _ => Ok(MachineCycles::Two),
        }             
    }

    // RLA, RRA, ... are legacy instructions made for compatibility with 8080
    // No zero flag is set
    pub(super) fn rla(&self, gb: &mut GameBoy) -> Result<MachineCycles, Error> {
        bitwise_rotate(gb, &RegistersIndirect::A, RotateDirection::Left, false);
        gb.cpu.regs.flags.zero = false;
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
        Ok(MachineCycles::One)
    }

    pub(super) fn rlca(&self, gb: &mut GameBoy) -> Result<MachineCycles, Error> {
        bitwise_rotate(gb, &RegistersIndirect::A, RotateDirection::Left, true);
        gb.cpu.regs.flags.zero = false;
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
        Ok(MachineCycles::One)
    }

    pub(super) fn rra(&self, gb: &mut GameBoy) -> Result<MachineCycles, Error> {
        bitwise_rotate(gb, &RegistersIndirect::A, RotateDirection::Right, false);
        gb.cpu.regs.flags.zero = false;
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
        Ok(MachineCycles::One)
    }

    pub(super) fn rrca(&self, gb: &mut GameBoy) -> Result<MachineCycles, Error> {
        bitwise_rotate(gb, &RegistersIndirect::A, RotateDirection::Right, true);
        gb.cpu.regs.flags.zero = false;
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
        Ok(MachineCycles::One)
    }

    pub(super) fn sla(&self, gb: &mut GameBoy, target: RegistersIndirect) -> Result<MachineCycles, Error> {
        bitwise_rotate(gb, &target, RotateDirection::Left, true);
        res_set(gb, ResSetType::Registers(BitTarget::Zero, target.clone()), false);
        set_flag_zero(gb, &target);
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
        
        match target {
            RegistersIndirect::HLI => Ok(MachineCycles::Four),
            _ => Ok(MachineCycles::Two),
        }
    }

    pub(super) fn sra(&self, gb: &mut GameBoy, target: RegistersIndirect) -> Result<MachineCycles, Error> {
        let value = get_register_indirect_val(gb, target.clone());
        let bit7 = get_bit_val(7, value);

        bitwise_rotate(gb, &target, RotateDirection::Right, true);
        res_set(gb, ResSetType::Registers(BitTarget::Seven, target.clone()), bit7);
        set_flag_zero(gb, &target);
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));

        match target {
            RegistersIndirect::HLI => Ok(MachineCycles::Four),
            _ => Ok(MachineCycles::Two),
        }
    }

    pub(super) fn srl(&self, gb: &mut GameBoy, target: RegistersIndirect) -> Result<MachineCycles, Error> {
        bitwise_rotate(gb, &target, RotateDirection::Right, true);
        res_set(gb, ResSetType::Registers(BitTarget::Seven, target.clone()), false);
        set_flag_zero(gb, &target);
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
        
        match target {
            RegistersIndirect::HLI => Ok(MachineCycles::Four),
            _ => Ok(MachineCycles::Two),
        }
    }

    pub(super) fn rr(&self, gb: &mut GameBoy, target: RegistersIndirect) -> Result<MachineCycles, Error> {
        bitwise_rotate(gb, &target, RotateDirection::Right, false);
        set_flag_zero(gb, &target);
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
        
        match target {
            RegistersIndirect::HLI => Ok(MachineCycles::Four),
            _ => Ok(MachineCycles::Two),
        }
    }

    pub(super) fn rrc(&self, gb: &mut GameBoy, target: RegistersIndirect) -> Result<MachineCycles, Error> {
        bitwise_rotate(gb, &target, RotateDirection::Right, true);
        set_flag_zero(gb, &target);
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));

        match target {
            RegistersIndirect::HLI => Ok(MachineCycles::Four),
            _ => Ok(MachineCycles::Two),
        }
    }

    pub(super) fn rl(&self, gb: &mut GameBoy, target: RegistersIndirect) -> Result<MachineCycles, Error> {
        bitwise_rotate(gb, &target, RotateDirection::Left, false);
        set_flag_zero(gb, &target);
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));

        match target {
            RegistersIndirect::HLI => Ok(MachineCycles::Four),
            _ => Ok(MachineCycles::Two),
        }
    }

    pub(super) fn rlc(&self, gb: &mut GameBoy, target: RegistersIndirect) -> Result<MachineCycles, Error> {
        bitwise_rotate(gb, &target, RotateDirection::Left, true);
        set_flag_zero(gb, &target);
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));

        match target {
            RegistersIndirect::HLI => Ok(MachineCycles::Four),
            _ => Ok(MachineCycles::Two),
        }
    }

    pub(super) fn swap(&self, gb: &mut GameBoy, target: RegistersIndirect) -> Result<MachineCycles, Error> {
        let value = get_register_indirect_val(gb, target.clone());

        let low = value & 0x0F;
        let high = value & 0xF0;

        let new_value = (low << 4).wrapping_add(high >> 4);

        match target {
            RegistersIndirect::A => { gb.cpu.regs.a = new_value; },
            RegistersIndirect::B => { gb.cpu.regs.b = new_value; },
            RegistersIndirect::C => { gb.cpu.regs.c = new_value; },
            RegistersIndirect::D => { gb.cpu.regs.d = new_value; },
            RegistersIndirect::E => { gb.cpu.regs.e = new_value; },
            RegistersIndirect::H => { gb.cpu.regs.h = new_value; },
            RegistersIndirect::L => { gb.cpu.regs.l = new_value; },
            RegistersIndirect::HLI => {
                MMU::write_byte(gb, gb.cpu.regs.get_hl(), new_value);
            }
        };

        gb.cpu.regs.flags.subtract = false;
        gb.cpu.regs.flags.half_carry = false;
        gb.cpu.regs.flags.carry = false;
        set_flag_zero(gb, &target);
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
        Ok(MachineCycles::Two)
    }

    pub(super) fn res(&self, gb: &mut GameBoy, target: ResSetType) -> Result<MachineCycles, Error> {

        let ResSetType::Registers(_, register) = target.clone();

        res_set(gb, target, false);
        
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
        
        match register {
            RegistersIndirect::HLI => Ok(MachineCycles::Four),
            _ => Ok(MachineCycles::Two),
        }
    }

    pub(super) fn set(&self, gb: &mut GameBoy, target: ResSetType) -> Result<MachineCycles, Error> {

        let ResSetType::Registers(_, register) = target.clone();

        res_set(gb, target, true);
        
        gb.cpu.pc = gb.cpu.pc.wrapping_add(u16::from(self.size()));
        
        match register {
            RegistersIndirect::HLI => Ok(MachineCycles::Four),
            _ => Ok(MachineCycles::Two),
        }
    }
}

// RR r and RL r instructions
// If is_rc is true we consider the RLC and RRC instructions, otherwise the RL and RR
fn bitwise_rotate(gb: &mut GameBoy, target: &RegistersIndirect, direction: RotateDirection, is_rc: bool) {
    gb.cpu.regs.flags.subtract = false;
    gb.cpu.regs.flags.half_carry = false;

    match direction {
        RotateDirection::Left => shift_left_register(gb, &target, is_rc),
        RotateDirection::Right => shift_right_register(gb, &target, is_rc),
    }
}

pub(super) fn res_set(gb: &mut GameBoy, target: ResSetType, value: bool) {
    let ResSetType::Registers(bt, register) = target;
    
    let i = get_position_by_bittarget(bt);

    match register {
        RegistersIndirect::A => gb.cpu.regs.a = set_bit_val(i, value, gb.cpu.regs.a),
        RegistersIndirect::B => gb.cpu.regs.b = set_bit_val(i, value, gb.cpu.regs.b),
        RegistersIndirect::C => gb.cpu.regs.c = set_bit_val(i, value, gb.cpu.regs.c),
        RegistersIndirect::D => gb.cpu.regs.d = set_bit_val(i, value, gb.cpu.regs.d),
        RegistersIndirect::E => gb.cpu.regs.e = set_bit_val(i, value, gb.cpu.regs.e),
        RegistersIndirect::H   => gb.cpu.regs.h = set_bit_val(i, value, gb.cpu.regs.h),
        RegistersIndirect::L   => gb.cpu.regs.l = set_bit_val(i, value, gb.cpu.regs.l),
        RegistersIndirect::HLI => {
            let new_value = set_bit_val(i, value, MMU::read_byte(gb, gb.cpu.regs.get_hl()));
            MMU::write_byte(gb, gb.cpu.regs.get_hl(), new_value);
        }
    };
}

fn get_register_indirect_val(gb: &GameBoy, source: RegistersIndirect) -> u8 {
    match source {
        RegistersIndirect::A => gb.cpu.regs.a,
        RegistersIndirect::B => gb.cpu.regs.b,
        RegistersIndirect::C => gb.cpu.regs.c,
        RegistersIndirect::D => gb.cpu.regs.d,
        RegistersIndirect::E => gb.cpu.regs.e,
        RegistersIndirect::H => gb.cpu.regs.h,
        RegistersIndirect::L => gb.cpu.regs.l,
        RegistersIndirect::HLI => MMU::read_byte(gb, gb.cpu.regs.get_hl()),
    }
}

fn get_arithmetic_target_val(gb: &GameBoy, target: &RegistersIndDir) -> u8 {
    match target {
        RegistersIndDir::A     => gb.cpu.regs.a,
        RegistersIndDir::B     => gb.cpu.regs.b,
        RegistersIndDir::C     => gb.cpu.regs.c,
        RegistersIndDir::D     => gb.cpu.regs.d,
        RegistersIndDir::E     => gb.cpu.regs.e,
        RegistersIndDir::H     => gb.cpu.regs.h,
        RegistersIndDir::L     => gb.cpu.regs.l,
        RegistersIndDir::HLI   => MMU::read_byte(gb, gb.cpu.regs.get_hl()),
        RegistersIndDir::D8    => MMU::read_next_byte(gb, gb.cpu.pc)
    }
}

fn shift_left_register(gb: &mut GameBoy, target: &RegistersIndirect, is_rlc: bool) {
    let new_bit0;
    let prev_bit7;

    match target {
        RegistersIndirect::A => prev_bit7 = get_bit_val(7,gb.cpu.regs.a),
        RegistersIndirect::B => prev_bit7 = get_bit_val(7,gb.cpu.regs.b),
        RegistersIndirect::C => prev_bit7 = get_bit_val(7,gb.cpu.regs.c),
        RegistersIndirect::D => prev_bit7 = get_bit_val(7,gb.cpu.regs.d),
        RegistersIndirect::E => prev_bit7 = get_bit_val(7,gb.cpu.regs.e),
        RegistersIndirect::H   => prev_bit7 = get_bit_val(7,gb.cpu.regs.h),
        RegistersIndirect::L   => prev_bit7 = get_bit_val(7,gb.cpu.regs.l),
        RegistersIndirect::HLI => { 
            let hl_value = MMU::read_byte(gb, gb.cpu.regs.get_hl());
            prev_bit7 = get_bit_val(7,hl_value);
        }
    };

    if is_rlc {
        new_bit0 = prev_bit7;
        gb.cpu.regs.flags.carry = prev_bit7;
    }else{
        new_bit0 = gb.cpu.regs.flags.carry;
        gb.cpu.regs.flags.carry = prev_bit7;
    }

    match target {
        RegistersIndirect::A => { gb.cpu.regs.a = (gb.cpu.regs.a << 1).wrapping_add(new_bit0 as u8); },
        RegistersIndirect::B => { gb.cpu.regs.b = (gb.cpu.regs.b << 1).wrapping_add(new_bit0 as u8); },
        RegistersIndirect::C => { gb.cpu.regs.c = (gb.cpu.regs.c << 1).wrapping_add(new_bit0 as u8); },
        RegistersIndirect::D => { gb.cpu.regs.d = (gb.cpu.regs.d << 1).wrapping_add(new_bit0 as u8); },
        RegistersIndirect::E => { gb.cpu.regs.e = (gb.cpu.regs.e << 1).wrapping_add(new_bit0 as u8); },
        RegistersIndirect::H => { gb.cpu.regs.h = (gb.cpu.regs.h << 1).wrapping_add(new_bit0 as u8); },
        RegistersIndirect::L => { gb.cpu.regs.l = (gb.cpu.regs.l << 1).wrapping_add(new_bit0 as u8); },
        RegistersIndirect::HLI => {
            let new_val = (MMU::read_byte(gb, gb.cpu.regs.get_hl()) << 1).wrapping_add(new_bit0 as u8);
            MMU::write_byte(gb, gb.cpu.regs.get_hl(), new_val);
        }
    };

}

fn shift_right_register(gb: &mut GameBoy, target: &RegistersIndirect, is_rrc: bool) {
    let new_bit7;
    let prev_bit0;

    match target {
        RegistersIndirect::A => prev_bit0 = get_bit_val(0,gb.cpu.regs.a),
        RegistersIndirect::B => prev_bit0 = get_bit_val(0,gb.cpu.regs.b),
        RegistersIndirect::C => prev_bit0 = get_bit_val(0,gb.cpu.regs.c),
        RegistersIndirect::D => prev_bit0 = get_bit_val(0,gb.cpu.regs.d),
        RegistersIndirect::E => prev_bit0 = get_bit_val(0,gb.cpu.regs.e),
        RegistersIndirect::H   => prev_bit0 = get_bit_val(0,gb.cpu.regs.h),
        RegistersIndirect::L   => prev_bit0 = get_bit_val(0,gb.cpu.regs.l),
        RegistersIndirect::HLI => prev_bit0 = get_bit_val(0,MMU::read_byte(gb, gb.cpu.regs.get_hl()))
    };

    if is_rrc {
        new_bit7 = prev_bit0;
        gb.cpu.regs.flags.carry = prev_bit0;
    }else{
        new_bit7 = gb.cpu.regs.flags.carry;
        gb.cpu.regs.flags.carry = prev_bit0;
    }
    
    match target {
        RegistersIndirect::A => { gb.cpu.regs.a = (gb.cpu.regs.a >> 1).wrapping_add((new_bit7 as u8) << 7); },
        RegistersIndirect::B => { gb.cpu.regs.b = (gb.cpu.regs.b >> 1).wrapping_add((new_bit7 as u8) << 7); },
        RegistersIndirect::C => { gb.cpu.regs.c = (gb.cpu.regs.c >> 1).wrapping_add((new_bit7 as u8) << 7); },
        RegistersIndirect::D => { gb.cpu.regs.d = (gb.cpu.regs.d >> 1).wrapping_add((new_bit7 as u8) << 7); },
        RegistersIndirect::E => { gb.cpu.regs.e = (gb.cpu.regs.e >> 1).wrapping_add((new_bit7 as u8) << 7); },
        RegistersIndirect::H => { gb.cpu.regs.h = (gb.cpu.regs.h >> 1).wrapping_add((new_bit7 as u8) << 7); },
        RegistersIndirect::L => { gb.cpu.regs.l = (gb.cpu.regs.l >> 1).wrapping_add((new_bit7 as u8) << 7); },
        RegistersIndirect::HLI => {
            let new_val = (MMU::read_byte(gb, gb.cpu.regs.get_hl()) >> 1).wrapping_add((new_bit7 as u8) << 7);
            MMU::write_byte(gb, gb.cpu.regs.get_hl(), new_val);
        }
    };

}

fn set_flag_zero(gb: &mut GameBoy, target: &RegistersIndirect) {
    match target {
        RegistersIndirect::A => { gb.cpu.regs.flags.zero = gb.cpu.regs.a == 0; },
        RegistersIndirect::B => { gb.cpu.regs.flags.zero = gb.cpu.regs.b == 0; },
        RegistersIndirect::C => { gb.cpu.regs.flags.zero = gb.cpu.regs.c == 0; },
        RegistersIndirect::D => { gb.cpu.regs.flags.zero = gb.cpu.regs.d == 0; },
        RegistersIndirect::E => { gb.cpu.regs.flags.zero = gb.cpu.regs.e == 0; },
        RegistersIndirect::H => { gb.cpu.regs.flags.zero = gb.cpu.regs.h == 0; },
        RegistersIndirect::L => { gb.cpu.regs.flags.zero = gb.cpu.regs.l == 0; },
        RegistersIndirect::HLI => { gb.cpu.regs.flags.zero = MMU::read_byte(gb, gb.cpu.regs.get_hl()) == 0; }
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