use chrono::NaiveDateTime;
use reqwest::{blocking, header};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use std::io::Read;

use crate::error::{Error, Result};
use crate::utils::table_formatter::FormattedResponse;

use super::{Api, ErrorResponse, BACKEND};

#[derive(Deserialize, Serialize)]
pub struct TableCharacteristicsResponse {
    pub res: Vec<TableCharacteristicsResponseDetails>,
}

#[derive(Deserialize, Serialize)]
pub struct TableCharacteristicsResponseDetails {
    pub has_due: bool,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetTaskResponse {
    pub res: Vec<GetTaskResponseDetail>,
}

#[derive(Deserialize, Serialize)]
#[skip_serializing_none]
pub struct GetTaskResponseDetail {
    pub description: String,
    pub group: String,
    pub due: Option<NaiveDateTime>,
}

impl Api {
    pub fn get_tasks(
        &self,
        tablename: Option<&str>,
        opts: HashMap<&str, &str>,
    ) -> Result<Box<dyn FormattedResponse>> {
        let client = blocking::Client::builder()
            .cookie_store(true)
            .build()
            .map_err(|_| Error::FailedToConnectToServer)?;

        let token: String = self.token.clone().unwrap_or_default().into();
        let table = match tablename {
            Some(name) => {
                if ["reminder", "todo"].contains(&name) {
                    name.to_owned()
                } else {
                    format!("user/{}", name)
                }
            }
            None => "list".to_owned(),
        };

        let mut url = format!("{}/{}", BACKEND, table);

        if !opts.is_empty() {
            let mut encoded_params = String::new();
            for (key, value) in opts.iter() {
                let encoded_key = urlencoding::encode(key);
                let encoded_value = urlencoding::encode(value);
                encoded_params.push_str(&format!("{}={}&", encoded_key, encoded_value));
            }
            // remove trailing '&'
            encoded_params.pop();
            url.push_str(&format!("?{}", encoded_params));
        }

        let mut response = client
            .get(url)
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
            match tablename {
                Some(_) => {
                    let task_response: GetTaskResponse = serde_json::from_str(&body)
                        .map_err(|_| Error::FailedtoReadServerResponse)?;
                    Box::new(task_response)
                }
                None => {
                    let table_char_response: TableCharacteristicsResponse =
                        serde_json::from_str(&body)
                            .map_err(|_| Error::FailedtoReadServerResponse)?;
                    Box::new(table_char_response)
                }
            }
        };

        Ok(json_response_obj)
    }
}
