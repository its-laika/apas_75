use std::time::{SystemTime, UNIX_EPOCH};

/// Returns a fake "random" number based on the current system time.
///
/// # Arguments
///
/// * `lower` - Lower bound of "random" range. **Inclusive!**
/// * `upper` - Upper bound of "random" range. +*Exclusive!**
///
/// # Returns
///
/// A fake "random" number in given bounds
pub fn get_fake(lower: u32, upper: u32) -> Option<u32> {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()?
        .subsec_millis();

    let fake_random_number = lower + (millis % (upper - lower));
    Some(fake_random_number)
}
