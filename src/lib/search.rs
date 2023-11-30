use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::{
    classify_response_error, classify_response_status_error, Boolean,
    KeyValuePair, KeyValuePairInput, KnownErrorCodes, ResponseStatus,
    UnknownError,
};

#[derive(Clone, Debug, Error, PartialEq)]
pub enum SearchError {
    #[error("An unexpected error occurred during search operation.")]
    SearchGenericError,
    #[error("Search Engine activation failed.")]
    SearchActivationFailed,
    #[error("Unknown response error code: {code}: {message}")]
    UnknownErrorCode { code: i64, message: String },
    #[error("Unknown response error: {message}")]
    UnknownErrorMessage { message: String },
    #[error("Unknown response error.")]
    UnknownError,
}

impl From<i64> for SearchError {
    fn from(code: i64) -> Self {
        match code {
            4001 => SearchError::SearchGenericError,
            4002 => SearchError::SearchActivationFailed,
            _ => SearchError::UnknownErrorCode {
                code,
                message: "Unknown error".to_string(),
            },
        }
    }
}

impl UnknownError for SearchError {
    fn unknown_error_code(code: i64, message: String) -> Self {
        SearchError::UnknownErrorCode { code, message }
    }
    fn unknown_error_message(message: String) -> Self {
        SearchError::UnknownErrorMessage { message }
    }
    fn unknown_error() -> Self {
        SearchError::UnknownError
    }
}

impl KnownErrorCodes for SearchError {
    fn known_error_codes() -> Vec<i64> {
        vec![4001, 4002]
    }

    fn is_known_error_code(code: i64) -> bool {
        (4001..=4002).contains(&code)
    }
}

#[derive(Clone, Deserialize)]
pub struct SearchEngine {
    #[serde(rename = "isEnabled")]
    pub is_enabled: Boolean,
    pub key: String,
    pub title: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub website: Option<String>,
    #[serde(rename = "isAvailable")]
    pub is_available: Option<Boolean>,
    pub config: Option<Vec<Option<KeyValuePair>>>,
}

#[derive(Clone, Serialize)]
pub struct SearchEngineInput {
    #[serde(rename = "isEnabled")]
    pub is_enabled: Boolean,
    pub key: String,
    pub config: Option<Vec<Option<KeyValuePairInput>>>,
}

pub mod search_engine_list {
    use super::*;

    pub struct SearchEngineList;

    pub const OPERATION_NAME: &str = "SearchEngineList";
    pub const QUERY : & str = "query SearchEngineList($filter: String, $orderBy: String) {\n  search {\n    searchEngines(filter: $filter, orderBy: $orderBy) {\n      isEnabled\n      key\n      title\n      description\n      logo\n      website\n      isAvailable\n      config {\n        key\n        value\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub filter: Option<String>,
        #[serde(rename = "orderBy")]
        pub order_by: Option<String>,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub search: Option<SearchEngineListSearch>,
    }

    #[derive(Deserialize)]
    pub struct SearchEngineListSearch {
        #[serde(rename = "searchEngines")]
        pub search_engines: Option<Vec<Option<SearchEngine>>>,
    }

    impl graphql_client::GraphQLQuery for SearchEngineList {
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

pub fn search_engine_list(
    client: &Client,
    url: &str,
    filter: Option<String>,
    order_by: Option<String>,
) -> Result<Vec<SearchEngine>, SearchError> {
    let variables = search_engine_list::Variables { filter, order_by };
    let response = post_graphql::<search_engine_list::SearchEngineList, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(SearchError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(search) = data.search {
            if let Some(search_engines) = search.search_engines {
                return Ok(search_engines.into_iter().flatten().collect());
            }
        }
    }
    Err(classify_response_error::<SearchError>(response_body.errors))
}

pub mod search_engine_index_rebuild {
    use super::*;

    pub struct SearchEngineIndexRebuild;

    pub const OPERATION_NAME: &str = "SearchEngineIndexRebuild";
    pub const QUERY : & str = "mutation SearchEngineIndexRebuild {\n  search {\n    rebuildIndex {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub search: Option<Search>,
    }

    #[derive(Deserialize)]
    pub struct Search {
        #[serde(rename = "rebuildIndex")]
        pub rebuild_index: Option<RebuildIndex>,
    }

    #[derive(Deserialize)]
    pub struct RebuildIndex {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for SearchEngineIndexRebuild {
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

pub fn search_engine_index_rebuild(
    client: &Client,
    url: &str,
) -> Result<(), SearchError> {
    let variables = search_engine_index_rebuild::Variables;
    let response = post_graphql::<
        search_engine_index_rebuild::SearchEngineIndexRebuild,
        _,
    >(client, url, variables);
    if response.is_err() {
        return Err(SearchError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(search) = data.search {
            if let Some(rebuild_index) = search.rebuild_index {
                if let Some(response_result) = rebuild_index.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(classify_response_status_error::<
                            SearchError,
                        >(response_result));
                    }
                }
            }
        }
    }
    Err(classify_response_error::<SearchError>(response_body.errors))
}

pub mod search_engine_update {
    use super::*;

    pub struct SearchEngineUpdate;

    pub const OPERATION_NAME: &str = "SearchEngineUpdate";
    pub const QUERY : & str = "mutation SearchEngineUpdate($engines: [SearchEngineInput]) {\n  search {\n    updateSearchEngines(engines: $engines) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub engines: Option<Vec<Option<SearchEngineInput>>>,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub search: Option<Search>,
    }

    #[derive(Deserialize)]
    pub struct Search {
        #[serde(rename = "updateSearchEngines")]
        pub update_search_engines: Option<UpdateSearchEngines>,
    }

    #[derive(Deserialize)]
    pub struct UpdateSearchEngines {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for SearchEngineUpdate {
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

pub fn search_engine_update(
    client: &Client,
    url: &str,
    engines: Vec<SearchEngineInput>,
) -> Result<(), SearchError> {
    let variables = search_engine_update::Variables {
        engines: Some(engines.into_iter().map(Some).collect()),
    };
    let response = post_graphql::<search_engine_update::SearchEngineUpdate, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(SearchError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(search) = data.search {
            if let Some(update_search_engines) = search.update_search_engines {
                if let Some(response_result) =
                    update_search_engines.response_result
                {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(classify_response_status_error::<
                            SearchError,
                        >(response_result));
                    }
                }
            }
        }
    }
    Err(classify_response_error::<SearchError>(response_body.errors))
}
