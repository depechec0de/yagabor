#[cfg(test)]
use std::path::PathBuf;

#[cfg(test)]
use crate::gameboy::{cartridge::Cartridge, cpu::instructions::decode::{Instruction, RegistersIndDir, StackTarget, RegistersIndirect}, gameboy::GameBoy, mmu::MMU};

#[test]
fn add_without_carry() {
    let mut gb = GameBoy::new(None);

    gb.cpu.pc = 0x100;
    gb.cpu.regs.a = 0b00000001;    
    gb.cpu.regs.b = 0b00000001;

    // ADD A, B
    let inst = Instruction::ADD(RegistersIndDir::B);

    let _ = inst.execute(&mut gb);

    assert_eq!(gb.cpu.regs.a, 0b00000010);
    assert_eq!(gb.cpu.regs.flags.subtract, false);
    assert_eq!(gb.cpu.regs.flags.carry, false);
    assert_eq!(gb.cpu.regs.flags.zero, false);
    assert_eq!(gb.cpu.regs.flags.half_carry, false);
}

#[test]
fn add_with_half_carry() {
    let mut gb = GameBoy::new(None);

    gb.cpu.pc = 0x100;
    gb.cpu.regs.a = 0b00001111;
    gb.cpu.regs.b = 0b00000001;

    // ADD A, B
    let inst = Instruction::ADD(RegistersIndDir::B);

    let _ = inst.execute(&mut gb);

    assert_eq!(gb.cpu.regs.a, 0b00010000);
    assert_eq!(gb.cpu.regs.flags.subtract, false);
    assert_eq!(gb.cpu.regs.flags.carry, false);
    assert_eq!(gb.cpu.regs.flags.zero, false);
    assert_eq!(gb.cpu.regs.flags.half_carry, true);
}
#[test]
fn add_with_carry() {
    let mut gb = GameBoy::new(None);

    gb.cpu.pc = 0x100;
    gb.cpu.regs.a = 0b11111111;
    gb.cpu.regs.b = 0b1;

    // ADD A, B
    let inst = Instruction::ADD(RegistersIndDir::B);

    let _ = inst.execute(&mut gb);

    assert_eq!(gb.cpu.regs.a, 0b0);
    assert_eq!(gb.cpu.regs.flags.subtract, false);
    assert_eq!(gb.cpu.regs.flags.zero, true);
    assert_eq!(gb.cpu.regs.flags.carry, true);
    assert_eq!(gb.cpu.regs.flags.half_carry, true);
}

#[test]
fn adc_with_carry() {
    let mut gb = GameBoy::new(None);

    gb.cpu.pc = 0x100;
    gb.cpu.regs.a = 0b11111110;
    gb.cpu.regs.b = 0b1;
    gb.cpu.regs.flags.carry = true;

    // ADC A, B
    let inst = Instruction::ADC(RegistersIndDir::B);

    let _ = inst.execute(&mut gb);

    assert_eq!(gb.cpu.regs.a, 0b0);
    assert_eq!(gb.cpu.regs.flags.subtract, false);
    assert_eq!(gb.cpu.regs.flags.zero, true);
    assert_eq!(gb.cpu.regs.flags.carry, true);
    assert_eq!(gb.cpu.regs.flags.half_carry, true);
}

#[test]
fn adc_with_half_carry() {
    let mut gb = GameBoy::new(None);

    gb.cpu.pc = 0x100;
    gb.cpu.regs.a = 0b00001110;
    gb.cpu.regs.b = 0b00000001;
    gb.cpu.regs.flags.carry = true;

    // ADC A, B
    let inst = Instruction::ADC(RegistersIndDir::B);

    let _ = inst.execute(&mut gb);

    assert_eq!(gb.cpu.regs.a, 0b00010000);
    assert_eq!(gb.cpu.regs.flags.subtract, false);
    assert_eq!(gb.cpu.regs.flags.zero, false);
    assert_eq!(gb.cpu.regs.flags.carry, false);
    assert_eq!(gb.cpu.regs.flags.half_carry, true);
}

