use std::io::Read;

use chrono::{DateTime, Utc};
use reqwest::{blocking, header};
use serde_json::json;

use super::{Api, ErrorResponse, SuccessfulResponse, BACKEND};
use crate::{
    error::{Error, Result},
    utils::table_formatter::FormattedResponse,
};

impl Api {
    pub fn post_signup(&self, usr: &str, pwd: &str) -> Result<Box<dyn FormattedResponse>> {
        let client = blocking::Client::builder()
            .cookie_store(true)
            .build()
            .map_err(|_| Error::FailedToConnectToServer)?;

        let token: String = self.token.clone().into();
        let url = format!("{}/signup", BACKEND);
        let payload = json!({
            "username": usr.trim(),
            "password": pwd.trim(),
        })
        .to_string();

        let mut response = client
            .post(url)
            .header(header::COOKIE, token)
            .header(header::CONTENT_TYPE, "application/json")
            .body(payload)
            .send()
            .map_err(|_| Error::FailedToConnectToServer)?;

        let mut body = String::new();
        response
            .read_to_string(&mut body)
            .map_err(|_| Error::InvalidServerResponse)?;

        let json_response_obj: Box<dyn FormattedResponse> = if body.contains("error") {
            let err_response: ErrorResponse =
                serde_json::from_str(&body).map_err(|_| Error::FailedtoReadServerResponse)?;
            Box::new(err_response)
        } else {
            let err_response: SuccessfulResponse =
                serde_json::from_str(&body).map_err(|_| Error::FailedtoReadServerResponse)?;
            Box::new(err_response)
        };

        Ok(json_response_obj)
    }

    pub fn post_login(&self, key: &str) -> Result<(Box<dyn FormattedResponse>, String)> {
        let client = blocking::Client::builder()
            .cookie_store(true)
            .build()
            .map_err(|_| Error::FailedToConnectToServer)?;

        let token: String = self.token.clone().into();
        let url = format!("{}/login", BACKEND);
        let payload = json!({
            "key": key.trim(),
        })
        .to_string();

        let mut response = client
            .post(url)
            .header(header::COOKIE, token)
            .header(header::CONTENT_TYPE, "application/json")
            .body(payload)
            .send()
            .map_err(|_| Error::FailedToConnectToServer)?;

        let token = response
            .cookies()
            .into_iter()
            .map(|cookie| {
                let expires_string = match cookie.expires() {
                    Some(expires) => {
                        let datetime: DateTime<Utc> = expires.into(); // Convert SystemTime to DateTime<Utc>
                        datetime.format("%a, %d %b %Y %H:%M:%S GMT").to_string()
                        // Format the DateTime
                    }
                    None => String::new(),
                };

                format!(
                    "{}={}; Path={}; HttpOnly; Expires={}",
                    cookie.name(),
                    cookie.value(),
                    cookie.path().unwrap_or("/"),
                    expires_string,
                )
            })
            .collect::<Vec<String>>()
            .join("; ");

        let mut body = String::new();

        response
            .read_to_string(&mut body)
            .map_err(|_| Error::InvalidServerResponse)?;

        let json_response_obj: Box<dyn FormattedResponse> = if body.contains("error") {
            let err_response: ErrorResponse =
                serde_json::from_str(&body).map_err(|_| Error::FailedtoReadServerResponse)?;
            Box::new(err_response)
        } else {
            let err_response: SuccessfulResponse =
                serde_json::from_str(&body).map_err(|_| Error::FailedtoReadServerResponse)?;
            Box::new(err_response)
        };

        Ok((json_response_obj, token))
    }
}
