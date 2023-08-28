use crate::gameboy::{mmu::Address, gameboy::GameBoy};

const VBLANK_INT_HANDLER: Address = 0x0040;
const LCD_INT_HANDLER: Address = 0x0048;
const TIMER_INT_HANDLER: Address = 0x0050;
const SERIAL_INT_HANDLER: Address = 0x0058;
const JOYPAD_INT_HANDLER: Address = 0x0060;


pub(crate) struct Interrupts {
    pub(crate) interrupt_enable: u8,
    pub(crate) interrupt_flag: u8,    
}

impl Interrupts {
    pub(crate) fn new() -> Self {
        Self { interrupt_enable: 0x0, interrupt_flag: 0xe0 }
    }

    pub(crate) fn write_enable(gb: &mut GameBoy, value: u8) {
        gb.io.interrupts.interrupt_enable = value;
    }

    pub(crate) fn write_flag(gb: &mut GameBoy, value: u8) { 
        gb.io.interrupts.interrupt_flag = value;
    } 

    pub(crate) fn read_enable(gb: &GameBoy) -> u8 {
        gb.io.interrupts.interrupt_enable
    }

    pub(crate) fn read_flag(gb: &GameBoy) -> u8 {
        gb.io.interrupts.interrupt_flag
    }

    // We respect the interruptions priorities
    pub(crate) fn interrupt_to_handle(gb: &mut GameBoy) -> Option<Interruption> {
        if Interrupts::some_interrupt_enabled(gb) {
            if Interrupts::is_vblank(gb) {
                Interrupts::turnoff(gb, Interruption::VBlank);
                Some(Interruption::VBlank)
            } else if Interrupts::is_lcd(gb) {
                Interrupts::turnoff(gb, Interruption::LCDStat);
                Some(Interruption::LCDStat)
            } else if Interrupts::is_timer(gb) {
                Interrupts::turnoff(gb, Interruption::Timer);
                Some(Interruption::Timer)
            } else if Interrupts::is_serial(gb) {
                Interrupts::turnoff(gb, Interruption::Serial);
                Some(Interruption::Serial)
            } else if Interrupts::is_joypad(gb) {
                Interrupts::turnoff(gb, Interruption::Joypad);
                Some(Interruption::Joypad)
            }else{
                None
            }
        }else {
            None
        }
    }

    pub(crate) fn some_interrupt_enabled(gb: &GameBoy) -> bool {
        (gb.io.interrupts.interrupt_enable & gb.io.interrupts.interrupt_flag) != 0
    }

    fn is_vblank(gb: &GameBoy) -> bool {
        let bit_mask = 0b00000001;
        (gb.io.interrupts.interrupt_enable & bit_mask) > 0 && (gb.io.interrupts.interrupt_flag & bit_mask) > 0
    }

    fn is_lcd(gb: &GameBoy) -> bool {
        let bit_mask = 0b00000010;
        (gb.io.interrupts.interrupt_enable & bit_mask) > 0 && (gb.io.interrupts.interrupt_flag & bit_mask) > 0
    }

    fn is_timer(gb: &GameBoy) -> bool {
        let bit_mask = 0b00000100;
        (gb.io.interrupts.interrupt_enable & bit_mask) > 0 && (gb.io.interrupts.interrupt_flag & bit_mask) > 0
    }

    fn is_serial(gb: &GameBoy) -> bool {
        let bit_mask = 0b00001000;
        (gb.io.interrupts.interrupt_enable & bit_mask) > 0 && (gb.io.interrupts.interrupt_flag & bit_mask) > 0
    }

    fn is_joypad(gb: &GameBoy) -> bool {
        let bit_mask = 0b00010000;
        (gb.io.interrupts.interrupt_enable & bit_mask) > 0 && (gb.io.interrupts.interrupt_flag & bit_mask) > 0
    }

    pub(crate) fn turnoff(gb: &mut GameBoy, interruption: Interruption) {
        match interruption {
            Interruption::VBlank => { gb.io.interrupts.interrupt_flag = gb.io.interrupts.interrupt_flag &   0b11111110; },
            Interruption::LCDStat => { gb.io.interrupts.interrupt_flag = gb.io.interrupts.interrupt_flag &  0b11111101; },
            Interruption::Timer => { gb.io.interrupts.interrupt_flag = gb.io.interrupts.interrupt_flag &    0b11111011; },
            Interruption::Serial => { gb.io.interrupts.interrupt_flag = gb.io.interrupts.interrupt_flag &   0b11110111; },            
            Interruption::Joypad => { gb.io.interrupts.interrupt_flag = gb.io.interrupts.interrupt_flag &   0b11101111; },
        };
    }

    pub(crate) fn turnon(gb: &mut GameBoy, interruption: Interruption) {
        match interruption {
            Interruption::VBlank => { gb.io.interrupts.interrupt_flag = gb.io.interrupts.interrupt_flag |   0b00000001; },
            Interruption::LCDStat => { gb.io.interrupts.interrupt_flag = gb.io.interrupts.interrupt_flag |  0b00000010; },
            Interruption::Timer => { gb.io.interrupts.interrupt_flag = gb.io.interrupts.interrupt_flag |    0b00000100; },
            Interruption::Serial => { gb.io.interrupts.interrupt_flag = gb.io.interrupts.interrupt_flag |   0b00001000; },            
            Interruption::Joypad => { gb.io.interrupts.interrupt_flag = gb.io.interrupts.interrupt_flag |   0b00010000; },
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