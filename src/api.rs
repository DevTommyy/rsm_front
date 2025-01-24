use ureq::json;

use crate::utils;

struct Token(String);

const API_BASE_PATH: &str = "http://127.0.0.1:8080/";

#[derive(Default)]
pub struct Api {
    token: Option<Token>,
}

impl Api {
    pub fn from_token_file() -> Self {
        let token: Option<Token> = std::fs::read_to_string(".token")
            .ok()
            .filter(|s| !s.trim().is_empty())
            .map(|s| Token(s.trim().to_string()));

        Api { token }
    }

    // i dont care about the pass being shown on the terminal since only i am using this
    pub fn register_user(&self, usr: String, pwd: String) -> Result<serde_json::Value, String> {
        let url = format!("{API_BASE_PATH}signup");

        let request = ureq::post(&url);
        let request = if let Some(token) = &self.token {
            request.set("Authorization", &format!("Bearer {}", token.0))
        } else {
            request
        };

        // defaults to UTC
        let tz: chrono_tz::Tz = utils::get_sys_tz().unwrap_or_default();
        let json_body = json!({"username": usr, "password": pwd, "timezone": tz});

        match request.send_json(json_body) {
            Ok(res) => match res.into_json() {
                Ok(json) => Ok(json),
                Err(_) => Err("Internal Error: Failed to parse the API response".to_string()),
            },
            Err(ureq::Error::Status(_, res)) => Err(res.status_text().to_string()),
            _ => unreachable!(),
        }
    }

    pub fn list_table_contents(
        &self,
        tablename: &str,
        group: Option<&str>,
        sort_by: Option<&str>,
    ) -> Result<serde_json::Value, String> {
        let mut url = format!("{API_BASE_PATH}{tablename}");

        let mut query_params = vec![];
        if let Some(group) = group {
            query_params.push(format!("group={}", group));
        }
        if let Some(sort_by) = sort_by {
            query_params.push(format!("sort_by={}", sort_by));
        }
        if !query_params.is_empty() {
            url.push('?');
            url.push_str(&query_params.join("&"));
        }

        let request = ureq::get(&url);
        let request = if let Some(token) = &self.token {
            request.set("Authorization", &format!("Bearer {}", token.0))
        } else {
            request
        };

        match request.call() {
            Ok(res) => match res.into_json() {
                Ok(json) => Ok(json),
                Err(_) => Err("Internal Error: Failed to parse the API response".to_string()),
            },
            Err(ureq::Error::Status(_, res)) => Err(res.status_text().to_string()),
            _ => unreachable!(),
        }
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    pub fn has_connection(&self) -> Result<bool, String> {
        use std::error::Error;
        use std::io;

        let url = "http://neverssl.com";

        match ureq::get(url).call() {
            Ok(_) => Ok(true),
            Err(ureq::Error::Transport(transport_error)) => {
                if let Some(io_err) = transport_error
                    .source()
                    .and_then(|e| e.downcast_ref::<io::Error>())
                {
                    match io_err.kind() {
                        io::ErrorKind::ConnectionRefused => {
                            Err("Connection refused the server is down".to_string())
                        }
                        io::ErrorKind::TimedOut => Err("Connection timed out".to_string()),
                        _ => Err(format!("Network error: {}", io_err)),
                    }
                } else {
                    Err("A network error occurred".to_string())
                }
            }
            Err(e) => {
                println!("{e}");
                Err("An unexpected error has occurred".to_string())
            }
        }
    }
}
