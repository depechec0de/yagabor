use core::fmt;

use pretty_hex::*;

use crate::gameboy::{mmu::{Address, IO_SIZE, IO_BEGIN, IO_END, MMU}, cpu::cpu::ClockCycles, gameboy::GameBoy};

use super::{interrupts::{Interruption, Interrupts}, lcd::LCD, timers::Timers};

pub(crate) const JOYPAD_INPUT_ADDRESS: Address = 0xFF00;
pub(crate) const SERIAL_DATA_ADDRESS: Address = 0xFF01;
pub(crate) const SERIAL_CONTROL_ADDRESS: Address = 0xFF02;

pub(crate) const DIV_ADDRESS: Address = 0xFF04;
pub(crate) const TIMA_ADDRESS: Address = 0xFF05;
pub(crate) const TMA_ADDRESS: Address = 0xFF06;
pub(crate) const TAC_ADDRESS: Address = 0xFF07;

pub(crate) const LCD_BEGIN: Address = 0xFF40;
pub(crate) const LCD_END: Address = 0xFF4B;

pub(crate) const BOOT_SWITCH_ADDRESS: Address = 0xFF50;

pub(crate) const INTERRUPT_FLAG_ADDRESS: Address = 0xFF0F;


pub(crate) struct IO {
    pub(crate) interrupts: Interrupts,
    pub(crate) lcd: LCD,
    pub(crate) timers: Timers,
    data: [u8; IO_SIZE],
}

impl IO {
    pub(crate) fn new() -> Self {
        Self { 
             interrupts: Interrupts::new(),
             lcd: LCD::new(),
             timers: Timers::new(),
             data:[0; IO_SIZE] 
        }
    }

    pub(crate) fn read_byte(gb: &GameBoy, address: Address) -> u8 {
        match address {
            LCD_BEGIN ..= LCD_END => LCD::read_byte(gb, address),
            INTERRUPT_FLAG_ADDRESS => Interrupts::read_flag(gb),
            // DIV value is 8 upper bits
            DIV_ADDRESS => IO::get_div_register(gb),
            _ => gb.io.data[(address - IO_BEGIN) as usize]
        }
    }

    pub(crate) fn write_byte(gb: &mut GameBoy, address: Address, value: u8) {
        match address {
            DIV_ADDRESS => {
                // Writing DIV reset it
                gb.io.data[(DIV_ADDRESS - IO_BEGIN) as usize] = 0;
            },
            LCD_BEGIN ..= LCD_END => LCD::write_byte(gb, address, value),
            BOOT_SWITCH_ADDRESS => {
                gb.io.data[(address - IO_BEGIN) as usize] = value;
                MMU::set_boot_mapping(gb, value);
            },
            INTERRUPT_FLAG_ADDRESS => {
                Interrupts::write_flag(gb, value);
            },
            _ => {
                gb.io.data[(address - IO_BEGIN) as usize] = value;
            }
        }
    }

    pub(crate) fn serial_control_clear(gb: &mut GameBoy) {
        // Turn off bit 7
        gb.io.data[(SERIAL_CONTROL_ADDRESS - IO_BEGIN) as usize] = gb.io.data[(SERIAL_CONTROL_ADDRESS - IO_BEGIN) as usize] & 0b01111111;
    }

    pub(crate) fn get_tac_register(gb: &GameBoy) -> u8 {
        gb.io.data[(TAC_ADDRESS - IO_BEGIN) as usize]
    }

    pub(crate) fn get_div_register(gb: &GameBoy) -> u8 {
        gb.io.data[(DIV_ADDRESS - IO_BEGIN) as usize]
    }

    pub(crate) fn get_tma_register(gb: &GameBoy) -> u8 {
        gb.io.data[(TMA_ADDRESS - IO_BEGIN) as usize]
    }

    pub(crate) fn inc_div(gb: &mut GameBoy) {
        let div = gb.io.data[(DIV_ADDRESS - IO_BEGIN) as usize];
        gb.io.data[(DIV_ADDRESS - IO_BEGIN) as usize] = div.wrapping_add(1);
    }

    pub(crate) fn inc_tima(gb: &mut GameBoy) -> bool {
        let tima = gb.io.data[(TIMA_ADDRESS - IO_BEGIN) as usize];
        let (new_tima, overflow) = tima.overflowing_add(1);
        gb.io.data[(TIMA_ADDRESS - IO_BEGIN) as usize] = new_tima;
        overflow
    }

    pub(crate) fn reset_tima(gb: &mut GameBoy) {
        let tma: u8 = IO::get_tma_register(gb);
        gb.io.data[(TIMA_ADDRESS - IO_BEGIN) as usize] = tma;
    }

    pub(crate) fn ack_sent_serial(gb: &mut GameBoy){
        Interrupts::turnon(gb, Interruption::Serial);
        IO::serial_control_clear(gb);
    }    
}

#[derive(Debug)]
pub(crate) enum SerialTransferMode {
    NoTransfer,
    TransferExternalClock,
    TransferInternalClock
}

impl SerialTransferMode {
    pub(crate) fn parse_from_byte(byte: u8) -> SerialTransferMode {
        if byte == 0x81 {
            SerialTransferMode::TransferInternalClock
        }else if byte == 0x80 {
            SerialTransferMode::TransferExternalClock
        }else{
            SerialTransferMode::NoTransfer
        }
    }
}