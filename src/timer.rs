use std::time::{Duration, Instant};

pub struct Timer {
    endtime: Instant,
}

impl Timer {
    pub fn new(duration_milli: u64) -> Timer {
        Timer {
            endtime: Instant::now() + Duration::from_millis(duration_milli),
        }
    }

    pub fn elapsed(&self) -> bool {
        Instant::now() >= self.endtime
    }

    pub fn remaining(&self) -> u128 {
        self.endtime.duration_since(Instant::now()).as_millis()
    }
}
