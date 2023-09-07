use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::{
    classify_response_error, UnknownError, Boolean, KeyValuePair,
};

#[derive(Debug, Error, PartialEq)]
pub enum LoggingError {
    #[error("Unknown response error code: {code}: {message}")]
    UnknownErrorCode { code: i64, message: String },
    #[error("Unknown response error: {message}")]
    UnknownErrorMessage { message: String },
    #[error("Unknown response error.")]
    UnknownError,
}

impl From<i64> for LoggingError {
    fn from(code: i64) -> Self {
        match code {
            _ => LoggingError::UnknownErrorCode {
                code,
                message: "Unknown error".to_string(),
            },
        }
    }
}

impl UnknownError for LoggingError {
    fn unknown_error_code(code: i64, message: String) -> Self {
        LoggingError::UnknownErrorCode { code, message }
    }
    fn unknown_error_message(message: String) -> Self {
        LoggingError::UnknownErrorMessage { message }
    }
    fn unknown_error() -> Self {
        LoggingError::UnknownError
    }
}

#[derive(Deserialize, Debug)]
pub struct Logger {
    #[serde(rename = "isEnabled")]
    pub is_enabled: Boolean,
    pub key: String,
    pub title: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub website: Option<String>,
    pub level: Option<String>,
    pub config: Option<Vec<Option<KeyValuePair>>>,
}

pub mod logger_list {
    use super::*;

    pub struct LoggerList;

    pub const OPERATION_NAME: &str = "LoggerList";
    pub const QUERY : & str = "query LoggerList($filter: String, $orderBy: String) {\n  logging {\n    loggers(filter: $filter, orderBy: $orderBy) {\n      isEnabled\n      key\n      title\n      description\n      logo\n      website\n      level\n      config {\n        key\n        value\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub filter: Option<String>,
        #[serde(rename = "orderBy")]
        pub order_by: Option<String>,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub logging: Option<Logging>,
    }

    #[derive(Deserialize)]
    pub struct Logging {
        pub loggers: Option<Vec<Option<Logger>>>,
    }

    impl graphql_client::GraphQLQuery for LoggerList {
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

pub fn logger_list(
    client: &Client,
    url: &str,
    filter: Option<String>,
    order_by: Option<String>,
) -> Result<Vec<Logger>, LoggingError> {
    let variables = logger_list::Variables { filter, order_by };
    let response = post_graphql::<logger_list::LoggerList, _>(client, url, variables);
    if let Err(e) = response {
        return Err(LoggingError::UnknownErrorMessage {
            message: e.to_string(),
        });
    }
    let response = response.unwrap();
    if let Some(data) = response.data {
        if let Some(logging) = data.logging {
            if let Some(loggers) = logging.loggers {
                return Ok(loggers.into_iter().flatten().collect());
            }
        }
    }
    Err(classify_response_error(
        response.errors,
    ))
}
