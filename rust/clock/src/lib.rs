use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    const MINUTES_IN_DAY: i32 = 1440;

    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            minutes: ((hours * 60) + minutes).rem_euclid(Clock::MINUTES_IN_DAY) // non-negative modulo
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }
}
