use super::*;

impl CPU {
    pub(super) fn add(&mut self, target: ArithmeticTarget) {
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
    }

    pub(super) fn addsp8(&mut self) {
        let value = self.read_next_byte() as u16;

        let (new_value, did_overflow) = self.sp.overflowing_add(value);
        self.regs.flags.zero = false;
        self.regs.flags.subtract = false;
        self.regs.flags.carry = did_overflow;
        self.regs.flags.half_carry = (self.sp & 0xF) + (value & 0xF) > 0xF;
        self.sp = new_value;
    }

    pub(super) fn add16(&mut self, target: WordRegister) {
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
    }

    pub(super) fn adc(&mut self, target: ArithmeticTarget) {
        let value = self.get_arithmetic_target_val(&target);

        let (new_value1, did_overflow1) = self.regs.a.overflowing_add(value);

        let (new_value2, did_overflow2) = new_value1.overflowing_add(self.regs.flags.carry as u8);

        self.regs.flags.zero = new_value2 == 0;
        self.regs.flags.subtract = false;
        self.regs.flags.half_carry = ((self.regs.a & 0xF) + (value & 0xF) > 0xF) || ((new_value1 & 0xF) + (self.regs.flags.carry as u8) > 0xF);
        self.regs.flags.carry = did_overflow1 || did_overflow2;      
        self.regs.a = new_value2;
    }

    pub(super) fn sub(&mut self, target: ArithmeticTarget) {
        let value = self.get_arithmetic_target_val(&target);

        let (new_value, did_overflow) = self.regs.a.overflowing_sub(value);
        self.regs.flags.zero = new_value == 0;
        self.regs.flags.subtract = true;
        self.regs.flags.carry = did_overflow;
        let (new_value_low, _) = (self.regs.a & 0xF).overflowing_sub(value & 0xF);
        self.regs.flags.half_carry = (new_value_low & 0x10) == 0x10;
        self.regs.a = new_value;
    }

    pub(super) fn sbc(&mut self, target: ArithmeticTarget) {
        let value = self.get_arithmetic_target_val(&target);

        let (new_value1, did_overflow1) = self.regs.a.overflowing_sub(self.regs.flags.carry as u8);
        let (new_value2, did_overflow2) = new_value1.overflowing_sub(value);

        self.regs.flags.zero = new_value2 == 0;
        self.regs.flags.subtract = true;
        self.regs.flags.carry = did_overflow1 || did_overflow2;
        let (new_value_low, _) = (new_value2 & 0xF).overflowing_sub(value & 0xF);
        self.regs.flags.half_carry = (new_value_low & 0x10) == 0x10;
        self.regs.a = new_value2;
    }

    pub(super) fn and(&mut self, target: ArithmeticTarget) {
        let value = self.get_arithmetic_target_val(&target);

        self.regs.a = self.regs.a & value;
        self.regs.flags.zero = self.regs.a == 0;
        self.regs.flags.subtract = false;
        self.regs.flags.half_carry = true;
        self.regs.flags.carry = false;
    }

    pub(super) fn xor(&mut self, target: ArithmeticTarget) {
        let value = self.get_arithmetic_target_val(&target);

        self.regs.a = self.regs.a ^ value;
        self.regs.flags.zero = self.regs.a == 0;
        self.regs.flags.subtract = false;
        self.regs.flags.half_carry = false;
        self.regs.flags.carry = false;

    }

    pub(super) fn or(&mut self, target: ArithmeticTarget) {
        let value = self.get_arithmetic_target_val(&target);

        self.regs.a = self.regs.a | value;
        self.regs.flags.zero = self.regs.a == 0;
        self.regs.flags.subtract = false;
        self.regs.flags.half_carry = false;
        self.regs.flags.carry = false;
    }