#[test]
fn sub_with_carry() {
    let mut gb = GameBoy::new(None);

    gb.cpu.pc = 0x100;
    gb.cpu.regs.a = 0b00001111;
    gb.cpu.regs.b = 0b10000000;

    // SUB B
    let inst = Instruction::SUB(RegistersIndDir::B);

    let _ = inst.execute(&mut gb);

    assert_eq!(gb.cpu.regs.a, 0b10001111);
    assert_eq!(gb.cpu.regs.flags.subtract, true);
    assert_eq!(gb.cpu.regs.flags.zero, false);
    assert_eq!(gb.cpu.regs.flags.carry, true);
    assert_eq!(gb.cpu.regs.flags.half_carry, false);
}

#[test]
fn sub_with_half_carry() {
    let mut gb = GameBoy::new(None);

    gb.cpu.pc = 0x100;
    gb.cpu.regs.a = 0x1;
    gb.cpu.regs.b = 0xF;

    // SUB B
    let inst = Instruction::SUB(RegistersIndDir::B);

    let _ = inst.execute(&mut gb);

    assert_eq!(gb.cpu.regs.a, 0xF2);
    assert_eq!(gb.cpu.regs.flags.subtract, true);
    assert_eq!(gb.cpu.regs.flags.zero, false);
    assert_eq!(gb.cpu.regs.flags.carry, true);
    assert_eq!(gb.cpu.regs.flags.half_carry, true);
}

#[test]
fn sbc_with_carry() {
    let mut gb = GameBoy::new(None);

    gb.cpu.pc = 0x100;
    gb.cpu.regs.a = 0b00001111;
    gb.cpu.regs.b = 0b01111111;

    gb.cpu.regs.flags.carry = true;

    // SBC B
    let inst = Instruction::SBC(RegistersIndDir::B);

    let _ = inst.execute(&mut gb);

    assert_eq!(gb.cpu.regs.a, 0b10001111);
    assert_eq!(gb.cpu.regs.flags.subtract, true);
    assert_eq!(gb.cpu.regs.flags.zero, false);
    assert_eq!(gb.cpu.regs.flags.carry, true);
    assert_eq!(gb.cpu.regs.flags.half_carry, true);
}

#[test]
fn sbc_with_half_carry() {
    let mut gb = GameBoy::new(None);

    gb.cpu.pc = 0x100;
    gb.cpu.regs.a = 0x0;

    gb.cpu.regs.flags.carry = true;

    let inst = Instruction::SBC(RegistersIndDir::B);

    let _ = inst.execute(&mut gb);

    assert_eq!(gb.cpu.regs.a, 0xFF);
    assert_eq!(gb.cpu.regs.flags.subtract, true);
    assert_eq!(gb.cpu.regs.flags.zero, false);
    assert_eq!(gb.cpu.regs.flags.carry, true);
    assert_eq!(gb.cpu.regs.flags.half_carry, true);
}

#[test]
fn get_af() {
    let mut gb = GameBoy::new(None);

    gb.cpu.pc = 0x100;

    gb.cpu.regs.a = 0b01010101;
    gb.cpu.regs.flags.zero = false;
    gb.cpu.regs.flags.subtract = true;
    gb.cpu.regs.flags.half_carry = false;
    gb.cpu.regs.flags.carry = true;
    
    assert_eq!(gb.cpu.regs.get_af(), 0b0101010101010000);
}

#[test]
fn set_af() {
    let mut gb = GameBoy::new(None);

    gb.cpu.pc = 0x100;
    gb.cpu.regs.set_af(0b0101010101010000);

    assert_eq!(gb.cpu.regs.a, 0b01010101);
    assert_eq!(gb.cpu.regs.flags.subtract, true);
    assert_eq!(gb.cpu.regs.flags.zero, false);
    assert_eq!(gb.cpu.regs.flags.carry, true);
    assert_eq!(gb.cpu.regs.flags.half_carry, false);
}

#[test]
fn stack_push() {
    let mut gb = GameBoy::new(None);

    gb.cpu.pc = 0x100;
    let init_sp = 0xDFFF;
    gb.cpu.sp = init_sp;

    let low:u8 = 0b01010000;
    let high:u8 = 0b01010101;
    let test_value: u16 = ((high as u16) << 8) + low as u16;

    gb.cpu.regs.set_bc(test_value);

    let inst = Instruction::PUSH(StackTarget::BC);
    let _ = inst.execute(&mut gb);

    assert_eq!(gb.cpu.sp, init_sp-2);
    assert_eq!(MMU::read_byte(&gb, init_sp-1), high);
    assert_eq!(MMU::read_byte(&gb, init_sp-2), low);
}

