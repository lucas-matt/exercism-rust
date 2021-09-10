// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    (221 * speed as u16) as f64 * success_rate(speed)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60
}

fn success_rate(speed: u8) -> f64 {
    match speed {
        s if s >= 9 => 0.77,
        s if s >= 5 => 0.9,
        _ => 1.0
    }
}