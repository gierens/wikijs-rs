use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::{
    classify_response_error, UnknownError,
};

#[derive(Error, Debug, PartialEq)]
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

#[derive(Deserialize, Debug)]
pub struct Theme {
    pub key: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
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
    let response = post_graphql::<theme_list::ThemeList, _>(client, url, variables);
    if response.is_err() {
        return Err(ThemeError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(theming) = data.theming {
            if let Some(themes) = theming.themes {
                return Ok(themes
                    .into_iter()
                    .flatten()
                    .collect());
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}
