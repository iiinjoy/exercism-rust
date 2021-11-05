pub fn production_rate_per_hour(speed: u8) -> f64 {
    const PROD_RATE: f64 = 221f64;
    let success_rate: f64 = match speed {
        1..=4 => 1.0,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => 0.0,
    };

    speed as f64 * PROD_RATE * success_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60f64) as u32
}
