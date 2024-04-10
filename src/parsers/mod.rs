use std::ops::RangeInclusive;

use chrono::{Duration, Local, NaiveTime};

// -- Custom Parsers
#[derive(Clone, Debug)]
pub struct LineRange(pub RangeInclusive<u16>);

impl std::str::FromStr for LineRange {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("..").collect();

        if parts.len() != 2 {
            return Err("Incorrect input format. Expected format: <start>..<end>");
        }

        let start: u16 = match parts[0].parse() {
            Ok(val) => val,
            Err(_) => return Err("Failed to parse start value"),
        };

        let end: u16 = match parts[1].parse() {
            Ok(val) => val,
            Err(_) => return Err("Failed to parse end value"),
        };

        Ok(LineRange(start..=end))
    }
}

#[derive(Clone, Debug, Default)]
pub struct Due(pub String);
impl std::str::FromStr for Due {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split_whitespace().collect();
        if parts.len() > 2 || parts.len() <= 0 {
            return Err("Invalid date and time format");
        }

        if parts.len() == 1 {
            // only time is provided
            let time_raw = parts.get(0).map_or("", |s| s).trim();
            if time_raw.split(":").collect::<Vec<&str>>().len() != 2 || time_raw.is_empty() {
                return Err("Invalid time");
            }

            let time = NaiveTime::parse_from_str(time_raw, "%H:%M")
                .map_err(|_| Self::Err::from("Invalid time format"))?;

            let now = Local::now().time();
            let today = Local::now().naive_local();

            // if the time is in the past then the date has to be tomorrow
            let date = if time < now {
                (today + Duration::days(1)).date()
            } else {
                today.date()
            }
            .to_string();

            Ok(Due(format!("{date}T{time_raw}:00")))
        } else {
            // date and time are provided
            let date_raw = parts.get(0).map_or("", |s| s).trim();
            if date_raw.split("-").collect::<Vec<&str>>().len() != 3 || date_raw.is_empty() {
                return Err("Invalid date");
            }

            let time_raw = parts.get(1).map_or("", |s| s).trim();
            if time_raw.split(":").collect::<Vec<&str>>().len() != 2 || time_raw.is_empty() {
                return Err("Invalid time");
            }
            Ok(Due(format!("{date_raw}T{time_raw}:00")))
        }
    }
}
