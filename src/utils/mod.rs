pub mod config_helper;
pub mod table_formatter;

use std::io::{self, Write};

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
pub fn get_user_choice() -> std::io::Result<Choice> {
    loop {
        print!("do you already have a key([yes]/no): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if let Some(choice) = Choice::from_input(&input) {
            break Ok(choice);
        } else {
            println!("Invalid input. Please enter 'yes', 'y', 'no', or 'n'.");
        }
    }
}
