use crate::gameboy::{cartridge::Cartridge, cpu::{cpu::CPU, instructions::{instructions::{Instruction}, decode::{InstructionType, RegistersIndDir, StackTarget, RegistersIndirect}}}, gameboy::GameBoy, serial::SerialOutput, mmu::MMU, io::{io::{IO, IOEvent}, interrupts::Interruption}};

#[test]
fn add_without_carry() {
    let mut cpu = CPU::new(MMU::new(Cartridge::empty(), IO::new()));

    cpu.pc = 0x100;
    cpu.regs.a = 0b00000001;    
    cpu.regs.b = 0b00000001;

    // ADD A, B
    let inst = Instruction::new(InstructionType::ADD(RegistersIndDir::B), Some(0x0));

    let _ = inst.execute(&mut cpu);

    assert_eq!(cpu.regs.a, 0b00000010);
    assert_eq!(cpu.regs.flags.subtract, false);
    assert_eq!(cpu.regs.flags.carry, false);
    assert_eq!(cpu.regs.flags.zero, false);
    assert_eq!(cpu.regs.flags.half_carry, false);
}

#[test]
fn add_with_half_carry() {
    let mut cpu = CPU::new(MMU::new(Cartridge::empty(), IO::new()));

    cpu.pc = 0x100;
    cpu.regs.a = 0b00001111;
    cpu.regs.b = 0b00000001;

    // ADD A, B
    let inst = Instruction::new(InstructionType::ADD(RegistersIndDir::B), None);

    let _ = inst.execute(&mut cpu);

    assert_eq!(cpu.regs.a, 0b00010000);
    assert_eq!(cpu.regs.flags.subtract, false);
    assert_eq!(cpu.regs.flags.carry, false);
    assert_eq!(cpu.regs.flags.zero, false);
    assert_eq!(cpu.regs.flags.half_carry, true);
}
#[test]
fn add_with_carry() {
    let mut cpu = CPU::new(MMU::new(Cartridge::empty(), IO::new()));

    cpu.pc = 0x100;
    cpu.regs.a = 0b11111111;
    cpu.regs.b = 0b1;

    // ADD A, B
    let inst = Instruction::new(InstructionType::ADD(RegistersIndDir::B), None);

    let _ = inst.execute(&mut cpu);

    assert_eq!(cpu.regs.a, 0b0);
    assert_eq!(cpu.regs.flags.subtract, false);
    assert_eq!(cpu.regs.flags.zero, true);
    assert_eq!(cpu.regs.flags.carry, true);
    assert_eq!(cpu.regs.flags.half_carry, true);
}

#[test]
fn adc_with_carry() {
    let mut cpu = CPU::new(MMU::new(Cartridge::empty(), IO::new()));

    cpu.pc = 0x100;
    cpu.regs.a = 0b11111110;
    cpu.regs.b = 0b1;
    cpu.regs.flags.carry = true;

    // ADC A, B
    let inst = Instruction::new(InstructionType::ADC(RegistersIndDir::B), None);

    let _ = inst.execute(&mut cpu);

    assert_eq!(cpu.regs.a, 0b0);
    assert_eq!(cpu.regs.flags.subtract, false);
    assert_eq!(cpu.regs.flags.zero, true);
    assert_eq!(cpu.regs.flags.carry, true);
    assert_eq!(cpu.regs.flags.half_carry, true);
}

#[test]
fn adc_with_half_carry() {
    let mut cpu = CPU::new(MMU::new(Cartridge::empty(), IO::new()));

    cpu.pc = 0x100;
    cpu.regs.a = 0b00001110;
    cpu.regs.b = 0b00000001;
    cpu.regs.flags.carry = true;

    // ADC A, B
    let inst = Instruction::new(InstructionType::ADC(RegistersIndDir::B), None);

    let _ = inst.execute(&mut cpu);

    assert_eq!(cpu.regs.a, 0b00010000);
    assert_eq!(cpu.regs.flags.subtract, false);
    assert_eq!(cpu.regs.flags.zero, false);
    assert_eq!(cpu.regs.flags.carry, false);
    assert_eq!(cpu.regs.flags.half_carry, true);
}

#[test]
fn sub_with_carry() {
    let mut cpu = CPU::new(MMU::new(Cartridge::empty(), IO::new()));

    cpu.pc = 0x100;
    cpu.regs.a = 0b00001111;
    cpu.regs.b = 0b10000000;

    // SUB B
    let inst = Instruction::new(InstructionType::SUB(RegistersIndDir::B), None);

    let _ = inst.execute(&mut cpu);

    assert_eq!(cpu.regs.a, 0b10001111);
    assert_eq!(cpu.regs.flags.subtract, true);
    assert_eq!(cpu.regs.flags.zero, false);
    assert_eq!(cpu.regs.flags.carry, true);
    assert_eq!(cpu.regs.flags.half_carry, false);
}

