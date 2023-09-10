use actix_web::{HttpResponse, Responder};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use reqwest::{Client, StatusCode};
use serde_json::json;

static URI: &str = "https://notionpush.sumoboker.repl.co/get_porto";

pub fn parse_date(date_str: &str) -> eyre::Result<i64> {
    let dt = DateTime::parse_from_str(date_str, "%a, %d %b %Y %H:%M:%S %z")?;
    Ok(dt.timestamp())
}

pub fn datetime_to_string(datetime: Option<DateTime<Utc>>) -> Option<String> {
    datetime.map(|opt| opt.to_rfc3339())
}

pub fn parse_datetime_timezone(datetime_str: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    return Utc.datetime_from_str(datetime_str, "%Y-%m-%d %H:%M:%S%z");
}

pub fn string_to_datetime(input: &str) -> eyre::Result<DateTime<Utc>> {
    let date = DateTime::parse_from_rfc2822(input)?.with_timezone(&Utc);

    Ok(date)
}

pub async fn reqwst_to_server() -> impl Responder {
    let response = Client::new().get(URI).send().await;
    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                let res = json!({
                "message": "rewst to server",
                "status": "OK"
                });

                HttpResponse::Ok().json(res)
            } else {
                HttpResponse::InternalServerError().json(json!({
                "message": "error from internal"
                }))
            }
        }
        Err(_) => HttpResponse::InternalServerError().json(json!({
        "message": "error from internal"
        })),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use chrono::{DateTime, Utc};

    #[test]
    fn test_parse_rfc2822() {
        let date_str = "Fri, 01 Sep 2023 13:04:06 +0000";
        let expected = DateTime::parse_from_rfc3339("2023-09-01T13:04:06Z").unwrap();
        let actual = DateTime::parse_from_rfc2822(date_str)
            .unwrap()
            .with_timezone(&Utc);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_rfc2822_with_different_timezone() {
        let date_str = "Fri, 01 Sep 2023 15:04:06 +0200";
        let expected = DateTime::parse_from_rfc3339("2023-09-01T13:04:06Z").unwrap();
        let actual = DateTime::parse_from_rfc2822(date_str)
            .unwrap()
            .with_timezone(&Utc);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_rfc2822_with_another_different_timezone() {
        let date_str = "Fri, 01 Sep 2023 08:04:06 -0500";
        let expected = DateTime::parse_from_rfc3339("2023-09-01T13:04:06Z").unwrap();
        let actual = DateTime::parse_from_rfc2822(date_str)
            .unwrap()
            .with_timezone(&Utc);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_invalid_rfc2822() {
        let date_str = "Invalid date string";
        let result = DateTime::parse_from_rfc2822(date_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_empty_string() {
        let date_str = "";
        let result = DateTime::parse_from_rfc2822(date_str);
        assert!(result.is_err());
    }

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
