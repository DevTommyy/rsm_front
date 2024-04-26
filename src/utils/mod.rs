/// # Utils Module
///
/// This module contains utility functions used throughout the application.
///
/// ## Submodules
///
/// - `config_helper`: Provides functionality for working with configuration files.
/// - `table_formatter`: Provides functionality for formatting table data.
///
/// ## Functions
///
/// - `get_user_choice`: Prompts the user to enter a choice ('yes' or 'no').
/// - `resolve_file_input`: Resolves input from a file, optionally extracting a single line or a range of lines.
/// - `find_log_path`: Finds the path to the log file.
///
/// ## Types
///
/// - `Choice`: An enumeration representing a user choice ('Yes' or 'No').
///
/// ## Examples
///
/// ```rust
/// use std::path::PathBuf;
/// use custom_utils::{Choice, get_user_choice, resolve_file_input, find_log_path};
///
/// let choice = get_user_choice().expect("Failed to get user choice");
///
/// let file_path = PathBuf::from("example.txt");
/// let content = resolve_file_input(&file_path, Some(&1), None).expect("Failed to resolve file input");
///
/// let log_path = find_log_path();
/// ```
pub mod config_helper;
pub mod table_formatter;

use std::{cmp::min, env, fs, io, path::PathBuf};

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

pub fn find_log_path() -> String {
    env::var("LOG").unwrap()
}
