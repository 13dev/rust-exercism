use chrono::{DateTime, Utc};
use std::time::{UNIX_EPOCH, Duration};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let seconds = start.timestamp() + 10i64.pow(9);

    DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(seconds as u64))
}
