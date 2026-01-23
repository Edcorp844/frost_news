use std::{cmp::Ordering, collections::HashMap};

use chrono::{DateTime, Datelike, Local, Utc};

pub struct UITimeOrganiser {}

impl UITimeOrganiser {
    pub fn new() -> UITimeOrganiser {
        UITimeOrganiser {}
    }

    // -----------------------------
    // Date parsing
    // -----------------------------
    pub fn parse_datetime(&self, datetime_str: Option<String>) -> DateTime<Utc> {
        match datetime_str {
            Some(s) => DateTime::parse_from_rfc3339(&s)
                .or_else(|_| DateTime::parse_from_rfc2822(&s))
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            None => Utc::now(),
        }
    }

    // -----------------------------
    // Bucket key: start of day (UTC)
    // -----------------------------
    pub fn time_bucket_key(&self, published_at: &str) -> DateTime<Utc> {
        let dt = self.parse_datetime(Some(published_at.to_string()));
        dt.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc()
    }

    // -----------------------------
    // Human-readable labels
    // -----------------------------
    pub fn categorize_by_relative_time(&self, bucket: DateTime<Utc>) -> String {
        let now = Utc::now();
        let local_bucket = bucket.with_timezone(&Local);

        let days_diff = (now.date_naive() - bucket.date_naive()).num_days();
        let month = bucket.date_naive().month();

        if days_diff == 0 {
            "Today".to_string()
        } else if days_diff == 1 {
            "Yesterday".to_string()
        } else if days_diff < 7 {
            match local_bucket.weekday() {
                chrono::Weekday::Mon => "This Monday",
                chrono::Weekday::Tue => "This Tuesday",
                chrono::Weekday::Wed => "This Wednesday",
                chrono::Weekday::Thu => "This Thursday",
                chrono::Weekday::Fri => "This Friday",
                chrono::Weekday::Sat => "This Saturday",
                chrono::Weekday::Sun => "This Sunday",
            }
            .to_string()
        } else if days_diff < 14 {
            "Last week".to_string()
        } else if days_diff < 30 {
            "Earlier this month".to_string()
        } else if days_diff < 365 && month > 1 {
            "Earlier this year".to_string()
        } else {
            "Last year".to_string()
        }
    }

    pub fn comapre(a: String, b: String) -> Ordering {
        let ordered = HashMap::from([
            ("Today", 1),
            ("Yesterday", 2),
            ("This Monday", 3),
            ("This Tuesday", 4),
            ("This Wednesday", 5),
            ("This Thursday", 6),
            ("This Friday", 7),
            ("This Saturday", 8),
            ("This Sunday", 9),
            ("Last week", 10),
            ("Earlier this month", 11),
            ("Earlier this year", 12),
            ("Last year", 13),
        ]);
        let ordering = ordered
            .get(a.as_str())
            .unwrap()
            .cmp(ordered.get(b.as_str()).unwrap());

        ordering
    }
}
