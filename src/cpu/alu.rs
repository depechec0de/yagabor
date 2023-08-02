use crate::cpu::*;

impl CPU {
    pub(super) fn add(&mut self, target: ArithmeticTarget) -> ProgramCounter {
        let value = self.get_arithmetic_target_val(&target);

        let (new_value, did_overflow) = self.regs.a.overflowing_add(value);
        self.regs.flags.zero = new_value == 0;
        self.regs.flags.subtract = false;
        self.regs.flags.carry = did_overflow;
        // Half Carry is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble.
        self.regs.flags.half_carry = (self.regs.a & 0xF) + (value & 0xF) > 0xF;
        self.regs.a = new_value;

        self.arithmetic_pc_increment(&target)
    }

    pub(super) fn addsp8(&mut self) -> ProgramCounter {
        let value = self.read_next_byte() as u16;

        let (new_value, did_overflow) = self.sp.overflowing_add(value);
        self.regs.flags.zero = false;
        self.regs.flags.subtract = false;
        self.regs.flags.carry = did_overflow;
        self.regs.flags.half_carry = (self.sp & 0xF) + (value & 0xF) > 0xF;
        self.sp = new_value;

        self.pc.wrapping_add(2)
    }

    pub(super) fn add16(&mut self, target: WordRegister) -> ProgramCounter {
        let value = match target {
            WordRegister::BC => self.regs.get_bc(),
            WordRegister::DE => self.regs.get_de(),
            WordRegister::HL => self.regs.get_hl(),
            WordRegister::SP => self.sp,
        };

        let (new_value, did_overflow) = self.regs.get_hl().overflowing_add(value);
        self.regs.flags.subtract = false;
        self.regs.flags.carry = did_overflow;
        self.regs.flags.half_carry = (self.regs.get_hl() & 0xF) + (value & 0xF) > 0xF;
        self.regs.set_hl(new_value);

        self.pc.wrapping_add(1)
    }

    pub(super) fn adc(&mut self, target: ArithmeticTarget) -> ProgramCounter {
        let value = self.get_arithmetic_target_val(&target);

        let (new_value1, did_overflow1) = self.regs.a.overflowing_add(value);

        let (new_value2, did_overflow2) = new_value1.overflowing_add(self.regs.flags.carry as u8);

        self.regs.flags.zero = new_value2 == 0;
        self.regs.flags.subtract = false;
        self.regs.flags.half_carry = ((self.regs.a & 0xF) + (value & 0xF) > 0xF) || ((new_value1 & 0xF) + (self.regs.flags.carry as u8) > 0xF);
        self.regs.flags.carry = did_overflow1 || did_overflow2;      
        self.regs.a = new_value2;

        self.arithmetic_pc_increment(&target)
    }

    pub(super) fn sub(&mut self, target: ArithmeticTarget) -> ProgramCounter {
        let value = self.get_arithmetic_target_val(&target);

        let (new_value, did_overflow) = self.regs.a.overflowing_sub(value);
        self.regs.flags.zero = new_value == 0;
        self.regs.flags.subtract = true;
        self.regs.flags.carry = did_overflow;
        let (new_value_low, _) = (self.regs.a & 0xF).overflowing_sub(value & 0xF);
        self.regs.flags.half_carry = (new_value_low & 0x10) == 0x10;
        self.regs.a = new_value;

        self.arithmetic_pc_increment(&target)
    }

    pub(super) fn sbc(&mut self, target: ArithmeticTarget) -> ProgramCounter {
        let value = self.get_arithmetic_target_val(&target);

        let (new_value1, did_overflow1) = self.regs.a.overflowing_sub(self.regs.flags.carry as u8);
        let (new_value2, did_overflow2) = new_value1.overflowing_sub(value);

        self.regs.flags.zero = new_value2 == 0;
        self.regs.flags.subtract = true;
        self.regs.flags.carry = did_overflow1 || did_overflow2;
        let (new_value_low, _) = (new_value2 & 0xF).overflowing_sub(value & 0xF);
        self.regs.flags.half_carry = (new_value_low & 0x10) == 0x10;
        self.regs.a = new_value2;

        self.arithmetic_pc_increment(&target)
    }

    pub(super) fn and(&mut self, target: ArithmeticTarget) -> ProgramCounter {
        let value = self.get_arithmetic_target_val(&target);

        self.regs.a = self.regs.a & value;
        self.regs.flags.zero = self.regs.a == 0;
        self.regs.flags.subtract = false;
        self.regs.flags.half_carry = true;
        self.regs.flags.carry = false;

        self.arithmetic_pc_increment(&target)
    }

    pub(super) fn xor(&mut self, target: ArithmeticTarget) -> ProgramCounter {
        let value = self.get_arithmetic_target_val(&target);

        self.regs.a = self.regs.a ^ value;
        self.regs.flags.zero = self.regs.a == 0;
        self.regs.flags.subtract = false;
        self.regs.flags.half_carry = false;
        self.regs.flags.carry = false;

        self.arithmetic_pc_increment(&target)
    }

