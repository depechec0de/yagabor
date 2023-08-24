use crate::gameboy::{mmu::Address};

use super::io::INTERRUPT_FLAG_ADDRESS;

const VBLANK_INT_HANDLER: Address = 0x0040;
const LCD_INT_HANDLER: Address = 0x0048;
const TIMER_INT_HANDLER: Address = 0x0050;
const SERIAL_INT_HANDLER: Address = 0x0058;
const JOYPAD_INT_HANDLER: Address = 0x0060;


pub(crate) struct InterruptsRegister {
    pub(crate) interrupt_enable: u8,
    pub(crate) interrupt_flag: u8,    
}

impl InterruptsRegister {
    pub(crate) fn new() -> Self {
        Self { interrupt_enable: 0x0, interrupt_flag: 0x0 }
    }

    pub(crate) fn write_enable(&mut self, value: u8) {
        self.interrupt_enable = value;
    }

    pub(crate) fn write_flag(&mut self, value: u8) { 
        self.interrupt_flag = value;
    } 

    pub(crate) fn read_enable(&self) -> u8 {
        self.interrupt_enable
    }

    pub(crate) fn read_flag(&self) -> u8 {
        self.interrupt_flag
    }

    // We respect the interruptions priorities
    pub(crate) fn interrupt_to_handle(&mut self) -> Option<Interruption> {
        if self.some_interrupt_enabled() {
            if self.is_vblank() {
                self.turnoff(Interruption::VBlank);
                Some(Interruption::VBlank)
            } else if self.is_lcd() {
                self.turnoff(Interruption::LCDStat);
                Some(Interruption::LCDStat)
            } else if self.is_timer() {
                self.turnoff(Interruption::Timer);
                Some(Interruption::Timer)
            } else if self.is_serial() {
                self.turnoff(Interruption::Serial);
                Some(Interruption::Serial)
            } else if self.is_joypad() {
                self.turnoff(Interruption::Joypad);
                Some(Interruption::Joypad)
            }else{
                None
            }
        }else {
            None
        }
    }

    pub(crate) fn some_interrupt_enabled(&self) -> bool {
        (self.interrupt_enable & self.interrupt_flag) != 0
    }

    fn is_vblank(&self) -> bool {
        let bit_mask = 0b00000001;
        (self.interrupt_enable & bit_mask) > 0 && (self.interrupt_flag & bit_mask) > 0
    }

    fn is_lcd(&self) -> bool {
        let bit_mask = 0b00000010;
        (self.interrupt_enable & bit_mask) > 0 && (self.interrupt_flag & bit_mask) > 0
    }

    fn is_timer(&self) -> bool {
        let bit_mask = 0b00000100;
        (self.interrupt_enable & bit_mask) > 0 && (self.interrupt_flag & bit_mask) > 0
    }

    fn is_serial(&self) -> bool {
        let bit_mask = 0b00001000;
        (self.interrupt_enable & bit_mask) > 0 && (self.interrupt_flag & bit_mask) > 0
    }

    fn is_joypad(&self) -> bool {
        let bit_mask = 0b00010000;
        (self.interrupt_enable & bit_mask) > 0 && (self.interrupt_flag & bit_mask) > 0
    }

    pub(crate) fn turnoff(&mut self, interruption: Interruption) {
        match interruption {
            Interruption::VBlank => { self.interrupt_flag = self.interrupt_flag &   0b11111110; },
            Interruption::LCDStat => { self.interrupt_flag = self.interrupt_flag &  0b11111101; },
            Interruption::Timer => { self.interrupt_flag = self.interrupt_flag &    0b11111011; },
            Interruption::Serial => { self.interrupt_flag = self.interrupt_flag &   0b11110111; },            
            Interruption::Joypad => { self.interrupt_flag = self.interrupt_flag &   0b11101111; },
        };
    }

    pub(crate) fn turnon(&mut self, interruption: Interruption) {
        match interruption {
            Interruption::VBlank => { self.interrupt_flag = self.interrupt_flag |   0b00000001; },
            Interruption::LCDStat => { self.interrupt_flag = self.interrupt_flag |  0b00000010; },
            Interruption::Timer => { self.interrupt_flag = self.interrupt_flag |    0b00000100; },
            Interruption::Serial => { self.interrupt_flag = self.interrupt_flag |   0b00001000; },            
            Interruption::Joypad => { self.interrupt_flag = self.interrupt_flag |   0b00010000; },
        };
    }
    
}

#[derive(Debug)]
pub(crate) enum Interruption {
    VBlank,
    LCDStat,
    Timer,
    Serial,
    Joypad
}

impl Interruption {
    pub(crate) fn handler(&self) -> Address {
        match self {
            Interruption::VBlank => VBLANK_INT_HANDLER,
            Interruption::LCDStat => LCD_INT_HANDLER,
            Interruption::Timer => TIMER_INT_HANDLER,
            Interruption::Serial => SERIAL_INT_HANDLER,
            Interruption::Joypad => JOYPAD_INT_HANDLER,
        }
    }
}