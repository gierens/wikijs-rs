use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::{
    classify_response_error, classify_response_status_error, Boolean,
    KnownErrorCodes, ResponseStatus, UnknownError,
};

#[derive(Clone, Error, Debug, PartialEq)]
pub enum ThemeError {
    #[error("Unknown response error code: {code}: {message}")]
    UnknownErrorCode { code: i64, message: String },
    #[error("Unknown response error: {message}")]
    UnknownErrorMessage { message: String },
    #[error("Unknown response error.")]
    UnknownError,
}

impl From<i64> for ThemeError {
    fn from(code: i64) -> Self {
        ThemeError::UnknownErrorCode {
            code,
            message: "Unknown error".to_string(),
        }
    }
}

impl UnknownError for ThemeError {
    fn unknown_error_code(code: i64, message: String) -> Self {
        ThemeError::UnknownErrorCode { code, message }
    }
    fn unknown_error_message(message: String) -> Self {
        ThemeError::UnknownErrorMessage { message }
    }
    fn unknown_error() -> Self {
        ThemeError::UnknownError
    }
}

impl KnownErrorCodes for ThemeError {
    fn known_error_codes() -> Vec<i64> {
        Vec::new()
    }

    fn is_known_error_code(_code: i64) -> bool {
        false
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct Theme {
    pub key: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
}

#[derive(Clone, Deserialize)]
pub struct ThemingConfig {
    pub theme: String,
    pub iconset: String,
    #[serde(rename = "darkMode")]
    pub dark_mode: Boolean,
    #[serde(rename = "tocPosition")]
    pub toc_position: Option<String>,
    #[serde(rename = "injectCSS")]
    pub inject_css: Option<String>,
    #[serde(rename = "injectHead")]
    pub inject_head: Option<String>,
    #[serde(rename = "injectBody")]
    pub inject_body: Option<String>,
}

pub mod theme_list {
    use super::*;

    pub struct ThemeList;

    pub const OPERATION_NAME: &str = "ThemeList";
    pub const QUERY : & str = "query ThemeList {\n  theming {\n    themes {\n      key\n      title\n      author\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub theming: Option<Theming>,
    }

    #[derive(Deserialize)]
    pub struct Theming {
        pub themes: Option<Vec<Option<Theme>>>,
    }

    impl graphql_client::GraphQLQuery for ThemeList {
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

pub fn theme_list(
    client: &Client,
    url: &str,
) -> Result<Vec<Theme>, ThemeError> {
    let variables = theme_list::Variables {};
    let response =
        post_graphql::<theme_list::ThemeList, _>(client, url, variables);
    if response.is_err() {
        return Err(ThemeError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(theming) = data.theming {
            if let Some(themes) = theming.themes {
                return Ok(themes.into_iter().flatten().collect());
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub mod theme_config_get {
    use super::*;

    pub struct ThemeConfigGet;

    pub const OPERATION_NAME: &str = "ThemeConfigGet";
    pub const QUERY : & str = "query ThemeConfigGet {\n  theming {\n    config {\n      theme\n      iconset\n      darkMode\n      tocPosition\n      injectCSS\n      injectHead\n      injectBody\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub theming: Option<Theming>,
    }

    #[derive(Deserialize)]
    pub struct Theming {
        pub config: Option<ThemingConfig>,
    }

    impl graphql_client::GraphQLQuery for ThemeConfigGet {
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

pub fn theme_config_get(
    client: &Client,
    url: &str,
) -> Result<ThemingConfig, ThemeError> {
    let variables = theme_config_get::Variables {};
    let response = post_graphql::<theme_config_get::ThemeConfigGet, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(ThemeError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(theming) = data.theming {
            if let Some(config) = theming.config {
                return Ok(config);
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub mod theme_config_update {
    use super::*;

    pub struct ThemeConfigUpdate;

    pub const OPERATION_NAME: &str = "ThemeConfigUpdate";
    pub const QUERY : & str = "mutation ThemeConfigUpdate (\n  $theme: String!\n  $iconset: String!\n  $darkMode: Boolean!\n  $tocPosition: String\n  $injectCSS: String\n  $injectHead: String\n  $injectBody: String\n) {\n  theming {\n    setConfig (\n      theme: $theme\n      iconset: $iconset\n      darkMode: $darkMode\n      tocPosition: $tocPosition\n      injectCSS: $injectCSS\n      injectHead: $injectHead\n      injectBody: $injectBody\n    ) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub theme: String,
        pub iconset: String,
        #[serde(rename = "darkMode")]
        pub dark_mode: Boolean,
        #[serde(rename = "tocPosition")]
        pub toc_position: Option<String>,
        #[serde(rename = "injectCSS")]
        pub inject_css: Option<String>,
        #[serde(rename = "injectHead")]
        pub inject_head: Option<String>,
        #[serde(rename = "injectBody")]
        pub inject_body: Option<String>,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub theming: Option<Theming>,
    }
    #[derive(Deserialize)]
    pub struct Theming {
        #[serde(rename = "setConfig")]
        pub set_config: Option<SetConfig>,
    }

    #[derive(Deserialize)]
    pub struct SetConfig {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for ThemeConfigUpdate {
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

#[allow(clippy::too_many_arguments)]
pub fn theme_config_update(
    client: &Client,
    url: &str,
    theme: String,
    iconset: String,
    dark_mode: Boolean,
    toc_position: Option<String>,
    inject_css: Option<String>,
    inject_head: Option<String>,
    inject_body: Option<String>,
) -> Result<(), ThemeError> {
    let variables = theme_config_update::Variables {
        theme,
        iconset,
        dark_mode,
        toc_position,
        inject_css,
        inject_head,
        inject_body,
    };
    let response = post_graphql::<theme_config_update::ThemeConfigUpdate, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(ThemeError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(theming) = data.theming {
            if let Some(set_config) = theming.set_config {
                if let Some(response_result) = set_config.response_result {
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
    Err(classify_response_error(response_body.errors))
}
