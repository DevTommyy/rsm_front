/// # Api Module: Task Removal
///
/// This module provides functionality for removing tasks.
///
/// ## Methods
///
/// - `remove_task`: Method to remove a task from a specified table.
use std::io::Read;

use reqwest::{blocking, header};
use urlencoding::encode;

use crate::api::{ErrorResponse, SuccessfulResponse};
use crate::error::{Error, Result};
use crate::utils::table_formatter::FormattedResponse;

use super::{Api, BACKEND};

impl Api {
    pub fn remove_task(
        &self,
        tablename: String,
        desc: String,
    ) -> Result<Box<dyn FormattedResponse>> {
        let client = blocking::Client::builder()
            .cookie_store(true)
            .build()
            .map_err(|_| Error::FailedToConnectToServer)?;

        let tablename = match tablename {
            x if ["reminder", "todo"].contains(&x.as_str()) => x.to_owned(),

            name => format!("user/{}", name),
        };
        let token: String = self.token.clone().unwrap_or_default().into();
        let url_encoded_desc = encode(&desc);
        let url = format!("{}/{}/{}", BACKEND, tablename, url_encoded_desc);

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
            let task_response: SuccessfulResponse =
                serde_json::from_str(&body).map_err(|_| Error::FailedtoReadServerResponse)?;
            Box::new(task_response)
        };

        Ok(json_response_obj)
    }
}
