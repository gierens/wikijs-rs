use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::{
    classify_response_error, Date, Int, KnownErrorCodes, UnknownError,
};

#[derive(Debug, Error, PartialEq)]
pub enum SearchError {
    #[error("An unexpected error occurred during search operation.")]
    SearchGenericError,
    #[error("Search Engine activation failed.")]
    SearchActivationFailed,
    #[error("Unknown response error code: {code}: {message}")]
    UnknownErrorCode { code: i64, message: String },
    #[error("Unknown response error: {message}")]
    UnknownErrorMessage { message: String },
    #[error("Unknown response error.")]
    UnknownError,
}

impl From<i64> for SearchError {
    fn from(code: i64) -> Self {
        match code {
            4001 => SearchError::SearchGenericError,
            4002 => SearchError::SearchActivationFailed,
            _ => SearchError::UnknownErrorCode {
                code,
                message: "Unknown error".to_string(),
            },
        }
    }
}

impl UnknownError for SearchError {
    fn unknown_error_code(code: i64, message: String) -> Self {
        SearchError::UnknownErrorCode { code, message }
    }
    fn unknown_error_message(message: String) -> Self {
        SearchError::UnknownErrorMessage { message }
    }
    fn unknown_error() -> Self {
        SearchError::UnknownError
    }
}

impl KnownErrorCodes for SearchError {
    fn known_error_codes() -> Vec<i64> {
        vec![4001, 4002]
    }

    fn is_known_error_code(code: i64) -> bool {
        (4001..=4002).contains(&code)
    }
}
