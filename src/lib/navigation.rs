use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::{
    classify_response_error, classify_response_status_error, Int,
    KnownErrorCodes, ResponseStatus, UnknownError,
};

#[derive(Clone, Error, Debug, PartialEq)]
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

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum NavigationMode {
    NONE,
    TREE,
    MIXED,
    STATIC,
}

#[derive(Clone, Deserialize, Debug)]
pub struct NavigationConfig {
    pub mode: NavigationMode,
}

#[derive(Clone, Deserialize, Debug)]
pub struct NavigationTree {
    pub locale: String,
    pub items: Vec<Option<NavigationTreeItem>>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct NavigationTreeItem {
    pub id: String,
    pub kind: String,
    pub label: Option<String>,
    pub icon: Option<String>,
    #[serde(rename = "targetType")]
    pub target_type: Option<String>,
    pub target: Option<String>,
    #[serde(rename = "visibilityMode")]
    pub visibility_mode: Option<String>,
    #[serde(rename = "visibilityGroups")]
    pub visibility_groups: Option<Vec<Option<Int>>>,
}

#[derive(Clone, Serialize, Debug)]
pub struct NavigationTreeInput {
    pub locale: String,
    pub items: Vec<Option<NavigationItemInput>>,
}

#[derive(Clone, Serialize, Debug)]
pub struct NavigationItemInput {
    pub id: String,
    pub kind: String,
    pub label: Option<String>,
    pub icon: Option<String>,
    #[serde(rename = "targetType")]
    pub target_type: Option<String>,
    pub target: Option<String>,
    #[serde(rename = "visibilityMode")]
    pub visibility_mode: Option<String>,
    #[serde(rename = "visibilityGroups")]
    pub visibility_groups: Option<Vec<Option<Int>>>,
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

        fn build_query(
            variables: Self::Variables,
        ) -> graphql_client::QueryBody<Self::Variables> {
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
        client, url, variables,
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
    Err(classify_response_error::<NavigationError>(
        response_body.errors,
    ))
}

pub mod navigation_tree_get {
    use super::*;

    pub struct NavigationTreeGet;

    pub const OPERATION_NAME: &str = "NavigationTreeGet";
    pub const QUERY : & str = "query NavigationTreeGet {\n  navigation {\n    tree {\n      locale\n      items {\n        id\n        kind\n        label\n        icon\n        targetType\n        target\n        visibilityMode\n        visibilityGroups\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub navigation: Option<Navigation>,
    }

    #[derive(Deserialize)]
    pub struct Navigation {
        pub tree: Vec<Option<NavigationTree>>,
    }

    impl graphql_client::GraphQLQuery for NavigationTreeGet {
        type Variables = Variables;
        type ResponseData = ResponseData;

        fn build_query(
            variables: Self::Variables,
        ) -> graphql_client::QueryBody<Self::Variables> {
            graphql_client::QueryBody {
                variables,
                query: QUERY,
                operation_name: OPERATION_NAME,
            }
        }
    }
}

pub fn navigation_tree_get(
    client: &Client,
    url: &str,
) -> Result<Vec<NavigationTree>, NavigationError> {
    let variables = navigation_tree_get::Variables {};
    let response = post_graphql::<navigation_tree_get::NavigationTreeGet, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(NavigationError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(navigation) = data.navigation {
            return Ok(navigation.tree.into_iter().flatten().collect());
        }
    }
    Err(classify_response_error::<NavigationError>(
        response_body.errors,
    ))
}

pub mod navigation_config_update {
    use super::*;

    pub struct NavigationConfigUpdate;

    pub const OPERATION_NAME: &str = "NavigationConfigUpdate";
    pub const QUERY : & str = "mutation NavigationConfigUpdate (\n  $mode: NavigationMode!\n) { \n  navigation {\n    updateConfig(mode: $mode) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub mode: NavigationMode,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub navigation: Option<Navigation>,
    }

    #[derive(Deserialize)]
    pub struct Navigation {
        #[serde(rename = "updateConfig")]
        pub update_config: Option<UpdateConfig>,
    }
    #[derive(Deserialize)]
    pub struct UpdateConfig {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for NavigationConfigUpdate {
        type Variables = Variables;
        type ResponseData = ResponseData;

        fn build_query(
            variables: Self::Variables,
        ) -> graphql_client::QueryBody<Self::Variables> {
            graphql_client::QueryBody {
                variables,
                query: QUERY,
                operation_name: OPERATION_NAME,
            }
        }
    }
}

pub fn navigation_config_update(
    client: &Client,
    url: &str,
    mode: NavigationMode,
) -> Result<(), NavigationError> {
    let variables = navigation_config_update::Variables { mode };
    let response = post_graphql::<
        navigation_config_update::NavigationConfigUpdate,
        _,
    >(client, url, variables);
    if response.is_err() {
        return Err(NavigationError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(navigation) = data.navigation {
            if let Some(update_config) = navigation.update_config {
                if let Some(response_result) = update_config.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(classify_response_status_error::<
                            NavigationError,
                        >(response_result));
                    }
                }
            }
        }
    }
    Err(classify_response_error::<NavigationError>(
        response_body.errors,
    ))
}

pub mod navigation_tree_update {
    use super::*;

    pub struct NavigationTreeUpdate;

    pub const OPERATION_NAME: &str = "NavigationTreeUpdate";
    pub const QUERY : & str = "mutation NavigationTreeUpdate (\n  $tree: [NavigationTreeInput]!\n) { \n  navigation {\n    updateTree(tree: $tree) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub tree: Vec<Option<NavigationTreeInput>>,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub navigation: Option<Navigation>,
    }

    #[derive(Deserialize)]
    pub struct Navigation {
        #[serde(rename = "updateTree")]
        pub update_tree: Option<UpdateTree>,
    }
    #[derive(Deserialize)]
    pub struct UpdateTree {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for NavigationTreeUpdate {
        type Variables = Variables;
        type ResponseData = ResponseData;

        fn build_query(
            variables: Self::Variables,
        ) -> graphql_client::QueryBody<Self::Variables> {
            graphql_client::QueryBody {
                variables,
                query: QUERY,
                operation_name: OPERATION_NAME,
            }
        }
    }
}

pub fn navigation_tree_update(
    client: &Client,
    url: &str,
    tree: Vec<NavigationTreeInput>,
) -> Result<(), NavigationError> {
    let variables = navigation_tree_update::Variables {
        tree: tree.into_iter().map(Some).collect(),
    };
    let response = post_graphql::<
        navigation_tree_update::NavigationTreeUpdate,
        _,
    >(client, url, variables);
    if response.is_err() {
        return Err(NavigationError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(navigation) = data.navigation {
            if let Some(update_tree) = navigation.update_tree {
                if let Some(response_result) = update_tree.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(classify_response_status_error::<
                            NavigationError,
                        >(response_result));
                    }
                }
            }
        }
    }
    Err(classify_response_error::<NavigationError>(
        response_body.errors,
    ))
}
