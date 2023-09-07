use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::{
    classify_response_error, Date, Int, KnownErrorCodes, UnknownError,
};

#[derive(Debug, Error, PartialEq)]
pub enum SystemError {
    #[error("An unexpected error occurred.")]
    SystemGenericError,
    #[error("SSL is not enabled.")]
    SystemSSLDisabled,
    #[error("Current provider does not support SSL certificate renewal.")]
    SystemSSLRenewInvalidProvider,
    #[error("Let's Encrypt is not initialized.")]
    SystemSSLLEUnavailable,
    #[error("Unknown response error code: {code}: {message}")]
    UnknownErrorCode { code: i64, message: String },
    #[error("Unknown response error: {message}")]
    UnknownErrorMessage { message: String },
    #[error("Unknown response error.")]
    UnknownError,
}

impl From<i64> for SystemError {
    fn from(code: i64) -> Self {
        match code {
            7001 => SystemError::SystemGenericError,
            7002 => SystemError::SystemSSLDisabled,
            7003 => SystemError::SystemSSLRenewInvalidProvider,
            7004 => SystemError::SystemSSLLEUnavailable,
            _ => SystemError::UnknownErrorCode {
                code,
                message: "Unknown error".to_string(),
            },
        }
    }
}

impl UnknownError for SystemError {
    fn unknown_error_code(code: i64, message: String) -> Self {
        SystemError::UnknownErrorCode { code, message }
    }
    fn unknown_error_message(message: String) -> Self {
        SystemError::UnknownErrorMessage { message }
    }
    fn unknown_error() -> Self {
        SystemError::UnknownError
    }
}

impl KnownErrorCodes for SystemError {
    fn known_error_codes() -> Vec<i64> {
        vec![7001, 7002, 7003, 7004]
    }

    fn is_known_error_code(code: i64) -> bool {
        (7001..=7004).contains(&code)
    }
}
