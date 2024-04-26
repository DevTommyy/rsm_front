/// # Api Module: Table Operations
///
/// This module provides functionality for creating and removing tables.
///
/// ## Methods
///
/// - `create_table`: Method to create a new table with optional due time.
/// - `remove_table`: Method to remove an existing table.
use std::io::Read;

use reqwest::{blocking, header};
use serde_json::json;

use crate::utils::table_formatter::FormattedResponse;

use crate::error::{Error, Result};

use super::{Api, ErrorResponse, SuccessfulResponse, BACKEND};

impl Api {
    pub fn create_table(
        &self,
        tablename: String,
        has_due: bool,
    ) -> Result<Box<dyn FormattedResponse>> {
        let client = blocking::Client::builder()
            .cookie_store(true)
            .build()
            .map_err(|_| Error::FailedToConnectToServer)?;

        let token: String = self.token.clone().unwrap_or_default().into();
        let url = format!("{}/{}", BACKEND, tablename.trim());
        let payload = json!({
            "due_time": has_due
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
            let success_response: SuccessfulResponse =
                serde_json::from_str(&body).map_err(|_| Error::FailedtoReadServerResponse)?;
            Box::new(success_response)
        };

        Ok(json_response_obj)
    }

    pub fn remove_table(&self, tablename: String) -> Result<Box<dyn FormattedResponse>> {
        let client = blocking::Client::builder()
            .cookie_store(true)
            .build()
            .map_err(|_| Error::FailedToConnectToServer)?;

        let token: String = self.token.clone().unwrap_or_default().into();
        let url = format!("{}/{}", BACKEND, tablename.trim());

        let mut response = client
            .delete(url)
            .header(header::COOKIE, token)
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
            let success_response: SuccessfulResponse =
                serde_json::from_str(&body).map_err(|_| Error::FailedtoReadServerResponse)?;
            Box::new(success_response)
        };

        Ok(json_response_obj)
    }
}
