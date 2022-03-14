use chrono::{
    format::{DelayedFormat, StrftimeItems},
    prelude::*,
    ParseResult,
};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Time {
    fmt: String,
    time: String,
}

impl Time {
    /// Makes a new `Time` with given format of DateTime and carring current Local DataTime.
    pub fn new() -> Self {
        let fmt = "%Y-%m-%d %H:%M:%S".to_string();
        let now: DateTime<Local> = Local::now();
        let dft: DelayedFormat<StrftimeItems> = now.format(&fmt);

        Time {
            fmt: fmt.clone(),
            time: dft.to_string(),
        }
    }

    /// Transfer string time of `Time` field to milliseconds as String type.
    pub fn from_mills(self, mills: i64) -> Self {
        Time {
            fmt: "%Y-%m-%d %H:%M:%S".to_string(),
            time: Local
                .timestamp_millis(mills)
                .date()
                .format("%Y-%m-%d")
                .to_string()
                + &" "
                + &Local.timestamp_millis(mills).time().to_string(),
        }
    }

    /// Transfer string time of `Time` field to DateTime as String type.
    pub fn from_string(self, date_string: String) -> Self {
        Time {
            fmt: "%Y-%m-%d %H:%M:%S".to_string(),
            time: date_string,
        }
    }

    /// Transfer string time of `Time` field to NaiveDateTime with format `self.fmt`.
    pub fn to_date(&self) -> NaiveDateTime {
        let result: ParseResult<NaiveDateTime> =
            NaiveDateTime::parse_from_str(self.time.as_str(), &self.fmt);
        if result.is_err() {
            result.expect("parse error");
        }
        result.unwrap()
    }

    /// Transfer string time of `Time` field to milliseconds as i64 type.
    pub fn to_mills(&self) -> i64 {
        let result: ParseResult<NaiveDateTime> =
            NaiveDateTime::parse_from_str(self.time.as_str(), &self.fmt);
        if result.is_err() {
            result.expect("parse error");
        }
        let date: NaiveDateTime = result.unwrap();
        Local::from_local_datetime(&chrono::Local, &date)
            .unwrap()
            .timestamp_millis()
    }
}

#[test]
fn check_str_to_date_test() {
    let time = Time::new().from_string("2022-03-14 01:50:37".to_string());
    println!("Grafana Time Date is : --> {}", time.to_date());

    let check_time = Time::new().from_mills(1647193837000);
    println!("Grafana Time Date is : --> {}", check_time.to_date());

    assert_eq!(check_time.to_date(), time.to_date());
}

#[test]
fn check_mills_to_date_test() {
    let time = Time::new().from_string("2022-03-14 01:50:37".to_string());
    println!("Grafana Time Date is : --> {}", &time.time);

    let check_time = Time::new().from_mills(1647193837000);
    println!("Grafana Time Date is : --> {}", &check_time.time);

    assert_eq!(check_time, time);
}
