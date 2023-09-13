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
pub enum RenderingError {
    #[error("Unknown response error code: {code}: {message}")]
    UnknownErrorCode { code: i64, message: String },
    #[error("Unknown response error: {message}")]
    UnknownErrorMessage { message: String },
    #[error("Unknown response error.")]
    UnknownError,
}

impl From<i64> for RenderingError {
    fn from(code: i64) -> Self {
        RenderingError::UnknownErrorCode {
            code,
            message: "Unknown error".to_string(),
        }
    }
}

impl UnknownError for RenderingError {
    fn unknown_error_code(code: i64, message: String) -> Self {
        RenderingError::UnknownErrorCode { code, message }
    }
    fn unknown_error_message(message: String) -> Self {
        RenderingError::UnknownErrorMessage { message }
    }
    fn unknown_error() -> Self {
        RenderingError::UnknownError
    }
}

impl KnownErrorCodes for RenderingError {
    fn known_error_codes() -> Vec<i64> {
        Vec::new()
    }

    fn is_known_error_code(_code: i64) -> bool {
        false
    }
}

#[derive(Deserialize)]
pub struct Renderer {
    #[serde(rename = "isEnabled")]
    pub is_enabled: Boolean,
    pub key: String,
    pub title: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    #[serde(rename = "dependsOn")]
    pub depends_on: Option<String>,
    pub input: Option<String>,
    pub output: Option<String>,
    pub config: Option<Vec<Option<KeyValuePair>>>,
}

#[derive(Serialize)]
pub struct RendererInput {
    #[serde(rename = "isEnabled")]
    pub is_enabled: Boolean,
    pub key: String,
    pub config: Option<Vec<Option<KeyValuePairInput>>>,
}

pub mod renderer_list {
    use super::*;

    pub struct RendererList;

    pub const OPERATION_NAME: &str = "RendererList";
    pub const QUERY : & str = "query RendererList($filter: String, $orderBy: String) {\n  rendering {\n    renderers(filter: $filter, orderBy: $orderBy) {\n      isEnabled\n      key\n      title\n      description\n      icon\n      dependsOn\n      input\n      output\n      config {\n        key\n        value\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub filter: Option<String>,
        #[serde(rename = "orderBy")]
        pub order_by: Option<String>,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub rendering: Option<Rendering>,
    }

    #[derive(Deserialize)]
    pub struct Rendering {
        pub renderers: Option<Vec<Option<Renderer>>>,
    }

    impl graphql_client::GraphQLQuery for RendererList {
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

pub fn renderer_list(
    client: &Client,
    url: &str,
    filter: Option<String>,
    order_by: Option<String>,
) -> Result<Vec<Renderer>, RenderingError> {
    let variables = renderer_list::Variables { filter, order_by };
    let response =
        post_graphql::<renderer_list::RendererList, _>(client, url, variables);
    if response.is_err() {
        return Err(RenderingError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(rendering) = data.rendering {
            if let Some(renderers) = rendering.renderers {
                return Ok(renderers.into_iter().flatten().collect());
            }
        }
    }
    Err(classify_response_error::<RenderingError>(
        response_body.errors,
    ))
}
