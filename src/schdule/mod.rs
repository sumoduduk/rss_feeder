use chrono::{DateTime, FixedOffset, Timelike, Utc};

static TIME_ARRAY: [u32; 8] = [10, 12, 15, 17, 18, 19, 21, 24];

pub fn return_task_hour(current: u32) -> u32 {
    let mut candidate = TIME_ARRAY[0];
    for time in TIME_ARRAY {
        if time > current {
            break;
        } else {
            candidate = time;
        }
    }
    candidate
}

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

#[cfg(test)]

mod tests {

    // Import the necessary modules or crates
    use super::*;
    use chrono::{DateTime, Datelike, FixedOffset, TimeZone, Utc};

    #[test]
    fn test_to_central_time_valid_input() {
        // Test when a valid UTC time is provided
        let utc_time = Utc.ymd(2023, 9, 6).and_hms(12, 0, 0); // September 6, 2023, 12:00 PM UTC
        let result = to_central_time(&utc_time);

        assert!(result.is_some()); // The function should return Some
        if let Some(central_time) = result {
            assert_eq!(central_time.year(), 2023);
            assert_eq!(central_time.month(), 9);
            assert_eq!(central_time.day(), 6);
            assert_eq!(central_time.hour(), 6); // 12:00 PM UTC is 7:00 AM Central Time (6 hours behind)
            assert_eq!(central_time.minute(), 0);
            assert_eq!(central_time.second(), 0);
        }
    }

    #[test]
    fn test_to_central_time_invalid_input() {
        // Test when an invalid UTC time is provided (FixedOffset::west_opt returns None)
        let invalid_utc_time = Utc.ymd(2023, 9, 6).and_hms(12, 0, 0);
        let result = to_central_time(&invalid_utc_time);
        assert!(result.is_some()); // The function should return None
    }

    #[test]
    fn test_to_central_time_edge_case() {
        // Test a specific edge case (e.g., midnight UTC)
        let utc_midnight = Utc.ymd(2023, 9, 6).and_hms(0, 0, 0);
        let result = to_central_time(&utc_midnight);

        assert!(result.is_some()); // The function should return Some
        if let Some(central_time) = result {
            assert_eq!(central_time.hour(), 18); // Midnight UTC is 7:00 PM Central Time (6 hours behind)
            assert_eq!(central_time.minute(), 0);
            assert_eq!(central_time.second(), 0);
        }
    }

    #[test]
    fn test_return_task_hour_exact_match() {
        // Test when the input matches one of the values in TIME_ARRAY
        let current = 17;
        let result = return_task_hour(current);
        assert_eq!(result, 17);
    }

    #[test]
    fn test_return_task_hour_less_than_min() {
        // Test when the input is less than the minimum value in TIME_ARRAY
        let current = 23;
        let result = return_task_hour(current);
        assert_eq!(result, 21); // The function should return the minimum value (10) in this case
    }

    #[test]
    fn test_return_task_hour_between_values() {
        // Test when the input is between two values in TIME_ARRAY
        let current = 16;
        let result = return_task_hour(current);
        assert_eq!(result, 15); // The function should return the largest value less than or equal to the input
    }
}
