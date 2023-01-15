use std::{
    time::{Instant},
};

pub struct Clock {
    ms_update_interval: u128,
    last_update: Instant,
}

impl Clock {
    pub fn new(ms_update_interval_in: u128) -> Self {
        Self {
            ms_update_interval: ms_update_interval_in,
            last_update: Instant::now(),
        }
    }

    pub fn enough_time_passed(&mut self) -> bool {
        if self.timer_finished() {
            self.reset_timer();
            return true;
        } else {
            return false;
        }
    }

    fn reset_timer(&mut self) {
        self.last_update = Instant::now();
    }
    fn timer_finished(&self) -> bool {
        return self.last_update.elapsed().as_millis() >= self.ms_update_interval;
    }
}


