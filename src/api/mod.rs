// modules_definition
pub mod api_list;

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
    pub error_type: ErrorType,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub enum ErrorType {
    LOGIN_FAIL,
    USER_NOT_FOUND,
    USERNAME_ALREADY_USED,
    TABLENAME_ALREADY_USED,
    NO_AUTH,
    INVALID_PARAMS,
    DUE_UNSUPPORTED,
    INVALID_QUERY_PARAMS,
    SERVICE_ERROR,
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_message = match self {
            ErrorType::LOGIN_FAIL => "Login failed",
            ErrorType::USER_NOT_FOUND => "User not found",
            ErrorType::USERNAME_ALREADY_USED => "Username already used",
            ErrorType::TABLENAME_ALREADY_USED => "Tablename already used",
            ErrorType::NO_AUTH => "No authentication",
            ErrorType::INVALID_PARAMS | ErrorType::INVALID_QUERY_PARAMS => "Invalid args",
            ErrorType::DUE_UNSUPPORTED => "Unsupported due",
            ErrorType::SERVICE_ERROR => "Server error",
        };
        write!(f, "{:<29}", error_message)
    }
}
