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
pub enum AnalyticsError {
    #[error("Unknown response error code: {code}: {message}")]
    UnknownErrorCode { code: i64, message: String },
    #[error("Unknown response error: {message}")]
    UnknownErrorMessage { message: String },
    #[error("Unknown response error.")]
    UnknownError,
}

impl From<i64> for AnalyticsError {
    fn from(code: i64) -> Self {
        AnalyticsError::UnknownErrorCode {
            code,
            message: "Unknown error".to_string(),
        }
    }
}

impl UnknownError for AnalyticsError {
    fn unknown_error_code(code: i64, message: String) -> Self {
        AnalyticsError::UnknownErrorCode { code, message }
    }
    fn unknown_error_message(message: String) -> Self {
        AnalyticsError::UnknownErrorMessage { message }
    }
    fn unknown_error() -> Self {
        AnalyticsError::UnknownError
    }
}

impl KnownErrorCodes for AnalyticsError {
    fn known_error_codes() -> Vec<i64> {
        Vec::new()
    }

    fn is_known_error_code(_code: i64) -> bool {
        false
    }
}

#[derive(Deserialize, Debug)]
pub struct AnalyticsProvider {
    #[serde(rename = "isEnabled")]
    pub is_enabled: Boolean,
    pub key: String,
    pub props: Option<Vec<Option<String>>>,
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "isAvailable")]
    pub is_available: Option<Boolean>,
    pub logo: Option<String>,
    pub website: Option<String>,
    pub config: Option<Vec<Option<KeyValuePair>>>,
}

#[derive(Serialize, Debug)]
pub struct AnalyticsProviderInput {
    #[serde(rename = "isEnabled")]
    pub is_enabled: Boolean,
    pub key: String,
    pub config: Option<Vec<Option<KeyValuePairInput>>>,
}

pub mod analytics_provider_list {
    use super::*;

    pub struct AnalyticsProviderList;

    pub const OPERATION_NAME: &str = "AnalyticsProviderList";
    pub const QUERY : & str = "query AnalyticsProviderList {\n  analytics {\n    providers {\n      isEnabled\n      key\n      props\n      title\n      description\n      isAvailable\n      logo\n      website\n      config {\n        key,\n        value\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub analytics: Option<Analytics>,
    }

    #[derive(Deserialize)]
    pub struct Analytics {
        pub providers: Option<Vec<Option<AnalyticsProvider>>>,
    }

    impl graphql_client::GraphQLQuery for AnalyticsProviderList {
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

pub fn analytics_provider_list(
    client: &Client,
    url: &str,
) -> Result<Vec<AnalyticsProvider>, AnalyticsError> {
    let variables = analytics_provider_list::Variables {};
    let response = post_graphql::<
        analytics_provider_list::AnalyticsProviderList,
        _,
    >(client, url, variables);
    if response.is_err() {
        return Err(AnalyticsError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(analytics) = data.analytics {
            if let Some(providers) = analytics.providers {
                return Ok(providers.into_iter().flatten().collect());
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub mod analytics_provider_update {
    use super::*;

    pub struct AnalyticsProviderUpdate;

    pub const OPERATION_NAME: &str = "AnalyticsProviderUpdate";
    pub const QUERY : & str = "mutation AnalyticsProviderUpdate($providers: [AnalyticsProviderInput]!) {\n  analytics {\n    updateProviders(providers: $providers) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub providers: Vec<Option<AnalyticsProviderInput>>,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub analytics: Option<Analytics>,
    }

    #[derive(Deserialize)]
    pub struct Analytics {
        #[serde(rename = "updateProviders")]
        pub update_providers: Option<UpdateProviders>,
    }

    #[derive(Deserialize)]
    pub struct UpdateProviders {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for AnalyticsProviderUpdate {
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

pub fn analytics_provider_update(
    client: &Client,
    url: &str,
    providers: Vec<AnalyticsProviderInput>,
) -> Result<(), AnalyticsError> {
    let variables = analytics_provider_update::Variables {
        providers: providers.into_iter().map(|p| Some(p)).collect(),
    };
    let response = post_graphql::<
        analytics_provider_update::AnalyticsProviderUpdate,
        _,
    >(client, url, variables);
    if response.is_err() {
        return Err(AnalyticsError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(analytics) = data.analytics {
            if let Some(update_providers) = analytics.update_providers {
                if let Some(response_result) = update_providers.response_result
                {
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
