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

#[derive(Deserialize, Debug)]
pub enum NavigationMode {
    NONE,
    TREE,
    MIXED,
    STATIC,
}

#[derive(Deserialize, Debug)]
pub struct NavigationConfig {
    pub mode: NavigationMode,
}

pub mod navigation_config_get {
    use super::*;

    pub struct NavigationConfigGet;

    pub const OPERATION_NAME: &str = "NavigationConfigGet";
    pub const QUERY : & str = "query NavigationConfigGet {\n  navigation {\n    config {\n      mode\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub navigation: Option<Navigation>,
    }

    #[derive(Deserialize)]
    pub struct Navigation {
        pub config: NavigationConfig,
    }

    impl graphql_client::GraphQLQuery for NavigationConfigGet {
        type Variables = Variables;
        type ResponseData = ResponseData;

        fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
            graphql_client::QueryBody {
                variables,
                query: QUERY,
                operation_name: OPERATION_NAME,
            }
        }
    }
}

pub fn navigation_config_get(
    client: &Client,
    url: &str,
) -> Result<NavigationConfig, NavigationError> {
    let variables = navigation_config_get::Variables {};
    let response = post_graphql::<navigation_config_get::NavigationConfigGet, _>(
        client,
        url,
        variables,
    );
    if response.is_err() {
        return Err(NavigationError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(navigation) = data.navigation {
            return Ok(navigation.config);
        }
    }
    Err(classify_response_error::<NavigationError>(response_body.errors))
}
