use std::path::PathBuf;

use crate::emulation::Emulation;
use crate::gameboy::GameBoy;
use crate::gameboy::cartridge;
use crate::gameboy::cpu::*;
use crate::gameboy::cpu::Instruction::*;
use crate::gameboy::cpu::ArithmeticTarget::*;
use crate::gameboy::{cpu::mmu::MMU, rom::ROM, cartridge::Cartridge};

#[test]
fn add_without_carry() {
    let mut mmu = MMU::new(ROM::empty(), Cartridge::empty());
    let mut cpu = CPU::new(mmu);

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
    let mut mmu = MMU::new(ROM::empty(), Cartridge::empty());
    let mut cpu = CPU::new(mmu);

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
    let mut mmu = MMU::new(ROM::empty(), Cartridge::empty());
    let mut cpu = CPU::new(mmu);

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
    let mut mmu = MMU::new(ROM::empty(), Cartridge::empty());
    let mut cpu = CPU::new(mmu);

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
    let mut mmu = MMU::new(ROM::empty(), Cartridge::empty());
    let mut cpu = CPU::new(mmu);

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
    let mut mmu = MMU::new(ROM::empty(), Cartridge::empty());
    let mut cpu = CPU::new(mmu);

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
    let mut mmu = MMU::new(ROM::empty(), Cartridge::empty());
    let mut cpu = CPU::new(mmu);

    cpu.regs.a = 0x1;
    cpu.regs.b = 0xF;

    cpu.execute(SUB(B));

    assert_eq!(cpu.regs.a, 0xF2);
    assert_eq!(cpu.regs.flags.subtract, true);
    assert_eq!(cpu.regs.flags.zero, false);
    assert_eq!(cpu.regs.flags.carry, true);
    assert_eq!(cpu.regs.flags.half_carry, true);
}

#[test]
fn sbc_with_carry() {
    let mut mmu = MMU::new(ROM::empty(), Cartridge::empty());
    let mut cpu = CPU::new(mmu);

    cpu.regs.a = 0b00001111;
    cpu.regs.b = 0b01111111;

    cpu.regs.flags.carry = true;

    cpu.execute(SBC(B));

    assert_eq!(cpu.regs.a, 0b10001111);
    assert_eq!(cpu.regs.flags.subtract, true);
    assert_eq!(cpu.regs.flags.zero, false);
    assert_eq!(cpu.regs.flags.carry, true);
    assert_eq!(cpu.regs.flags.half_carry, false);
}

#[test]
fn get_af() {
    let mut mmu = MMU::new(ROM::empty(), Cartridge::empty());
    let mut cpu = CPU::new(mmu);

    cpu.regs.a = 0b01010101;
    cpu.regs.flags.zero = false;
    cpu.regs.flags.subtract = true;
    cpu.regs.flags.half_carry = false;
    cpu.regs.flags.carry = true;
    
    assert_eq!(cpu.regs.get_af(), 0b0101010101010000);
}

#[test]
fn set_af() {
    let mut mmu = MMU::new(ROM::empty(), Cartridge::empty());
    let mut cpu = CPU::new(mmu);

    cpu.regs.set_af(0b0101010101010000);

    assert_eq!(cpu.regs.a, 0b01010101);
    assert_eq!(cpu.regs.flags.subtract, true);
    assert_eq!(cpu.regs.flags.zero, false);
    assert_eq!(cpu.regs.flags.carry, true);
    assert_eq!(cpu.regs.flags.half_carry, false);
}

#[test]
fn stack_push() {
    let mut mmu = MMU::new(ROM::empty(), Cartridge::empty());
    let mut cpu = CPU::new(mmu);

    let init_sp = 0xDFFF;
    cpu.sp = init_sp;

    let low:u8 = 0b01010000;
    let high:u8 = 0b01010101;
    let test_value: u16 = ((high as u16) << 8) + low as u16;

    cpu.regs.set_bc(test_value);

    cpu.push(crate::gameboy::cpu::instructions::StackTarget::BC);

    assert_eq!(cpu.sp, init_sp-2);
    assert_eq!(cpu.mmu.read_byte(init_sp-1), high);
    assert_eq!(cpu.mmu.read_byte(init_sp-2), low);
}

#[test]
fn stack_push_pop() {
    let mut mmu = MMU::new(ROM::empty(), Cartridge::empty());
    let mut cpu = CPU::new(mmu);
    cpu.sp = 0xDFFF;

    let test_value: u16 = 0b0101010101010000;

    cpu.regs.set_bc(test_value);

    cpu.push(crate::gameboy::cpu::instructions::StackTarget::BC);
    cpu.pop(crate::gameboy::cpu::instructions::StackTarget::HL);

    assert_eq!(cpu.regs.get_hl(), cpu.regs.get_bc());
}

/* #[test]
fn cpu_instrs() {
    let cartridge = Cartridge::test_cpu_instrs();
    let mut gb: GameBoy = GameBoy::new(cartridge);

    let mut emu = Emulation::new(gb);

    emu.run();
} */