    pub(super) fn or(&mut self, target: ArithmeticTarget) -> ProgramCounter {
        let value = self.get_arithmetic_target_val(&target);

        self.regs.a = self.regs.a | value;
        self.regs.flags.zero = self.regs.a == 0;
        self.regs.flags.subtract = false;
        self.regs.flags.half_carry = false;
        self.regs.flags.carry = false;

        self.arithmetic_pc_increment(&target)
    }

    pub(super) fn cp(&mut self, target: ArithmeticTarget) -> ProgramCounter {
        let value = self.get_arithmetic_target_val(&target);

        let (result, did_overflow) = self.regs.a.overflowing_sub(value);
        self.regs.flags.zero = result == 0;
        self.regs.flags.subtract = true;
        let (new_value_low, _) = (self.regs.a & 0xF).overflowing_sub(value & 0xF);
        self.regs.flags.half_carry = (new_value_low & 0x10) == 0x10;
        self.regs.flags.carry = did_overflow;

        self.arithmetic_pc_increment(&target)
    }

    pub(super) fn inc(&mut self, target: IncDecTarget) -> ProgramCounter {
        let value = match target {
            IncDecTarget::A => self.regs.a = self.regs.a.wrapping_add(1),
            IncDecTarget::B => self.regs.b = self.regs.b.wrapping_add(1),
            IncDecTarget::C => self.regs.c = self.regs.c.wrapping_add(1),
            IncDecTarget::D => self.regs.d = self.regs.d.wrapping_add(1),
            IncDecTarget::E => self.regs.e = self.regs.e.wrapping_add(1),
            IncDecTarget::H => self.regs.h = self.regs.h.wrapping_add(1),
            IncDecTarget::L => self.regs.l = self.regs.l.wrapping_add(1),
            IncDecTarget::HLI => {
                let new_val = self.bus.read_byte(self.regs.get_hl()).wrapping_add(1);
                self.bus.write_byte(self.regs.get_hl(), new_val);
            }
        };
        self.pc.wrapping_add(1)
    }

    pub(super) fn dec(&mut self, target: IncDecTarget) -> ProgramCounter {
        let value = match target {
            IncDecTarget::A => self.regs.a = self.regs.a.wrapping_sub(1),
            IncDecTarget::B => self.regs.b = self.regs.b.wrapping_sub(1),
            IncDecTarget::C => self.regs.c = self.regs.c.wrapping_sub(1),
            IncDecTarget::D => self.regs.d = self.regs.d.wrapping_sub(1),
            IncDecTarget::E => self.regs.e = self.regs.e.wrapping_sub(1),
            IncDecTarget::H => self.regs.h = self.regs.h.wrapping_sub(1),
            IncDecTarget::L => self.regs.l = self.regs.l.wrapping_sub(1),
            IncDecTarget::HLI => {
                let new_val = self.bus.read_byte(self.regs.get_hl()).wrapping_sub(1);
                self.bus.write_byte(self.regs.get_hl(), new_val);
            }
        };
        self.pc.wrapping_add(1)
    }

    pub(super) fn inc16(&mut self, target: WordRegister) -> ProgramCounter {
        let value = match target {
            WordRegister::BC => self.regs.set_bc(self.regs.get_bc().wrapping_add(1)),
            WordRegister::DE => self.regs.set_de(self.regs.get_de().wrapping_add(1)),
            WordRegister::HL => self.regs.set_hl(self.regs.get_hl().wrapping_add(1)),
            WordRegister::SP => self.sp = self.sp.wrapping_add(1),
        };
        self.pc.wrapping_add(1)
    }

    pub(super) fn dec16(&mut self, target: WordRegister) -> ProgramCounter {
        let value = match target {
            WordRegister::BC => self.regs.set_bc(self.regs.get_bc().wrapping_sub(1)),
            WordRegister::DE => self.regs.set_de(self.regs.get_de().wrapping_sub(1)),
            WordRegister::HL => self.regs.set_hl(self.regs.get_hl().wrapping_sub(1)),
            WordRegister::SP => self.sp = self.sp.wrapping_sub(1),
        };
        self.pc.wrapping_add(1)
    }

    fn arithmetic_pc_increment(&self, target: &ArithmeticTarget) -> ProgramCounter {
        let is_d8: ProgramCounter = match target {
            ArithmeticTarget::D8 => 1,
            _ => 0
        }; 
        self.pc.wrapping_add(1+is_d8)
    }

    fn get_arithmetic_target_val(&self, target: &ArithmeticTarget) -> u8 {
        match target {
            ArithmeticTarget::A => self.regs.a,
            ArithmeticTarget::B => self.regs.b,
            ArithmeticTarget::C => self.regs.c,
            ArithmeticTarget::D => self.regs.d,
            ArithmeticTarget::E => self.regs.e,
            ArithmeticTarget::H => self.regs.h,
            ArithmeticTarget::L => self.regs.l,
            ArithmeticTarget::HLI => self.bus.read_byte(self.regs.get_hl()),
            ArithmeticTarget::D8 => self.read_next_byte()
        }
    }
}