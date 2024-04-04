use std::{
    fs::File,
    io::{Read, Write},
};

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    key: Option<String>,
    pub first_run: bool,
}

impl Config {
    pub fn get_or_set_config() -> Result<Config> {
        read_file().map_err(|e| {
            log::error!("Error in reading the file {e}");
            Error::InvalidConfig
        })
    }
}

fn read_file() -> std::io::Result<Config> {
    let file_path = "rsm-conf.json";

    if !file_exists_or_empty(&file_path)? {
        write_default_json(&file_path)?;
    }

    let mut file = File::open(&file_path)?;

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
fn write_default_json(file_path: &str) -> std::io::Result<()> {
    let default_json = json!({
        "key": null,
        "first_run": true
    });

    let json_string = serde_json::to_string_pretty(&default_json)?;

    let mut file = File::create(&file_path)?;
    file.write_all(json_string.as_bytes())?;
    Ok(())
}
