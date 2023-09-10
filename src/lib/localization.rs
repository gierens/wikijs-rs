use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::{
    classify_response_error, Date, Int, KnownErrorCodes, UnknownError, Boolean,
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

#[derive(Deserialize, Debug)]
pub struct Locale {
    pub availability: Int,
    pub code: String,
    #[serde(rename = "createdAt")]
    pub created_at: Date,
    #[serde(rename = "installDate")]
    pub install_date: Option<Date>,
    #[serde(rename = "isInstalled")]
    pub is_installed: Boolean,
    #[serde(rename = "isRTL")]
    pub is_rtl: Boolean,
    pub name: String,
    #[serde(rename = "nativeName")]
    pub native_name: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: Date,
}

#[derive(Deserialize, Debug)]
pub struct LocaleConfig {
    pub locale: String,
    #[serde(rename = "autoUpdate")]
    pub auto_update: Boolean,
    pub namespacing: Boolean,
    pub namespaces: Vec<Option<String>>,
}

pub mod locale_list {
    use super::*;

    pub struct LocaleList;

    pub const OPERATION_NAME: &str = "LocaleList";
    pub const QUERY : & str = "query LocaleList {\n  localization {\n    locales {\n      availability\n      code\n      createdAt\n      installDate\n      isInstalled\n      isRTL\n      name\n      nativeName\n      updatedAt\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub localization: Option<Localization>,
    }

    #[derive(Deserialize)]
    pub struct Localization {
        pub locales: Option<Vec<Option<Locale>>>,
    }

    impl graphql_client::GraphQLQuery for LocaleList {
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

pub fn locale_list(
    client: &Client,
    url: &str,
) -> Result<Vec<Locale>, LocaleError> {
    let variables = locale_list::Variables {};
    let response = post_graphql::<locale_list::LocaleList, _>(client, url, variables);
    if response.is_err() {
        return Err(LocaleError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(localization) = data.localization {
            if let Some(locales) = localization.locales {
                return Ok(locales
                    .into_iter()
                    .flatten()
                    .collect());
            }
        }
    }
    Err(classify_response_error::<LocaleError>(
        response_body.errors,
    ))
}

// TODO the corresponding query file should be renamed as well
pub mod locale_config_get {
    use super::*;

    pub struct LocaleConfigGet;

    pub const OPERATION_NAME: &str = "LocaleConfigGet";
    pub const QUERY : & str = "query LocaleConfigGet {\n  localization {\n    config {\n      locale\n      autoUpdate\n      namespacing\n      namespaces\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub localization: Option<Localization>,
    }

    #[derive(Deserialize)]
    pub struct Localization {
        pub config: Option<LocaleConfig>,
    }

    impl graphql_client::GraphQLQuery for LocaleConfigGet {
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

pub fn locale_config_get(
    client: &Client,
    url: &str,
) -> Result<LocaleConfig, LocaleError> {
    let variables = locale_config_get::Variables {};
    let response = post_graphql::<locale_config_get::LocaleConfigGet, _>(client, url, variables);
    if response.is_err() {
        return Err(LocaleError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(localization) = data.localization {
            if let Some(config) = localization.config {
                return Ok(config);
            }
        }
    }
    Err(classify_response_error::<LocaleError>(
        response_body.errors,
    ))
}
