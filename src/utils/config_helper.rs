use std::{
    fs::File,
    io::{Read, Write},
};

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

const CONFIG_FILE: &str = "rsm-conf.json";

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    key: Option<String>,
    pub first_run: bool,
}

impl Config {
    pub fn get_config() -> Result<Config> {
        read_file().map_err(|e| {
            log::error!("Error in reading the file {e}");
            Error::InvalidConfig
        })
    }

    pub fn update_config(&self) -> Result<()> {
        write_config(CONFIG_FILE, self.key.as_deref(), self.first_run).map_err(|e| {
            log::error!("Error in updating file {e}");
            Error::FailedToUpdateConf
        })
    }
}

fn read_file() -> std::io::Result<Config> {
    if !file_exists_or_empty(CONFIG_FILE)? {
        write_config(CONFIG_FILE, None, true)?;
    }

    let mut file = File::open(CONFIG_FILE)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let data: Config = serde_json::from_str(&contents)?;
    Ok(data)
}

fn file_exists_or_empty(file_path: &str) -> std::io::Result<bool> {
    if let Ok(metadata) = std::fs::metadata(file_path) {
        if metadata.len() == 0 {
            return Ok(false);
        }
        Ok(true)
    } else {
        Ok(false)
    }
}

fn write_config(file_path: &str, key: Option<&str>, first_run: bool) -> std::io::Result<()> {
    let default_json = json!({
        "key": key,
        "first_run": first_run
    });

    let json_string = serde_json::to_string_pretty(&default_json)?;

    let mut file = File::create(&file_path)?;
    file.write_all(json_string.as_bytes())?;
    Ok(())
}
