use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    mins: i32
}

const DAY:i32 = 60 * 24;

fn normalize(minutes: i32) -> i32 {
    match minutes {
        neg if neg < 0 => (neg % DAY) + DAY,
        ovrf if ovrf >= DAY => ovrf % DAY,
        id => id
    }
}

impl Clock {

    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            mins: normalize((hours * 60) + minutes)
        }
    }

    pub fn add_minutes(mut self, minutes: i32) -> Self {
        self.mins = normalize(self.mins + minutes);
        self
    }

    fn hours(&self) -> i32 {
        (self.mins - self.minutes()) / 60
    }

    fn minutes(&self) -> i32 {
        self.mins % 60
    }

}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours(), self.minutes())
    }
}