#[test]
fn sub_with_half_carry() {
    let mut cpu = CPU::new(MMU::new(Cartridge::empty(), IO::new()));

    cpu.pc = 0x100;
    cpu.regs.a = 0x1;
    cpu.regs.b = 0xF;

    // SUB B
    let inst = Instruction::new(InstructionType::SUB(RegistersIndDir::B), None);

    let _ = inst.execute(&mut cpu);

    assert_eq!(cpu.regs.a, 0xF2);
    assert_eq!(cpu.regs.flags.subtract, true);
    assert_eq!(cpu.regs.flags.zero, false);
    assert_eq!(cpu.regs.flags.carry, true);
    assert_eq!(cpu.regs.flags.half_carry, true);
}

#[test]
fn sbc_with_carry() {
    let mut cpu = CPU::new(MMU::new(Cartridge::empty(), IO::new()));

    cpu.pc = 0x100;
    cpu.regs.a = 0b00001111;
    cpu.regs.b = 0b01111111;

    cpu.regs.flags.carry = true;

    // SBC B
    let inst = Instruction::new(InstructionType::SBC(RegistersIndDir::B), None);

    let _ = inst.execute(&mut cpu);

    assert_eq!(cpu.regs.a, 0b10001111);
    assert_eq!(cpu.regs.flags.subtract, true);
    assert_eq!(cpu.regs.flags.zero, false);
    assert_eq!(cpu.regs.flags.carry, true);
    assert_eq!(cpu.regs.flags.half_carry, true);
}

#[test]
fn sbc_with_half_carry() {
    let mut cpu = CPU::new(MMU::new(Cartridge::empty(), IO::new()));

    cpu.pc = 0x100;
    cpu.regs.a = 0x0;

    cpu.regs.flags.carry = true;

    let inst = Instruction::new(InstructionType::SBC(RegistersIndDir::B), None);

    let _ = inst.execute(&mut cpu);

    assert_eq!(cpu.regs.a, 0xFF);
    assert_eq!(cpu.regs.flags.subtract, true);
    assert_eq!(cpu.regs.flags.zero, false);
    assert_eq!(cpu.regs.flags.carry, true);
    assert_eq!(cpu.regs.flags.half_carry, true);
}

#[test]
fn get_af() {
    let mut cpu = CPU::new(MMU::new(Cartridge::empty(), IO::new()));

    cpu.pc = 0x100;

    cpu.regs.a = 0b01010101;
    cpu.regs.flags.zero = false;
    cpu.regs.flags.subtract = true;
    cpu.regs.flags.half_carry = false;
    cpu.regs.flags.carry = true;
    
    assert_eq!(cpu.regs.get_af(), 0b0101010101010000);
}

#[test]
fn set_af() {
    let mut cpu = CPU::new(MMU::new(Cartridge::empty(), IO::new()));

    cpu.pc = 0x100;
    cpu.regs.set_af(0b0101010101010000);

    assert_eq!(cpu.regs.a, 0b01010101);
    assert_eq!(cpu.regs.flags.subtract, true);
    assert_eq!(cpu.regs.flags.zero, false);
    assert_eq!(cpu.regs.flags.carry, true);
    assert_eq!(cpu.regs.flags.half_carry, false);
}

#[test]
fn stack_push() {
    let mut cpu = CPU::new(MMU::new(Cartridge::empty(), IO::new()));

    cpu.pc = 0x100;
    let init_sp = 0xDFFF;
    cpu.sp = init_sp;

    let low:u8 = 0b01010000;
    let high:u8 = 0b01010101;
    let test_value: u16 = ((high as u16) << 8) + low as u16;

    cpu.regs.set_bc(test_value);

    let inst = Instruction::new(InstructionType::PUSH(StackTarget::BC), None);
    let _ = inst.execute(&mut cpu);

    assert_eq!(cpu.sp, init_sp-2);
    assert_eq!(cpu.mmu.read_byte(init_sp-1), high);
    assert_eq!(cpu.mmu.read_byte(init_sp-2), low);
}

