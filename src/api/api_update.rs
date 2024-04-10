use std::collections::HashMap;
use std::io::Read;

use reqwest::{blocking, header};
use serde_json::json;
use urlencoding::encode;

use crate::api::{ErrorResponse, SuccessfulResponse};
use crate::error::{Error, Result};
use crate::utils::table_formatter::FormattedResponse;

use super::{Api, BACKEND};

impl Api {
    pub fn update_task(
        &self,
        tablename: String,
        old_desc: String,
        body: HashMap<&str, &str>,
    ) -> Result<Box<dyn FormattedResponse>> {
        let client = blocking::Client::builder()
            .cookie_store(true)
            .build()
            .map_err(|_| Error::FailedToConnectToServer)?;

        let token: String = self.token.clone().unwrap_or_default().into();
        let url_formatted_desc = encode(&old_desc);
        let url = format!("{}/{}/{}", BACKEND, tablename, url_formatted_desc);
        let body = json!(body).to_string();
        println!("DEBUG: {url}");

        let mut response = client
            .put(url)
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
