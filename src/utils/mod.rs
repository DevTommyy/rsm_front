pub mod config_helper;
pub mod table_formatter;

use std::{cmp::min, fs, io, path::PathBuf};

use crate::parsers::LineRange;

pub enum Choice {
    Yes,
    No,
}

impl Choice {
    pub fn from_input(input: &str) -> Option<Self> {
        match input.trim().to_lowercase().as_str() {
            "yes" | "y" => Some(Self::Yes),
            "no" | "n" => Some(Self::No),
            _ => None,
        }
    }
}

/// prompts the user asking if he has already a key, and retrives his choice
pub fn get_user_choice() -> io::Result<Choice> {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if let Some(choice) = Choice::from_input(&input) {
            break Ok(choice);
        } else {
            println!("Invalid input. Please enter 'yes' ('y') or 'no' ('n').");
        }
    }
}

pub fn resolve_file_input(
    file: &PathBuf,
    line: Option<&u16>,        // Changed type to usize for line number
    range: Option<&LineRange>, // Changed type to tuple of (start, end) line numbers
) -> io::Result<String> {
    if !file.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
    }
    let content = fs::read_to_string(&file)?;
    if content.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "File is empty"));
    }

    let lines: Vec<&str> = content.lines().collect();
    let result = match (line, range) {
        (Some(line_num), _) => lines
            .get(*line_num as usize - 1)
            .map_or("", |&line| line)
            .trim()
            .to_owned(),
        (_, Some(range)) => {
            let range = range.clone();
            let end = *range.0.end() as usize;

            let start_index = *range.0.start() as usize - 1;
            let end_index = min(end, lines.len());

            lines[start_index..end_index].join(" ").trim().to_owned()
        }
        _ => content.trim().to_owned(),
    };

    if result.len() > 256 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Data too long"));
    }

    Ok(result)
}