#[test]
fn stack_push_pop() {
    let mut cpu = CPU::new(MMU::new(Cartridge::empty(), IO::new()));
    cpu.sp = 0xDFFF;
    cpu.pc = 0x100;

    let test_value: u16 = 0b0101010101010000;

    cpu.regs.set_bc(test_value);

    let inst = Instruction::new(InstructionType::PUSH(StackTarget::BC), None);
    let _ = inst.execute(&mut cpu);
    let inst = Instruction::new(InstructionType::POP(StackTarget::HL), None);
    let _ = inst.execute(&mut cpu);

    assert_eq!(cpu.regs.get_hl(), cpu.regs.get_bc());
}

#[test]
fn rla() {
    let mut cpu = CPU::new(MMU::new(Cartridge::empty(), IO::new()));

    cpu.pc = 0x100;

    cpu.regs.a = 0b10000000;

    cpu.regs.flags.carry = true;

    let inst = Instruction::new(InstructionType::RLA, None);

    let _ = inst.execute(&mut cpu);

    assert_eq!(cpu.regs.a, 0b00000001);
    assert_eq!(cpu.regs.flags.subtract, false);
    assert_eq!(cpu.regs.flags.zero, false);
    assert_eq!(cpu.regs.flags.carry, true);
    assert_eq!(cpu.regs.flags.half_carry, false);

    // No carry to move to bit 0
    cpu.regs.a = 0b10000000;

    cpu.regs.flags.carry = false;

    let _ = inst.execute(&mut cpu);

    assert_eq!(cpu.regs.a, 0b00000000);
    assert_eq!(cpu.regs.flags.subtract, false);
    assert_eq!(cpu.regs.flags.zero, false);
    assert_eq!(cpu.regs.flags.carry, true);
    assert_eq!(cpu.regs.flags.half_carry, false);
}

#[test]
fn rlca() {
    let mut cpu = CPU::new(MMU::new(Cartridge::empty(), IO::new()));

    cpu.pc = 0x100;

    cpu.regs.a = 0b10000000;

    cpu.regs.flags.carry = false;

    let inst = Instruction::new(InstructionType::RLCA, None);

    let _ = inst.execute(&mut cpu);

    assert_eq!(cpu.regs.a, 0b00000001);
    assert_eq!(cpu.regs.flags.subtract, false);
    assert_eq!(cpu.regs.flags.zero, false);
    assert_eq!(cpu.regs.flags.carry, true);
    assert_eq!(cpu.regs.flags.half_carry, false);

    // No bit 7 to move
    cpu.regs.a = 0b01000001;

    cpu.regs.flags.carry = false;

    let _ = inst.execute(&mut cpu);

    assert_eq!(cpu.regs.a, 0b10000010);
    assert_eq!(cpu.regs.flags.subtract, false);
    assert_eq!(cpu.regs.flags.zero, false);
    assert_eq!(cpu.regs.flags.carry, false);
    assert_eq!(cpu.regs.flags.half_carry, false);
}

#[test]
fn srl() {
    let mut cpu = CPU::new(MMU::new(Cartridge::empty(), IO::new()));

    cpu.pc = 0x100;

    cpu.regs.b = 0xFF;

    cpu.regs.flags.zero = false;
    cpu.regs.flags.carry = false;
    cpu.regs.flags.half_carry = false;
    cpu.regs.flags.subtract = false;

    let inst = Instruction::new(InstructionType::SRL(RegistersIndirect::B), None);

    let _ = inst.execute(&mut cpu);

    assert_eq!(cpu.regs.b, 0x7F);
    assert_eq!(cpu.regs.flags.subtract, false);
    assert_eq!(cpu.regs.flags.zero, false);
    assert_eq!(cpu.regs.flags.carry, true);
    assert_eq!(cpu.regs.flags.half_carry, false);

    // Zero flag
    cpu.regs.b = 0x01;

    cpu.regs.flags.zero = false;
    cpu.regs.flags.carry = false;
    cpu.regs.flags.half_carry = false;
    cpu.regs.flags.subtract = false;

    let _ = inst.execute(&mut cpu);

    assert_eq!(cpu.regs.b, 0x00);
    assert_eq!(cpu.regs.flags.subtract, false);
    assert_eq!(cpu.regs.flags.zero, true);
    assert_eq!(cpu.regs.flags.carry, true);
    assert_eq!(cpu.regs.flags.half_carry, false);

}

#[test]
fn timers() {
    let mut cpu = CPU::new(MMU::new(Cartridge::empty(), IO::new()));

    cpu.ime = true;
    cpu.mmu.io.interrupts.turnon(Interruption::Timer);
    cpu.timers.tma = 0;
    cpu.timers.tac = 0b00000111; // timer enabled and frecuency 256 clocks

    let mut timer_fired = cpu.timer_tick(256);

    assert_eq!(cpu.timers.div, 1);
    assert_eq!(cpu.timers.tima, 1);
    //assert_eq!(timer_fired, false);

    for i in 1..=255 {
        //assert_eq!(timer_fired, false);
        assert_eq!(cpu.timers.tima, i);
        timer_fired = cpu.timer_tick(256);
    }

    assert_eq!(cpu.timers.tima, 0);
    //assert_eq!(timer_fired, true);
}

