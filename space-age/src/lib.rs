// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        let Duration(seconds) = d;
        *seconds as f64 / Self::year_in_seconds()
    }

    fn year_in_seconds() -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

const EARTH_YEAR_SECS:f64 = 31_557_600.0;

impl Planet for Mercury {
    fn year_in_seconds() -> f64 {
        EARTH_YEAR_SECS * 0.2408467
    }
}

impl Planet for Venus {
    fn year_in_seconds() -> f64 {
        EARTH_YEAR_SECS * 0.61519726
    }
}

impl Planet for Earth {
    fn year_in_seconds() -> f64 {
        EARTH_YEAR_SECS
    }
}

impl Planet for Mars {
    fn year_in_seconds() -> f64 {
        EARTH_YEAR_SECS * 1.8808158
    }
}

impl Planet for Jupiter {
    fn year_in_seconds() -> f64 {
        EARTH_YEAR_SECS * 11.862615
    }
}

impl Planet for Saturn {
    fn year_in_seconds() -> f64 {
        EARTH_YEAR_SECS * 29.447498
    }
}

impl Planet for Uranus {
    fn year_in_seconds() -> f64 {
        EARTH_YEAR_SECS * 84.016846
    }
}

impl Planet for Neptune {
    fn year_in_seconds() -> f64 {
        EARTH_YEAR_SECS * 164.79132
    }
}
