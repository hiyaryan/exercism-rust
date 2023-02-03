pub fn production_rate_per_hour(speed: u8) -> f64 {
    let prod = (speed as f64) * 221_f64;

    let multiplier = match speed {
        1..=4 => 1.00,
        5..=8 => 0.90,
        9..=10 => 0.77,
        _ => return 0.00,
    };

    prod * multiplier
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let rate_per_minute = production_rate_per_hour(speed) / 60_f64;
    rate_per_minute as u32
}