fn assert_serial_result(gb: &mut GameBoy, result: &mut Vec<char>) {
    let mut serial_buffer = Vec::<char>::new();
    loop {
        let execresult = gb.tick();
        if let Ok(execresult) = execresult {
            if let Some(event) = execresult.event {
                match event {
                    IOEvent::SerialOutput(byte) => {
                        serial_buffer.push(byte as char);   

                        let result_str = serial_buffer.iter().cloned().collect::<String>();
                        println!("{}", result_str);
                        if result_str.contains("Passed") {
                            println!("{}", result_str);
                            assert!(true);
                            break
                        }else if result_str.contains("Failed") {
                            println!("{}", result_str);
                            assert!(false);
                            break
                        }
                    },
                    _ => {}
                }   
                
            }
        }
    }
}

#[test]
fn cpu_instrs_01() {
    let cartridge = Cartridge::cpu_instrs_01();

    let mut gb: GameBoy = GameBoy::new(cartridge, None);
    let mut result = Vec::<char>::new();
    
    assert_serial_result(&mut gb, &mut result);
}

#[test]
fn cpu_instrs_02() {
    let cartridge = Cartridge::cpu_instrs_02();

    let mut gb: GameBoy = GameBoy::new(cartridge, None);
    let mut result = Vec::<char>::new();
    
    assert_serial_result(&mut gb, &mut result);
}

#[test]
fn cpu_instrs_03() {
    let cartridge = Cartridge::cpu_instrs_03();

    let mut gb: GameBoy = GameBoy::new(cartridge, None);
    let mut result = Vec::<char>::new();
    
    assert_serial_result(&mut gb, &mut result);
}

#[test]
fn cpu_instrs_04() {
    let cartridge = Cartridge::cpu_instrs_04();

    let mut gb: GameBoy = GameBoy::new(cartridge, None);
    let mut result = Vec::<char>::new();
    
    assert_serial_result(&mut gb, &mut result);
}

#[test]
fn cpu_instrs_05() {
    let cartridge = Cartridge::cpu_instrs_05();

    let mut gb: GameBoy = GameBoy::new(cartridge, None);
    let mut result = Vec::<char>::new();
    
    assert_serial_result(&mut gb, &mut result);
}

#[test]
fn cpu_instrs_06() {
    let cartridge = Cartridge::cpu_instrs_06();

    let mut gb: GameBoy = GameBoy::new(cartridge, None);
    let mut result = Vec::<char>::new();
    
    assert_serial_result(&mut gb, &mut result);
}

#[test]
fn cpu_instrs_07() {
    let cartridge = Cartridge::cpu_instrs_07();

    let mut gb: GameBoy = GameBoy::new(cartridge, None);
    let mut result = Vec::<char>::new();
    
    assert_serial_result(&mut gb, &mut result);
}

#[test]
fn cpu_instrs_08() {
    let cartridge = Cartridge::cpu_instrs_08();

    let mut gb: GameBoy = GameBoy::new(cartridge, None);
    let mut result = Vec::<char>::new();
    
    assert_serial_result(&mut gb, &mut result);
}

#[test]
fn cpu_instrs_09() {
    let cartridge = Cartridge::cpu_instrs_09();

    let mut gb: GameBoy = GameBoy::new(cartridge, None);
    let mut result = Vec::<char>::new();
    
    assert_serial_result(&mut gb, &mut result);
}

#[test]
fn cpu_instrs_10() {
    let cartridge = Cartridge::cpu_instrs_10();

    let mut gb: GameBoy = GameBoy::new(cartridge, None);
    let mut result = Vec::<char>::new();
    
    assert_serial_result(&mut gb, &mut result);
}

#[test]
fn cpu_instrs_11() {
    let cartridge = Cartridge::cpu_instrs_11();

    let mut gb: GameBoy = GameBoy::new(cartridge, None);
    let mut result = Vec::<char>::new();
    
    assert_serial_result(&mut gb, &mut result);
}

// #[test]
// fn halt_bug() {
//     let cartridge = Cartridge::halt_bug();

//     let mut gb: GameBoy = GameBoy::new(cartridge, None);
//     let mut result = Vec::<char>::new();
    
//     assert_serial_result(&mut gb, &mut result);
// }