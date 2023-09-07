use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::{
    classify_response_error, Date, Int, KnownErrorCodes, UnknownError,
};

#[derive(Debug, Error, PartialEq)]
pub enum MailError {
    #[error("An unexpected error occurred during mail operation.")]
    MailGenericError,
    #[error("The mail configuration is incomplete or invalid.")]
    MailNotConfigured,
    #[error("Mail template failed to load.")]
    MailTemplateFailed,
    #[error("The recipient email address is invalid.")]
    MailInvalidRecipient,
    #[error("Unknown response error code: {code}: {message}")]
    UnknownErrorCode { code: i64, message: String },
    #[error("Unknown response error: {message}")]
    UnknownErrorMessage { message: String },
    #[error("Unknown response error.")]
    UnknownError,
}

impl From<i64> for MailError {
    fn from(code: i64) -> Self {
        match code {
            3001 => MailError::MailGenericError,
            3002 => MailError::MailNotConfigured,
            3003 => MailError::MailTemplateFailed,
            3004 => MailError::MailInvalidRecipient,
            _ => MailError::UnknownErrorCode {
                code,
                message: "Unknown error".to_string(),
            },
        }
    }
}

impl UnknownError for MailError {
    fn unknown_error_code(code: i64, message: String) -> Self {
        MailError::UnknownErrorCode { code, message }
    }
    fn unknown_error_message(message: String) -> Self {
        MailError::UnknownErrorMessage { message }
    }
    fn unknown_error() -> Self {
        MailError::UnknownError
    }
}

impl KnownErrorCodes for MailError {
    fn known_error_codes() -> Vec<i64> {
        vec![3001, 3002, 3003, 3004]
    }

    fn is_known_error_code(code: i64) -> bool {
        (3001..=3004).contains(&code)
    }
}
