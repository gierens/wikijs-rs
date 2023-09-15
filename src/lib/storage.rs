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
pub enum StorageError {
    #[error("Unknown response error code: {code}: {message}")]
    UnknownErrorCode { code: i64, message: String },
    #[error("Unknown response error: {message}")]
    UnknownErrorMessage { message: String },
    #[error("Unknown response error.")]
    UnknownError,
}

impl From<i64> for StorageError {
    fn from(code: i64) -> Self {
        StorageError::UnknownErrorCode {
            code,
            message: "Unknown error".to_string(),
        }
    }
}

impl UnknownError for StorageError {
    fn unknown_error_code(code: i64, message: String) -> Self {
        StorageError::UnknownErrorCode { code, message }
    }
    fn unknown_error_message(message: String) -> Self {
        StorageError::UnknownErrorMessage { message }
    }
    fn unknown_error() -> Self {
        StorageError::UnknownError
    }
}

impl KnownErrorCodes for StorageError {
    fn known_error_codes() -> Vec<i64> {
        Vec::new()
    }

    fn is_known_error_code(_code: i64) -> bool {
        false
    }
}

#[derive(Deserialize, Debug)]
pub struct StorageStatus {
    pub key: String,
    pub title: String,
    pub status: String,
    pub message: String,
    #[serde(rename = "lastAttempt")]
    pub last_attempt: String,
}

pub mod storage_action_execute {
    use super::*;

    pub struct StorageActionExecute;

    pub const OPERATION_NAME: &str = "StorageActionExecute";
    pub const QUERY : & str = "mutation StorageActionExecute(\n  $targetKey: String!\n  $handler: String!\n) {\n  storage {\n    executeAction(\n      targetKey: $targetKey\n      handler: $handler\n    ) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "targetKey")]
        pub target_key: String,
        pub handler: String,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub storage: Option<Storage>,
    }

    #[derive(Deserialize)]
    pub struct Storage {
        #[serde(rename = "executeAction")]
        pub execute_action: Option<ExecuteAction>,
    }
    #[derive(Deserialize)]
    pub struct ExecuteAction {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for StorageActionExecute {
        type Variables = storage_action_execute::Variables;
        type ResponseData = storage_action_execute::ResponseData;
        fn build_query(
            variables: Self::Variables,
        ) -> ::graphql_client::QueryBody<Self::Variables> {
            graphql_client::QueryBody {
                variables,
                query: storage_action_execute::QUERY,
                operation_name: storage_action_execute::OPERATION_NAME,
            }
        }
    }
}

pub fn storage_action_execute(
    client: &Client,
    url: &str,
    target_key: String,
    handler: String,
) -> Result<(), StorageError> {
    let variables = storage_action_execute::Variables {
        target_key,
        handler,
    };
    let response = post_graphql::<
        storage_action_execute::StorageActionExecute,
        _,
    >(client, url, variables);
    if response.is_err() {
        return Err(StorageError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(storage) = data.storage {
            if let Some(execute_action) = storage.execute_action {
                if let Some(response_result) = execute_action.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(classify_response_status_error::<
                            StorageError,
                        >(response_result));
                    }
                }
            }
        }
    }
    Err(classify_response_error::<StorageError>(
        response_body.errors,
    ))
}

pub mod storage_status_get {
    use super::*;

    pub struct StorageStatusGet;

    pub const OPERATION_NAME: &str = "StorageStatusGet";
    pub const QUERY : & str = "query StorageStatusGet {\n  storage {\n    status {\n      key\n      title\n      status\n      message\n      lastAttempt\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub storage: Option<Storage>,
    }

    #[derive(Deserialize)]
    pub struct Storage {
        pub status: Option<Vec<Option<StorageStatus>>>,
    }

    impl graphql_client::GraphQLQuery for StorageStatusGet {
        type Variables = storage_status_get::Variables;
        type ResponseData = storage_status_get::ResponseData;
        fn build_query(
            variables: Self::Variables,
        ) -> ::graphql_client::QueryBody<Self::Variables> {
            graphql_client::QueryBody {
                variables,
                query: storage_status_get::QUERY,
                operation_name: storage_status_get::OPERATION_NAME,
            }
        }
    }
}

// TODO the internal stuff needs to be renamed
pub fn storage_status_list(
    client: &Client,
    url: &str,
) -> Result<Vec<StorageStatus>, StorageError> {
    let variables = storage_status_get::Variables;
    let response = post_graphql::<storage_status_get::StorageStatusGet, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(StorageError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(storage) = data.storage {
            if let Some(status) = storage.status {
                return Ok(status
                    .into_iter()
                    .filter_map(|x| x)
                    .collect::<Vec<StorageStatus>>());
            }
        }
    }
    Err(classify_response_error::<StorageError>(
        response_body.errors,
    ))
}
