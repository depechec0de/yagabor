use std::io::Error;

use super::cartridge::Cartridge;
use super::cpu::cpu::{CPU, ClockCycles};
use super::cpu::mmu::MMU;
use super::serial::SerialControl;

pub(crate) struct GameBoy {
    cpu: CPU,
}

impl GameBoy {
    pub(crate) fn new(cartridge: Cartridge) -> Self {
        let mmu = MMU::new(cartridge);
        let cpu = CPU::new(mmu);

        GameBoy { cpu }
    }

    pub(crate) fn tick(&mut self) -> Result<ClockCycles, Error> {
        let cpu_result = self.cpu.step();

        if self.cpu.interrupts_enabled() {

        }
    
        cpu_result
    }

    pub(crate) fn joypad_down(&mut self) {
        
    }

    // https://gbdev.io/pandocs/Serial_Data_Transfer_(Link_Cable).html
    pub(crate) fn serial_control(&mut self) -> SerialControl {
        if self.cpu.read_serial_control() == 0x81 {
            SerialControl::TransferStartInternal
        } else if self.cpu.read_serial_control() == 0x80 {
            SerialControl::TransferStartExternal
        }else {
            SerialControl::Undefined
        }
    }

    pub(crate) fn serial_data(&mut self) -> u8 {
        self.cpu.read_serial_data()
    }
    
}