    pub(super) fn cp(&mut self, target: ArithmeticTarget) {
        let value = self.get_arithmetic_target_val(&target);

        let (result, did_overflow) = self.regs.a.overflowing_sub(value);
        self.regs.flags.zero = result == 0;
        self.regs.flags.subtract = true;
        let (new_value_low, _) = (self.regs.a & 0xF).overflowing_sub(value & 0xF);
        self.regs.flags.half_carry = (new_value_low & 0x10) == 0x10;
        self.regs.flags.carry = did_overflow;

    }

    pub(super) fn inc(&mut self, target: IncDecTarget) {
        self.regs.flags.subtract = false;

        match target {
            IncDecTarget::A => { 
                self.regs.flags.half_carry = (self.regs.a & 0xF).wrapping_add(0b1 & 0xF) > 0xF;
                self.regs.flags.zero = self.regs.a.wrapping_add(1) == 0;
                self.regs.a = self.regs.a.wrapping_add(1);
            },
            IncDecTarget::B => { 
                self.regs.flags.half_carry = (self.regs.b & 0xF).wrapping_add(0b1 & 0xF) > 0xF;
                self.regs.flags.zero = self.regs.b.wrapping_add(1) == 0;
                self.regs.b = self.regs.b.wrapping_add(1);
            },
            IncDecTarget::C => { 
                self.regs.flags.half_carry = (self.regs.c & 0xF).wrapping_add(0b1 & 0xF) > 0xF;
                self.regs.flags.zero = self.regs.c.wrapping_add(1) == 0;
                self.regs.c = self.regs.c.wrapping_add(1);
            },
            IncDecTarget::D => { 
                self.regs.flags.half_carry = (self.regs.d & 0xF).wrapping_add(0b1 & 0xF) > 0xF;
                self.regs.flags.zero = self.regs.d.wrapping_add(1) == 0;
                self.regs.d = self.regs.d.wrapping_add(1);
            },
            IncDecTarget::E => { 
                self.regs.flags.half_carry = (self.regs.e & 0xF).wrapping_add(0b1 & 0xF) > 0xF;
                self.regs.flags.zero = self.regs.e.wrapping_add(1) == 0;
                self.regs.e = self.regs.e.wrapping_add(1);
            },
            IncDecTarget::H => { 
                self.regs.flags.half_carry = (self.regs.h & 0xF).wrapping_add(0b1 & 0xF) > 0xF;
                self.regs.flags.zero = self.regs.h.wrapping_add(1) == 0;
                self.regs.h = self.regs.h.wrapping_add(1);
            },
            IncDecTarget::L => { 
                self.regs.flags.half_carry = (self.regs.l & 0xF).wrapping_add(0b1 & 0xF) > 0xF;
                self.regs.flags.zero = self.regs.l.wrapping_add(1) == 0;
                self.regs.l = self.regs.l.wrapping_add(1);
            },
            IncDecTarget::HLI => {
                let old_val = self.mmu.read_byte(self.regs.get_hl());
                self.regs.flags.half_carry = (old_val & 0xF).wrapping_add(0b1 & 0xF) > 0xF;
                let new_val = old_val.wrapping_add(1);
                self.regs.flags.zero = new_val == 0;
                self.mmu.write_byte(self.regs.get_hl(), new_val);
            }
        };
    }

