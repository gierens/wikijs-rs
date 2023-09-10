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

impl KnownErrorCodes for LoggingError {
    fn known_error_codes() -> Vec<i64> {
        Vec::new()
    }

    fn is_known_error_code(_code: i64) -> bool {
        false
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

#[derive(Serialize, Debug)]
pub struct LoggerInput {
    #[serde(rename = "isEnabled")]
    pub is_enabled: Boolean,
    pub key: String,
    pub level: String,
    pub config: Option<Vec<Option<KeyValuePairInput>>>,
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
    let response =
        post_graphql::<logger_list::LoggerList, _>(client, url, variables);
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
    Err(classify_response_error(response.errors))
}

pub mod logger_update {
    use super::*;

    pub struct LoggerUpdate;

    pub const OPERATION_NAME: &str = "LoggerUpdate";
    pub const QUERY : & str = "mutation LoggerUpdate($loggers: [LoggerInput]) {\n  logging {\n    updateLoggers(loggers: $loggers) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub loggers: Option<Vec<Option<LoggerInput>>>,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub logging: Option<Logging>,
    }

    #[derive(Deserialize)]
    pub struct Logging {
        #[serde(rename = "updateLoggers")]
        pub update_loggers: Option<UpdateLoggers>,
    }

    #[derive(Deserialize)]
    pub struct UpdateLoggers {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for LoggerUpdate {
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

pub fn logger_update(
    client: &Client,
    url: &str,
    loggers: Vec<LoggerInput>,
) -> Result<(), LoggingError> {
    let variables = logger_update::Variables {
        loggers: Some(loggers.into_iter().map(|logger| Some(logger)).collect()),
    };
    let response =
        post_graphql::<logger_update::LoggerUpdate, _>(client, url, variables);
    if let Err(e) = response {
        return Err(LoggingError::UnknownErrorMessage {
            message: e.to_string(),
        });
    }
    let response = response.unwrap();
    if let Some(data) = response.data {
        if let Some(logging) = data.logging {
            if let Some(update_loggers) = logging.update_loggers {
                if let Some(response_result) = update_loggers.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(classify_response_status_error(
                            response_result,
                        ));
                    }
                }
            }
        }
    }
    Err(classify_response_error(response.errors))
}
