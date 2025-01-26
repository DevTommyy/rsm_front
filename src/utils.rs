use std::{
    fs,
    io::{self, Write},
};

// i dont care about the pass being shown on the terminal since only i am using this
pub fn prompt_credentials() -> io::Result<(String, String)> {
    print!("Enter username: ");
    io::stdout().flush()?;
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;

    print!("Enter password: ");
    io::stdout().flush()?;
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;

    Ok((username.trim().to_string(), password.trim().to_string()))
}

pub fn prompt_logout() -> io::Result<bool> {
    print!("Are you sure you want to logout? [Y/n]: ");
    io::stdout().flush()?;
    let mut logout = String::new();
    io::stdin().read_line(&mut logout)?;

    let logout = logout.trim();

    match logout.to_lowercase().as_str() {
        "y" | "" => Ok(true), // 'y' or empty input means yes
        "n" => Ok(false),     // 'n' means no
        _ => {
            println!("Invalid input. Please respond with 'Y' or 'n'.");
            prompt_logout()
        }
    }
}

/// Gets the system timezone as a `chrono_tz::Tz` instance.
pub fn get_sys_tz() -> Option<chrono_tz::Tz> {
    #[cfg(target_os = "linux")]
    {
        // Read the contents of /etc/timezone (common on Debian-based systems)
        if let Ok(timezone) = fs::read_to_string("/etc/timezone") {
            return timezone.trim().parse().ok();
        }

        // Read the timezone info from /etc/localtime (common on many Linux distributions)
        if let Ok(target) = fs::read_link("/etc/localtime") {
            if let Some(zone) = target
                .to_str()
                .and_then(|path| path.split("/zoneinfo/").nth(1))
            {
                return zone.parse().ok();
            }
        }
    }

    // TODO: test this on macos
    #[cfg(target_os = "macos")]
    {
        // macOS also uses /etc/localtime symlink to /var/db/timezone/zoneinfo
        if let Ok(target) = fs::read_link("/etc/localtime") {
            if let Some(zone) = target
                .to_str()
                .and_then(|path| path.split("/zoneinfo/").nth(1))
            {
                return zone.parse().ok();
            }
        }
    }

    None
}

/// Due parsing logic
#[derive(Debug, Clone, serde::Serialize)]
pub struct Due(pub chrono::NaiveDateTime); // due is parsed either as 'hh:mm' of the same day or tomorrow or 'YYYY-MM-dd hh:mm'.

#[derive(Debug)]
pub enum DueParseError {
    InvalidFormat,
}

impl std::fmt::Display for DueParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DueParseError::InvalidFormat => write!(
                f,
                "Invalid due date format. Expected 'hh:mm' or 'YYYY-MM-dd hh:mm'."
            ),
        }
    }
}

impl TryFrom<&str> for Due {
    type Error = DueParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use chrono::{Duration, Local, NaiveDateTime, NaiveTime};

        // Try parsing as 'YYYY-MM-dd hh:mm'
        if let Ok(parsed) = NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M") {
            return Ok(Due(parsed));
        }

        // Try parsing as 'hh:mm' (today or tomorrow)
        if let Ok(time) = NaiveTime::parse_from_str(value, "%H:%M") {
            let now = Local::now().naive_local();

            let today = now.date();
            let today_datetime = NaiveDateTime::new(today, time);

            // Check if the time is already in the past today
            if today_datetime < now {
                // If it's already passed today, set it for tomorrow
                let tomorrow = today + Duration::days(1);
                let tomorrow_datetime = NaiveDateTime::new(tomorrow, time);
                return Ok(Due(tomorrow_datetime));
            } else {
                // Otherwise, set it for today
                return Ok(Due(today_datetime));
            }
        }

        // If none of the formats matched, return an error
        Err(DueParseError::InvalidFormat)
    }
}

// Custom parser for the `due` field.
pub fn parse_due(value: &str) -> Result<Due, String> {
    Due::try_from(value).map_err(|e| e.to_string())
}
