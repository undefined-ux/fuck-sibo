use thiserror::Error;

use crate::model::JyhCode;

#[derive(Error, Debug)]
pub enum SiboError {
    #[error("Login Failed: {user_name} {message}")]
    LoginFailed { user_name: String, message: String },
    #[error("{message}")]
    SubmitFailed { message: String },

    #[error("Request Failed: [{error_code}] {error_message}")]
    RequestFailed {
        jyh: JyhCode,
        error_code: String,
        error_message: String,
    },
    #[error("Network Error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("Failed to parse Json: {0}")]
    JsonParseError(#[from] serde_json::Error),
    #[error("Unknown Error: {message}")]
    UnknownError { message: String },
}
