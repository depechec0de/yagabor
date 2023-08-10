use std::time::Duration;

pub(crate) struct Clock {
    herz: usize
}

impl Clock {
    pub(crate) fn new(herz: usize) -> Clock {
        Clock { herz }
    }

    pub(crate) fn wait_tick(&self, cycles_passed: u64) {
        //let time_passed: f32 = (cycles_passed as usize as f32) / self.herz as f32;
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