#[test]
fn stack_push_pop() {
    let mut gb = GameBoy::new(None);
    gb.cpu.sp = 0xDFFF;
    gb.cpu.pc = 0x100;

    let test_value: u16 = 0b0101010101010000;

    gb.cpu.regs.set_bc(test_value);

    let inst = Instruction::PUSH(StackTarget::BC);
    let _ = inst.execute(&mut gb);
    let inst = Instruction::POP(StackTarget::HL);
    let _ = inst.execute(&mut gb);

    assert_eq!(gb.cpu.regs.get_hl(), gb.cpu.regs.get_bc());
}

#[test]
fn rla() {
    let mut gb = GameBoy::new(None);

    gb.cpu.pc = 0x100;

    gb.cpu.regs.a = 0b10000000;

    gb.cpu.regs.flags.carry = true;

    let inst = Instruction::RLA;

    let _ = inst.execute(&mut gb);

    assert_eq!(gb.cpu.regs.a, 0b00000001);
    assert_eq!(gb.cpu.regs.flags.subtract, false);
    assert_eq!(gb.cpu.regs.flags.zero, false);
    assert_eq!(gb.cpu.regs.flags.carry, true);
    assert_eq!(gb.cpu.regs.flags.half_carry, false);

    // No carry to move to bit 0
    gb.cpu.regs.a = 0b10000000;

    gb.cpu.regs.flags.carry = false;

    let _ = inst.execute(&mut gb);

    assert_eq!(gb.cpu.regs.a, 0b00000000);
    assert_eq!(gb.cpu.regs.flags.subtract, false);
    assert_eq!(gb.cpu.regs.flags.zero, false);
    assert_eq!(gb.cpu.regs.flags.carry, true);
    assert_eq!(gb.cpu.regs.flags.half_carry, false);
}

#[test]
fn rlca() {
    let mut gb = GameBoy::new(None);

    gb.cpu.pc = 0x100;

    gb.cpu.regs.a = 0b10000000;

    gb.cpu.regs.flags.carry = false;

    let inst = Instruction::RLCA;

    let _ = inst.execute(&mut gb);

    assert_eq!(gb.cpu.regs.a, 0b00000001);
    assert_eq!(gb.cpu.regs.flags.subtract, false);
    assert_eq!(gb.cpu.regs.flags.zero, false);
    assert_eq!(gb.cpu.regs.flags.carry, true);
    assert_eq!(gb.cpu.regs.flags.half_carry, false);

    // No bit 7 to move
    gb.cpu.regs.a = 0b01000001;

    gb.cpu.regs.flags.carry = false;

    let _ = inst.execute(&mut gb);

    assert_eq!(gb.cpu.regs.a, 0b10000010);
    assert_eq!(gb.cpu.regs.flags.subtract, false);
    assert_eq!(gb.cpu.regs.flags.zero, false);
    assert_eq!(gb.cpu.regs.flags.carry, false);
    assert_eq!(gb.cpu.regs.flags.half_carry, false);
}

#[test]
fn srl() {
    let mut gb = GameBoy::new(None);

    gb.cpu.pc = 0x100;

    gb.cpu.regs.b = 0xFF;

    gb.cpu.regs.flags.zero = false;
    gb.cpu.regs.flags.carry = false;
    gb.cpu.regs.flags.half_carry = false;
    gb.cpu.regs.flags.subtract = false;

    let inst = Instruction::SRL(RegistersIndirect::B);

    let _ = inst.execute(&mut gb);

    assert_eq!(gb.cpu.regs.b, 0x7F);
    assert_eq!(gb.cpu.regs.flags.subtract, false);
    assert_eq!(gb.cpu.regs.flags.zero, false);
    assert_eq!(gb.cpu.regs.flags.carry, true);
    assert_eq!(gb.cpu.regs.flags.half_carry, false);

    // Zero flag
    gb.cpu.regs.b = 0x01;

    gb.cpu.regs.flags.zero = false;
    gb.cpu.regs.flags.carry = false;
    gb.cpu.regs.flags.half_carry = false;
    gb.cpu.regs.flags.subtract = false;

    let _ = inst.execute(&mut gb);

    assert_eq!(gb.cpu.regs.b, 0x00);
    assert_eq!(gb.cpu.regs.flags.subtract, false);
    assert_eq!(gb.cpu.regs.flags.zero, true);
    assert_eq!(gb.cpu.regs.flags.carry, true);
    assert_eq!(gb.cpu.regs.flags.half_carry, false);

}

