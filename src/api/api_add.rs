/// # Api Module: Add Task Operation
///
/// This module provides functionality for adding tasks to the specified table.
///
/// ## Struct
///
/// - `Api`: Implementation of the API structure.
///
/// ## Methods
///
/// - `add_task`: Method to add a task to the specified table.
///
///   ### Parameters
///
///   - `tablename`: Name of the table where the task will be added.
///   - `body`: HashMap containing key-value pairs representing the task details.
///
///   ### Returns
///
///   Returns a `Result<Box<dyn FormattedResponse>>`:
///
///   - On success, returns a boxed trait object implementing `FormattedResponse`, which can be
///     either `SuccessfulResponse` or `ErrorResponse`.
///   - On failure, returns an `Error`.
use std::collections::HashMap;
use std::io::Read;

use reqwest::{blocking, header};
use serde_json::json;

use crate::api::{ErrorResponse, SuccessfulResponse};
use crate::error::{Error, Result};
use crate::utils::table_formatter::FormattedResponse;

use super::{Api, BACKEND};

impl Api {
    pub fn add_task(
        &self,
        tablename: String,
        body: HashMap<&str, &str>,
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
        let url = format!("{}/{}", BACKEND, tablename);
        let body = json!(body).to_string();

        let mut response = client
            .post(url)
            .header(header::COOKIE, token)
            .header(header::CONTENT_TYPE, "application/json")
            .body(body)
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
