// modules_definition
pub mod api;

// -- general api utils definitions
use crate::utils::config_helper::Token;
use crate::{error::Result, utils::config_helper::Config};
use serde::{Deserialize, Serialize};

const BACKEND: &str = "http://100.97.63.15:10001";

pub struct Api {
    token: Token,
}

impl Api {
    pub fn new() -> Result<Api> {
        let token = Config::load_token()?;
        Ok(Api { token })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorDetail {
    pub req_uuid: String,
    #[serde(rename = "type")]
    pub error_type: String,
}
