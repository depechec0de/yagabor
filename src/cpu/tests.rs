#[cfg(test)]
use crate::cpu::CPU;
use crate::rom::ROM;
use crate::cpu::instructions::Instruction::*;
use crate::cpu::instructions::ArithmeticTarget::*;

#[test]
fn add8_without_carry() {
    let boot = ROM::empty();
    let mut cpu = CPU::new(boot);

    cpu.regs.a = 0b00000001;
    cpu.regs.b = 0b00000001;

    cpu.execute(ADD(B));

    assert_eq!(cpu.regs.a, 0b00000010);
    assert_eq!(cpu.regs.f.subtract, false);
    assert_eq!(cpu.regs.f.carry, false);
    assert_eq!(cpu.regs.f.zero, false);
    assert_eq!(cpu.regs.f.half_carry, false);
}

#[test]
fn add8_with_half_carry() {
    let boot = ROM::empty();
    let mut cpu = CPU::new(boot);

    cpu.regs.a = 0b00001111;
    cpu.regs.b = 0b00000001;

    cpu.execute(ADD(B));

    assert_eq!(cpu.regs.a, 0b00010000);
    assert_eq!(cpu.regs.f.subtract, false);
    assert_eq!(cpu.regs.f.carry, false);
    assert_eq!(cpu.regs.f.zero, false);
    assert_eq!(cpu.regs.f.half_carry, true);
}
#[test]
fn add8_with_carry() {
    let boot = ROM::empty();
    let mut cpu = CPU::new(boot);

    cpu.regs.a = 0b11111111;
    cpu.regs.b = 0b1;

    cpu.execute(ADD(B));

    assert_eq!(cpu.regs.a, 0b0);
    assert_eq!(cpu.regs.f.subtract, false);
    assert_eq!(cpu.regs.f.zero, true);
    assert_eq!(cpu.regs.f.carry, true);
    assert_eq!(cpu.regs.f.half_carry, true);
}

#[test]
fn exec_boot_room() {
    let boot = ROM::dmg();
    let mut cpu = CPU::new(boot);

    cpu.step();
}