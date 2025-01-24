use std::{
    fs,
    io::{self, Write},
};

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

/// Gets the system timezone as a `chrono_tz::Tz` instance.
pub fn get_sys_tz() -> Option<chrono_tz::Tz> {
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

    None
}
