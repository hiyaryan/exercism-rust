pub fn production_rate_per_hour(speed: u8) -> f64 {
    let prod = (speed as f64) * 221_f64;

    // Instead of making a `prod * multiplier` call in each branch,
    // isolate only the differing behavior of each branch.
    //
    // Also, matching on a range implies the `contains` check.
    let multiplier = match speed {
        1..=4 => 1.00,
        5..=8 => 0.90,
        9..=10 => 0.77,

        // Return early to distinct this is an "exception", even though
        // letting the multiplier be `0.00` would have the same effect.
        _ => return 0.00,
    };

    prod * multiplier
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let rate_per_minute = production_rate_per_hour(speed) / 60.0;

    // defer casting until as late as possible to retain precision
    rate_per_minute as u32
}
