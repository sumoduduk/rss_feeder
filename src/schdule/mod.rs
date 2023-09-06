use chrono::{FixedOffset, Timelike, Utc};

pub fn check_schedule() -> bool {
    let us_offset = FixedOffset::west_opt(6 * 3600);
    let now = Utc::now();

    if let Some(central_time) = us_offset {
        let time_us = now.with_timezone(&central_time);
        let now_hour = time_us.hour();
        let at_interval = now_hour >= 10 && now_hour < 24;
        at_interval
    } else {
        false
    }
}
