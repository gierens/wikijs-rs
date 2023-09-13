use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::{
    classify_response_error, classify_response_status_error, Boolean,
    KeyValuePair, KeyValuePairInput, KnownErrorCodes, ResponseStatus,
    UnknownError,
};

#[derive(Debug, Error, PartialEq)]
pub enum SiteError {
    #[error("Unknown response error code: {code}: {message}")]
    UnknownErrorCode { code: i64, message: String },
    #[error("Unknown response error: {message}")]
    UnknownErrorMessage { message: String },
    #[error("Unknown response error.")]
    UnknownError,
}

impl From<i64> for SiteError {
    fn from(code: i64) -> Self {
        SiteError::UnknownErrorCode {
            code,
            message: "Unknown error".to_string(),
        }
    }
}

impl UnknownError for SiteError {
    fn unknown_error_code(code: i64, message: String) -> Self {
        SiteError::UnknownErrorCode { code, message }
    }
    fn unknown_error_message(message: String) -> Self {
        SiteError::UnknownErrorMessage { message }
    }
    fn unknown_error() -> Self {
        SiteError::UnknownError
    }
}

impl KnownErrorCodes for SiteError {
    fn known_error_codes() -> Vec<i64> {
        Vec::new()
    }

    fn is_known_error_code(_code: i64) -> bool {
        false
    }
}
