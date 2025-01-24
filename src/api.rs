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
            .map(|s| {
                let token = s.split_whitespace().last().unwrap().trim().to_string();
                Token(token)
            });

        Api { token }
    }

    fn handle_response(
        response: Result<ureq::Response, ureq::Error>,
    ) -> Result<serde_json::Value, String> {
        match response {
            Ok(res) => res
                .into_json()
                .map_err(|_| "Internal Error: Failed to parse the API response".to_string()),
            Err(ureq::Error::Status(_, res)) => match res.into_json::<serde_json::Value>() {
                Ok(json) => {
                    if let Some(error_type) = json["error"]["type"].as_str() {
                        Err(error_type.to_string())
                    } else {
                        Err("Unknown error format".to_string())
                    }
                }
                _ => unreachable!(
                    "if this prints then what i thought was wrong and there is some other err"
                ),
            },
            Err(_) => Err("Request failed unexpectedly".to_string()),
        }
    }

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

        Self::handle_response(request.send_json(json_body))
    }

    pub fn login(&self, usr: String, pwd: String) -> Result<serde_json::Value, String> {
        let url = format!("{API_BASE_PATH}login");

        let request = ureq::post(&url);
        let request = if let Some(token) = &self.token {
            request.set("Authorization", &format!("Bearer {}", token.0))
        } else {
            request
        };

        let json_body = json!({"username": usr, "password": pwd});

        Self::handle_response(request.send_json(json_body))
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

        Self::handle_response(request.call())
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