#[cfg(test)]
fn assert_serial_result(cartridge: Cartridge) {
    let mut gb: GameBoy = GameBoy::new(Some(cartridge));
    let mut serial = Vec::<char>::new();
    loop {
        match gb.tick() {
            Ok(_) => {
                if let Some(data) = gb.read_serial() {   
                    serial.push(data as char);
                    let result_str = serial.iter().cloned().collect::<String>();
                    if result_str.contains("Passed") {
                        println!("{result_str}");
                        assert!(true);
                        break
                    }else if result_str.contains("Failed") {
                        println!("{result_str}");
                        assert!(false);
                        break
                    }       
                }
            },
            _ => assert!(false)
        }        
    }
}

#[test]
fn cpu_instrs_01() {
    let cartridge = Cartridge::new(PathBuf::from("assets/gb-test-roms/cpu_instrs/individual/01-special.gb")).unwrap();    
    assert_serial_result(cartridge);
}

#[test]
fn cpu_instrs_02() {
    let cartridge = Cartridge::new(PathBuf::from("assets/gb-test-roms/cpu_instrs/individual/02-interrupts.gb")).unwrap();
    assert_serial_result(cartridge);
}

#[test]
fn cpu_instrs_03() {
    let cartridge = Cartridge::new(PathBuf::from("assets/gb-test-roms/cpu_instrs/individual/03-op sp,hl.gb")).unwrap();
    assert_serial_result(cartridge);
}

#[test]
fn cpu_instrs_04() {
    let cartridge = Cartridge::new(PathBuf::from("assets/gb-test-roms/cpu_instrs/individual/04-op r,imm.gb")).unwrap();
    assert_serial_result(cartridge);
}

#[test]
fn cpu_instrs_05() {
    let cartridge = Cartridge::new(PathBuf::from("assets/gb-test-roms/cpu_instrs/individual/05-op rp.gb")).unwrap();
    assert_serial_result(cartridge);
}

#[test]
fn cpu_instrs_06() {
    let cartridge = Cartridge::new(PathBuf::from("assets/gb-test-roms/cpu_instrs/individual/06-ld r,r.gb")).unwrap();
    assert_serial_result(cartridge);
}

#[test]
fn cpu_instrs_07() {
    let cartridge = Cartridge::new(PathBuf::from("assets/gb-test-roms/cpu_instrs/individual/07-jr,jp,call,ret,rst.gb")).unwrap();
    assert_serial_result(cartridge);
}

#[test]
fn cpu_instrs_08() {
    let cartridge = Cartridge::new(PathBuf::from("assets/gb-test-roms/cpu_instrs/individual/08-misc instrs.gb")).unwrap();
    assert_serial_result(cartridge);
}

#[test]
fn cpu_instrs_09() {
    let cartridge = Cartridge::new(PathBuf::from("assets/gb-test-roms/cpu_instrs/individual/09-op r,r.gb")).unwrap();
    assert_serial_result(cartridge);
}

#[test]
fn cpu_instrs_10() {
    let cartridge = Cartridge::new(PathBuf::from("assets/gb-test-roms/cpu_instrs/individual/10-bit ops.gb")).unwrap();
    assert_serial_result(cartridge);
}

#[test]
fn cpu_instrs_11() {
    let cartridge = Cartridge::new(PathBuf::from("assets/gb-test-roms/cpu_instrs/individual/11-op a,(hl).gb")).unwrap();
    assert_serial_result(cartridge);
}

// #[test]
// fn halt_bug() {
//     let cartridge = Cartridge::halt_bug();
//     assert_serial_result(cartridge);
// }