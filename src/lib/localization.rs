use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::{
    classify_response_error, classify_response_status_error, Boolean, Date,
    Int, KnownErrorCodes, ResponseStatus, UnknownError,
};

#[derive(Clone, Debug, Error, PartialEq)]
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

#[derive(Clone, Deserialize, Debug)]
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

#[derive(Clone, Deserialize, Debug)]
pub struct LocaleConfig {
    pub locale: String,
    #[serde(rename = "autoUpdate")]
    pub auto_update: Boolean,
    pub namespacing: Boolean,
    pub namespaces: Vec<Option<String>>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Translation {
    pub key: String,
    pub value: String,
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
    let response =
        post_graphql::<locale_list::LocaleList, _>(client, url, variables);
    if response.is_err() {
        return Err(LocaleError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(localization) = data.localization {
            if let Some(locales) = localization.locales {
                return Ok(locales.into_iter().flatten().collect());
            }
        }
    }
    Err(classify_response_error::<LocaleError>(response_body.errors))
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
    let response = post_graphql::<locale_config_get::LocaleConfigGet, _>(
        client, url, variables,
    );
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
    Err(classify_response_error::<LocaleError>(response_body.errors))
}

pub mod translation_list {
    use super::*;

    pub struct TranslationList;

    pub const OPERATION_NAME: &str = "TranslationList";
    pub const QUERY : & str = "query TranslationList(\n  $locale: String!\n  $namespace: String!\n) {\n  localization {\n    translations(\n      locale: $locale\n      namespace: $namespace\n    ) {\n      key\n      value\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub locale: String,
        pub namespace: String,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub localization: Option<Localization>,
    }
    #[derive(Deserialize)]
    pub struct Localization {
        pub translations: Option<Vec<Option<Translation>>>,
    }

    impl graphql_client::GraphQLQuery for TranslationList {
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

pub fn translation_list(
    client: &Client,
    url: &str,
    locale: String,
    namespace: String,
) -> Result<Vec<Translation>, LocaleError> {
    let variables = translation_list::Variables { locale, namespace };
    let response = post_graphql::<translation_list::TranslationList, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(LocaleError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(localization) = data.localization {
            if let Some(translations) = localization.translations {
                return Ok(translations.into_iter().flatten().collect());
            }
        }
    }
    Err(classify_response_error::<LocaleError>(response_body.errors))
}

pub mod locale_download {
    use super::*;

    pub struct LocaleDownload;

    pub const OPERATION_NAME: &str = "LocaleDownload";
    pub const QUERY : & str = "mutation LocaleDownload(\n  $locale: String!\n) {\n  localization {\n    downloadLocale(\n      locale: $locale\n    ) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub locale: String,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub localization: Option<Localization>,
    }

    #[derive(Deserialize)]
    pub struct Localization {
        #[serde(rename = "downloadLocale")]
        pub download_locale: Option<DownloadLocale>,
    }

    #[derive(Deserialize)]
    pub struct DownloadLocale {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for LocaleDownload {
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

pub fn locale_download(
    client: &Client,
    url: &str,
    locale: String,
) -> Result<(), LocaleError> {
    let variables = locale_download::Variables { locale };
    let response = post_graphql::<locale_download::LocaleDownload, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(LocaleError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(localization) = data.localization {
            if let Some(download_locale) = localization.download_locale {
                if let Some(response_result) = download_locale.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(classify_response_status_error::<
                            LocaleError,
                        >(response_result));
                    }
                }
            }
        }
    }
    Err(classify_response_error::<LocaleError>(response_body.errors))
}

pub mod locale_update {
    use super::*;

    pub struct LocaleUpdate;

    pub const OPERATION_NAME: &str = "LocaleUpdate";
    pub const QUERY : & str = "mutation LocaleUpdate(\n  $locale: String!\n  $autoUpdate: Boolean!\n  $namespacing: Boolean!\n  $namespaces: [String]!\n) {\n  localization {\n    updateLocale(\n      locale: $locale\n      autoUpdate: $autoUpdate\n      namespacing: $namespacing\n      namespaces: $namespaces\n    ) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub locale: String,
        #[serde(rename = "autoUpdate")]
        pub auto_update: Boolean,
        pub namespacing: Boolean,
        pub namespaces: Vec<Option<String>>,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub localization: Option<Localization>,
    }

    #[derive(Deserialize)]
    pub struct Localization {
        #[serde(rename = "updateLocale")]
        pub update_locale: Option<UpdateLocale>,
    }

    #[derive(Deserialize)]
    pub struct UpdateLocale {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for LocaleUpdate {
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

pub fn locale_update(
    client: &Client,
    url: &str,
    locale: String,
    auto_update: bool,
    namespacing: bool,
    namespaces: Vec<String>,
) -> Result<(), LocaleError> {
    let variables = locale_update::Variables {
        locale,
        auto_update,
        namespacing,
        namespaces: namespaces.into_iter().map(Some).collect(),
    };
    let response =
        post_graphql::<locale_update::LocaleUpdate, _>(client, url, variables);
    if response.is_err() {
        return Err(LocaleError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(localization) = data.localization {
            if let Some(update_locale) = localization.update_locale {
                if let Some(response_result) = update_locale.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(classify_response_status_error::<
                            LocaleError,
                        >(response_result));
                    }
                }
            }
        }
    }
    Err(classify_response_error::<LocaleError>(response_body.errors))
}
