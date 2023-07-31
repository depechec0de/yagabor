#[cfg(test)]
use crate::cpu::CPU;
use crate::rom::ROM;
use crate::cpu::instructions::Instruction::*;
use crate::cpu::instructions::ArithmeticTarget::*;

#[test]
fn add_without_carry() {
    let boot = ROM::empty();
    let mut cpu = CPU::new(boot);

    cpu.regs.a = 0b00000001;
    cpu.regs.b = 0b00000001;

    cpu.execute(ADD(B));

    assert_eq!(cpu.regs.a, 0b00000010);
    assert_eq!(cpu.regs.flags.subtract, false);
    assert_eq!(cpu.regs.flags.carry, false);
    assert_eq!(cpu.regs.flags.zero, false);
    assert_eq!(cpu.regs.flags.half_carry, false);
}

#[test]
fn add_with_half_carry() {
    let boot = ROM::empty();
    let mut cpu = CPU::new(boot);

    cpu.regs.a = 0b00001111;
    cpu.regs.b = 0b00000001;

    cpu.execute(ADD(B));

    assert_eq!(cpu.regs.a, 0b00010000);
    assert_eq!(cpu.regs.flags.subtract, false);
    assert_eq!(cpu.regs.flags.carry, false);
    assert_eq!(cpu.regs.flags.zero, false);
    assert_eq!(cpu.regs.flags.half_carry, true);
}
#[test]
fn add_with_carry() {
    let boot = ROM::empty();
    let mut cpu = CPU::new(boot);

    cpu.regs.a = 0b11111111;
    cpu.regs.b = 0b1;

    cpu.execute(ADD(B));

    assert_eq!(cpu.regs.a, 0b0);
    assert_eq!(cpu.regs.flags.subtract, false);
    assert_eq!(cpu.regs.flags.zero, true);
    assert_eq!(cpu.regs.flags.carry, true);
    assert_eq!(cpu.regs.flags.half_carry, true);
}

#[test]
fn adc_with_carry() {
    let boot = ROM::empty();
    let mut cpu = CPU::new(boot);

    cpu.regs.a = 0b11111110;
    cpu.regs.b = 0b1;
    cpu.regs.flags.carry = true;

    cpu.execute(ADC(B));

    assert_eq!(cpu.regs.a, 0b0);
    assert_eq!(cpu.regs.flags.subtract, false);
    assert_eq!(cpu.regs.flags.zero, true);
    assert_eq!(cpu.regs.flags.carry, true);
    assert_eq!(cpu.regs.flags.half_carry, true);
}

#[test]
fn adc_with_half_carry() {
    let boot = ROM::empty();
    let mut cpu = CPU::new(boot);

    cpu.regs.a = 0b00001110;
    cpu.regs.b = 0b00000001;
    cpu.regs.flags.carry = true;

    cpu.execute(ADC(B));

    assert_eq!(cpu.regs.a, 0b00010000);
    assert_eq!(cpu.regs.flags.subtract, false);
    assert_eq!(cpu.regs.flags.zero, false);
    assert_eq!(cpu.regs.flags.carry, false);
    assert_eq!(cpu.regs.flags.half_carry, true);
}

#[test]
fn sub_with_carry() {
    let boot = ROM::empty();
    let mut cpu = CPU::new(boot);

    cpu.regs.a = 0b00001111;
    cpu.regs.b = 0b10000000;

    cpu.execute(SUB(B));

    assert_eq!(cpu.regs.a, 0b10001111);
    assert_eq!(cpu.regs.flags.subtract, true);
    assert_eq!(cpu.regs.flags.zero, false);
    assert_eq!(cpu.regs.flags.carry, true);
    assert_eq!(cpu.regs.flags.half_carry, false);
}

#[test]
fn sub_with_half_carry() {
    let boot = ROM::empty();
    let mut cpu = CPU::new(boot);

    cpu.regs.a = 0x1;
    cpu.regs.b = 0xF;

    cpu.execute(SUB(B));
    println!("asasasa {:x?}", cpu.regs.a);
    assert_eq!(cpu.regs.a, 0xF2);
    assert_eq!(cpu.regs.flags.subtract, true);
    assert_eq!(cpu.regs.flags.zero, false);
    assert_eq!(cpu.regs.flags.carry, true);
    assert_eq!(cpu.regs.flags.half_carry, true);
}

#[test]
fn exec_boot_room() {
    let boot = ROM::dmg();
    let mut cpu = CPU::new(boot);

    //cpu.step();
}