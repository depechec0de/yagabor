struct Emulation {
    ticks: u64,
    running: bool,
    gb: GameBoy
}

impl Emulation {
    pub fn new(gameboy: GameBoy) -> Emulation {
        Emulation { ticks: 0, running: false, gb: gameboy }
    }

    pub fn run(){
        self.running = true;

        loop {
            self.gameboy.step();
            self.ticks += 1;
        }

    }
}