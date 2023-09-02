use chrono::{DateTime, Utc};

pub fn parse_date(date_str: &str) -> eyre::Result<i64> {
    let dt = DateTime::parse_from_str(date_str, "%a, %d %b %Y %H:%M:%S %z")?;
    Ok(dt.timestamp())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_date_1() {
        let date_str = "Fri, 01 Sep 2023 02:19:13 +0000";
        let timestamp = parse_date(date_str).unwrap();
        assert_eq!(timestamp, 1693534753);
    }

    #[test]
    fn test_parse_date_2() {
        let date_str = "Wed, 31 Aug 2022 13:30:00 +0000";
        let timestamp = parse_date(date_str).unwrap();
        assert_eq!(timestamp, 1661952600);
    }

    #[test]
    fn test_parse_date_3() {
        let date_str = "Tue, 29 Nov 2022 00:00:01 +0000";
        let timestamp = parse_date(date_str).unwrap();
        assert_eq!(timestamp, 1669680001);
    }

    #[test]
    fn test_parse_date_4() {
        let date_str = "Thu, 31 Dec 2020 23:59:59 +0000";
        let timestamp = parse_date(date_str).unwrap();
        assert_eq!(timestamp, 1609459199);
    }

    #[test]
    fn test_parse_date_5() {
        let date_str = "Sun, 01 Jan 2023 00:00:00 +0000";
        let timestamp = parse_date(date_str).unwrap();
        assert_eq!(timestamp, 1672531200);
    }
}
