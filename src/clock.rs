use std::time::Instant;

pub struct Clock {
    ms_update_interval: u32,
    last_update: Instant,
}

impl Clock {
    pub fn new(ms_update_interval_in: u32) -> Self {
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
        return self.last_update.elapsed().as_millis() as u32 >= self.ms_update_interval;
    }


    pub fn set_update_interval(&mut self, interval: u32){
        self.ms_update_interval = interval;
    }


    pub fn get_update_interval(&self) -> u32 {
        return self.ms_update_interval; 
    }

    pub fn get_update_interval_ref_mut(&mut self) -> &mut u32 {
    
        return &mut self.ms_update_interval;
    }
}
