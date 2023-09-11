use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::{
    classify_response_error, classify_response_status_error, Boolean, Int,
    KnownErrorCodes, ResponseStatus, UnknownError,
};

#[derive(Error, Debug, PartialEq)]
pub enum NavigationError {
    #[error("Unknown response error code: {code}: {message}")]
    UnknownErrorCode { code: i64, message: String },
    #[error("Unknown response error: {message}")]
    UnknownErrorMessage { message: String },
    #[error("Unknown response error.")]
    UnknownError,
}

impl From<i64> for NavigationError {
    fn from(code: i64) -> Self {
        NavigationError::UnknownErrorCode {
            code,
            message: "Unknown error".to_string(),
        }
    }
}

impl UnknownError for NavigationError {
    fn unknown_error_code(code: i64, message: String) -> Self {
        NavigationError::UnknownErrorCode { code, message }
    }
    fn unknown_error_message(message: String) -> Self {
        NavigationError::UnknownErrorMessage { message }
    }
    fn unknown_error() -> Self {
        NavigationError::UnknownError
    }
}

impl KnownErrorCodes for NavigationError {
    fn known_error_codes() -> Vec<i64> {
        Vec::new()
    }

    fn is_known_error_code(_code: i64) -> bool {
        false
    }
}
