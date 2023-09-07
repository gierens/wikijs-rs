use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::{
    classify_response_error, Date, Int, KnownErrorCodes, UnknownError,
};

#[derive(Debug, Error, PartialEq)]
pub enum LocaleError {
    #[error("An unexpected error occurred during locale operation.")]
    LocaleGenericError,
    #[error("Invalid locale or namespace.")]
    LocaleInvalidNamespace,
    #[error("Unknown response error code: {code}: {message}")]
    UnknownErrorCode { code: i64, message: String },
    #[error("Unknown response error: {message}")]
    UnknownErrorMessage { message: String },
    #[error("Unknown response error.")]
    UnknownError,
}

impl From<i64> for LocaleError {
    fn from(code: i64) -> Self {
        match code {
            5001 => LocaleError::LocaleGenericError,
            5002 => LocaleError::LocaleInvalidNamespace,
            _ => LocaleError::UnknownErrorCode {
                code,
                message: "Unknown error".to_string(),
            },
        }
    }
}

impl UnknownError for LocaleError {
    fn unknown_error_code(code: i64, message: String) -> Self {
        LocaleError::UnknownErrorCode { code, message }
    }
    fn unknown_error_message(message: String) -> Self {
        LocaleError::UnknownErrorMessage { message }
    }
    fn unknown_error() -> Self {
        LocaleError::UnknownError
    }
}

impl KnownErrorCodes for LocaleError {
    fn known_error_codes() -> Vec<i64> {
        vec![5001, 5002]
    }

    fn is_known_error_code(code: i64) -> bool {
        (5001..=5002).contains(&code)
    }
}
