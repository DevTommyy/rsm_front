use std::ops::RangeInclusive;

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
