use self::{screen::Screen, keypad::Joypad, cartridge::Cartridge, rom::ROM, cpu::{mmu::MMU, CPU}};

pub mod cartridge;
mod ppu;
mod rom;
mod keypad;
mod cpu;
mod screen;

pub struct GameBoy {
    cpu: CPU,
    //ppu: Rc<PPU>,
    screen: Screen,
    joypad: Joypad
}

impl GameBoy {
    pub fn new(cartridge: Cartridge) -> GameBoy {
        let bootrom = ROM::dmg();
        //let mut ppu = Rc::new(PPU::new());
        let mut mmu = MMU::new(bootrom, cartridge);
        let mut cpu = CPU::new(mmu);
        let screen = Screen::new();
        let joypad = Joypad::new();

        GameBoy { 
                cpu,
                //ppu, 
                screen,
                joypad
        }
    }

    pub fn step(&mut self) {
        self.cpu.step();
    }
}