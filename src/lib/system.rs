use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::{
    classify_response_error, KnownErrorCodes, UnknownError, Boolean,
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

#[derive(Deserialize, Debug)]
pub struct SystemFlag {
    pub key: String,
    pub value: Boolean,
}

pub mod system_flag_list {
    use super::*;

    pub struct SystemFlagList;

    pub const OPERATION_NAME: &str = "SystemFlagList";
    pub const QUERY : & str = "query SystemFlagList {\n  system {\n    flags {\n      key\n      value\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub system: Option<System>,
    }

    #[derive(Deserialize, Debug)]
    pub struct System {
        pub flags: Option<Vec<Option<SystemFlag>>>,
    }

    impl graphql_client::GraphQLQuery for SystemFlagList {
        type Variables = Variables;
        type ResponseData = ResponseData;
        fn build_query(
            variables: Self::Variables,
        ) -> ::graphql_client::QueryBody<Self::Variables> {
            graphql_client::QueryBody {
                variables,
                query: QUERY,
                operation_name: OPERATION_NAME,
            }
        }
    }
}

pub fn system_flag_list(
    client: &Client,
    url: &str,
) -> Result<Vec<SystemFlag>, SystemError> {
    let variables = system_flag_list::Variables {};
    let response = post_graphql::<system_flag_list::SystemFlagList, _>(
        client,
        url,
        variables,
    );
    if response.is_err() {
        return Err(SystemError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(system) = data.system {
            if let Some(flags) = system.flags {
                return Ok(flags
                    .into_iter()
                    .filter_map(|x| x)
                    .collect::<Vec<_>>());
            }
        }
    }
    Err(classify_response_error::<SystemError>(
        response_body.errors,
    ))
}
