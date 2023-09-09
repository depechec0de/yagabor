use crate::{gameboy::GameBoy, Button};

#[derive(Debug)]
pub(crate) struct Joypad {
    // 0x20 => arrow selector (bit 5 of 0xFF00)
    // 0x10 => buttons selector (bit 4 of 0xFF00)
    register: u8,
    state: JoypadState,
}

// true if a button is pressed 
#[derive(Debug)]
pub(crate) struct JoypadState{ 
    a: bool, 
    b: bool, 
    start: bool, 
    select: bool, 
    up: bool, 
    down: bool, 
    left: bool, 
    right: bool
}

impl Joypad {
    pub(crate) fn new() -> Self {
        Joypad { 
            register: 0x0,
            state: JoypadState { 
                        a: false, 
                        b: false, 
                        start: false, 
                        select: false, 
                        up: false, 
                        down: false, 
                        left: false, 
                        right: false 
                    }
        }
    }

    // Depending on selector we return a set of button states as u8
    // http://imrannazar.com/GameBoy-Emulation-in-JavaScript:-Input
    pub(crate) fn read(gb: &GameBoy) -> u8 {
        let jp = &gb.io.joypad;
        let selector = jp.register & 0x30;
        let mut result = selector | 0b1100_0000;
        // Buttons states are negated because 0 is interpreted as pressed and 1 as released
        if selector & 0x20 != 0 {
            let buttons: u8 =   ((!jp.state.down as u8) << 3) + 
                                ((!jp.state.up as u8) << 2) + 
                                ((!jp.state.left as u8) << 1) + 
                                ((!jp.state.right as u8));
            result |= buttons & 0x0F;      
        }else if selector & 0x10 != 0 {
            let buttons: u8 =   ((!jp.state.start as u8) << 3) + 
                                ((!jp.state.select as u8) << 2) + 
                                ((!jp.state.b as u8) << 1) + 
                                ((!jp.state.a as u8));
            result |= buttons & 0x0F;
        }else if selector == 0 {
            result |= 0x0F;
        }

        result
    }

    pub(crate) fn write(gb: &mut GameBoy, value: u8) {
        gb.io.joypad.register = 0b1100_1111 | (value & 0x30);
    }

    pub(crate) fn button_pressed(gb: &mut GameBoy, b: Button) {
        match b {
            Button::A => gb.io.joypad.state.a = true,
            Button::B => gb.io.joypad.state.b = true,
            Button::Start => gb.io.joypad.state.start = true,
            Button::Select => gb.io.joypad.state.select = true,
            Button::Up => { gb.io.joypad.state.up = true },
            Button::Down => gb.io.joypad.state.down = true,
            Button::Left => gb.io.joypad.state.left = true,
            Button::Right => gb.io.joypad.state.right = true,
        }
        //println!("button_pressed {:08b}", Joypad::read(gb));
    }  

    pub(crate) fn button_released(gb: &mut GameBoy, b: Button) {
        match b {
            Button::A => gb.io.joypad.state.a = false,
            Button::B => gb.io.joypad.state.b = false,
            Button::Start => gb.io.joypad.state.start = false,
            Button::Select => gb.io.joypad.state.select = false,
            Button::Up => gb.io.joypad.state.up = false,
            Button::Down => gb.io.joypad.state.down = false,
            Button::Left => gb.io.joypad.state.left = false,
            Button::Right => gb.io.joypad.state.right = false,
        }
        //println!("button_released {:08b}", Joypad::read(gb));
    }      
}