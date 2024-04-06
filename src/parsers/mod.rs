use std::ops::RangeInclusive;

use chrono::{Local, NaiveDateTime};

// -- Custom Parsers
#[derive(Clone, Debug)]
pub struct LineRange(RangeInclusive<u16>);

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

// -- Validators
pub fn validate_description(description: &str) -> Result<&str, ()> {
    let len = description.len();

    if len <= 1 || len >= 50 {
        Err(())
    } else {
        Ok(description)
    }
}

pub fn validate_group(group: &Option<String>) -> Result<Option<String>, ()> {
    match &group {
        Some(g) if g.len() >= 16 || g.len() <= 1 => return Err(()),
        _ => Ok(group.clone()),
    }
}

pub fn validate_due(due: &Option<NaiveDateTime>) -> Result<Option<NaiveDateTime>, ()> {
    match &due {
        Some(date) if *date <= Local::now().naive_utc() => return Err(()),
        _ => Ok(*due),
    }
}

pub fn validate_username(username: &str) -> Result<(), ()> {
    let len = username.len();
    if len >= 30 || len <= 1 {
        Err(())
    } else {
        Ok(())
    }
}

pub fn validate_table_name(name: &str) -> Result<(), ()> {
    let len = name.len();
    if len >= 15 || len <= 1 {
        Err(())
    } else {
        Ok(())
    }
}