    pub(super) fn dec(&mut self, target: IncDecTarget) {
        self.regs.flags.subtract = true;

        match target {
            IncDecTarget::A => { 
                self.regs.flags.half_carry = (self.regs.a & 0xF).wrapping_sub(0b1 & 0xF) > 0xF;
                self.regs.flags.zero = self.regs.a.wrapping_sub(1) == 0;
                self.regs.a = self.regs.a.wrapping_sub(1);
            },
            IncDecTarget::B => { 
                self.regs.flags.half_carry = (self.regs.b & 0xF).wrapping_sub(0b1 & 0xF) > 0xF;
                self.regs.flags.zero = self.regs.b.wrapping_sub(1) == 0;
                self.regs.b = self.regs.b.wrapping_sub(1);
            },
            IncDecTarget::C => { 
                self.regs.flags.half_carry = (self.regs.c & 0xF).wrapping_sub(0b1 & 0xF) > 0xF;
                self.regs.flags.zero = self.regs.c.wrapping_sub(1) == 0;
                self.regs.c = self.regs.c.wrapping_sub(1);
            },
            IncDecTarget::D => { 
                self.regs.flags.half_carry = (self.regs.d & 0xF).wrapping_sub(0b1 & 0xF) > 0xF;
                self.regs.flags.zero = self.regs.d.wrapping_sub(1) == 0;
                self.regs.d = self.regs.d.wrapping_sub(1);
            },
            IncDecTarget::E => { 
                self.regs.flags.half_carry = (self.regs.e & 0xF).wrapping_sub(0b1 & 0xF) > 0xF;
                self.regs.flags.zero = self.regs.e.wrapping_sub(1) == 0;
                self.regs.e = self.regs.e.wrapping_sub(1);
            },
            IncDecTarget::H => { 
                self.regs.flags.half_carry = (self.regs.h & 0xF).wrapping_sub(0b1 & 0xF) > 0xF;
                self.regs.flags.zero = self.regs.h.wrapping_sub(1) == 0;
                self.regs.h = self.regs.h.wrapping_sub(1);
            },
            IncDecTarget::L => { 
                self.regs.flags.half_carry = (self.regs.l & 0xF).wrapping_sub(0b1 & 0xF) > 0xF;
                self.regs.flags.zero = self.regs.l.wrapping_sub(1) == 0;
                self.regs.l = self.regs.l.wrapping_sub(1);
            },
            IncDecTarget::HLI => {
                let old_val = self.mmu.read_byte(self.regs.get_hl());
                self.regs.flags.half_carry = (old_val & 0xF).wrapping_sub(0b1 & 0xF) > 0xF;
                let new_val = old_val.wrapping_sub(1);
                self.regs.flags.zero = new_val == 0;
                self.mmu.write_byte(self.regs.get_hl(), new_val);
            }
        };
    }

    pub(super) fn inc16(&mut self, target: WordRegister) {
        let value = match target {
            WordRegister::BC => self.regs.set_bc(self.regs.get_bc().wrapping_add(1)),
            WordRegister::DE => self.regs.set_de(self.regs.get_de().wrapping_add(1)),
            WordRegister::HL => self.regs.set_hl(self.regs.get_hl().wrapping_add(1)),
            WordRegister::SP => self.sp = self.sp.wrapping_add(1),
        };
    }

    pub(super) fn dec16(&mut self, target: WordRegister) {
        let value = match target {
            WordRegister::BC => self.regs.set_bc(self.regs.get_bc().wrapping_sub(1)),
            WordRegister::DE => self.regs.set_de(self.regs.get_de().wrapping_sub(1)),
            WordRegister::HL => self.regs.set_hl(self.regs.get_hl().wrapping_sub(1)),
            WordRegister::SP => self.sp = self.sp.wrapping_sub(1),
        };
    }

    pub(super) fn bit(&mut self, target:BitTarget, source: BitSource) {
        let i = get_position_by_bittarget(target);
        let value = self.get_bitsource_val(source);
        let bit_value = get_bit_val(i, value);

        self.regs.flags.zero = !bit_value;
    }

    fn get_bitsource_val(&self, source: BitSource) -> u8 {
        match source {
            BitSource::A => self.regs.a,
            BitSource::B => self.regs.b,
            BitSource::C => self.regs.c,
            BitSource::D => self.regs.d,
            BitSource::E => self.regs.e,
            BitSource::H => self.regs.h,
            BitSource::L => self.regs.l,
            BitSource::HLI => self.mmu.read_byte(self.regs.get_hl()),
        }
    }

