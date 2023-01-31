// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let prod = (speed as f64) * 221_f64;

    if (1..=4).contains(&speed) {
        return prod
    } else if (5..=8).contains(&speed) {
        return prod * 0.90
    } else if (9..=10).contains(&speed) {
        return prod * 0.77
    }

    0.00
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) as u32) / 60
}
