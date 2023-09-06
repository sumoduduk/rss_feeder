use chrono::{DateTime, FixedOffset, Timelike, Utc};

static TIME_ARRAY : [u32] = [10, 12, 15, 17, 18, 19, 21];

pub fn check_schedule(now_hour: u32) -> bool {
    let at_interval = now_hour >= 10 && now_hour < 24;
    at_interval
}

pub fn to_central_time(now: &DateTime<Utc>) -> Option<DateTime<FixedOffset>> {
    let us_offset = FixedOffset::west_opt(6 * 3600);

    if let Some(central_time) = us_offset {
        let time_us = now.with_timezone(&central_time);
        Some(time_us)
    } else {
        None
    }
}

pub fn return_task_hour(current: u32) -> u32{

    let task_hour = for time in TIME_ARRAY {
        
    }
}