    fn get_arithmetic_target_val(&self, target: &ArithmeticTarget) -> u8 {
        match target {
            ArithmeticTarget::A     => self.regs.a,
            ArithmeticTarget::B     => self.regs.b,
            ArithmeticTarget::C     => self.regs.c,
            ArithmeticTarget::D     => self.regs.d,
            ArithmeticTarget::E     => self.regs.e,
            ArithmeticTarget::H     => self.regs.h,
            ArithmeticTarget::L     => self.regs.l,
            ArithmeticTarget::HLI   => self.mmu.read_byte(self.regs.get_hl()),
            ArithmeticTarget::D8    => self.read_next_byte()
        }
    }

    pub(super) fn rla(&mut self) {
        self.regs.flags.zero = false;
        self.regs.flags.subtract = false;
        self.regs.flags.half_carry = false;

        self.shift_left_register(&IncDecTarget::A);
    }

    pub(super) fn rl(&mut self, target: IncDecTarget) {
        self.regs.flags.subtract = false;
        self.regs.flags.half_carry = false;

        self.shift_left_register(&target);

        match target {
            IncDecTarget::A => { self.regs.flags.zero = self.regs.a == 0; },
            IncDecTarget::B => { self.regs.flags.zero = self.regs.b == 0; },
            IncDecTarget::C => { self.regs.flags.zero = self.regs.c == 0; },
            IncDecTarget::D => { self.regs.flags.zero = self.regs.d == 0; },
            IncDecTarget::E => { self.regs.flags.zero = self.regs.e == 0; },
            IncDecTarget::H => { self.regs.flags.zero = self.regs.h == 0; },
            IncDecTarget::L => { self.regs.flags.zero = self.regs.l == 0; },
            IncDecTarget::HLI => { self.regs.flags.zero = self.mmu.read_byte(self.regs.get_hl()) == 0; }
        };
        
    }

    fn shift_left_register(&mut self, target: &IncDecTarget) {
        let old_carry = self.regs.flags.carry;

        match target {
            IncDecTarget::A => self.regs.flags.carry = get_bit_val(7,self.regs.a),
            IncDecTarget::B => self.regs.flags.carry = get_bit_val(7,self.regs.b),
            IncDecTarget::C => self.regs.flags.carry = get_bit_val(7,self.regs.c),
            IncDecTarget::D => self.regs.flags.carry = get_bit_val(7,self.regs.d),
            IncDecTarget::E => self.regs.flags.carry = get_bit_val(7,self.regs.e),
            IncDecTarget::H   => self.regs.flags.carry = get_bit_val(7,self.regs.h),
            IncDecTarget::L   => self.regs.flags.carry = get_bit_val(7,self.regs.l),
            IncDecTarget::HLI => self.regs.flags.carry = get_bit_val(7,self.mmu.read_byte(self.regs.get_hl()))
        };

        match target {
            IncDecTarget::A => { self.regs.a = (self.regs.a << 1) + old_carry as u8; },
            IncDecTarget::B => { self.regs.b = (self.regs.b << 1) + old_carry as u8; },
            IncDecTarget::C => { self.regs.c = (self.regs.c << 1) + old_carry as u8; },
            IncDecTarget::D => { self.regs.d = (self.regs.d << 1) + old_carry as u8; },
            IncDecTarget::E => { self.regs.e = (self.regs.e << 1) + old_carry as u8; },
            IncDecTarget::H => { self.regs.h = (self.regs.h << 1) + old_carry as u8; },
            IncDecTarget::L => { self.regs.l = (self.regs.l << 1) + old_carry as u8; },
            IncDecTarget::HLI => {
                let new_val = (self.mmu.read_byte(self.regs.get_hl()) << 1) + old_carry as u8;
                self.mmu.write_byte(self.regs.get_hl(), new_val);
            }
        };
    }
}

fn get_position_by_bittarget(target:BitTarget) -> u8 {
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

fn get_bit_val(position:u8, value:u8) -> bool {
    let mask = 1 << position;
    (mask & value) > 0
}