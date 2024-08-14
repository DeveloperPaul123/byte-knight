use stopwatch::Stopwatch;

pub struct Timer {
    sw: Stopwatch,
    starting_millis_remaining: i64
}

impl Timer {
    pub fn new(millis_remaining: i64) -> Timer {
        Timer {
            sw: Stopwatch::start_new(),
            starting_millis_remaining: millis_remaining
        }
    }

    pub fn milliseconds_remaining(self: &Self) -> i64 {
        let elapsed = self.sw.elapsed_ms();
        return (0 as i64).max(self.starting_millis_remaining - elapsed);
    }

    pub fn milliseconds_elapsed_this_turn(self: &Self) -> i64 {
        return self.sw.elapsed_ms();
    }
}

impl ToString for Timer {
    fn to_string(&self) -> String {
        let millis_remaining = self.milliseconds_remaining();
        let seconds_remaining = millis_remaining / 1000;
        let minutes_remaining = seconds_remaining / 60;
        let seconds_remaining = seconds_remaining % 60;
        return format!("{:02}:{:02}, {:04} elapsed", minutes_remaining, seconds_remaining, 
            self.milliseconds_elapsed_this_turn());
    }
}
