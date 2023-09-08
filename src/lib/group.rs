use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::{
    classify_response_error, Date, UnknownError, Boolean, Int, ResponseStatus
};
use crate::user::UserMinimal;

#[derive(Error, Debug, PartialEq)]
pub enum GroupError {
    #[error("Unknown response error code: {code}: {message}")]
    UnknownErrorCode { code: i64, message: String },
    #[error("Unknown response error: {message}")]
    UnknownErrorMessage { message: String },
    #[error("Unknown response error.")]
    UnknownError,
}

impl From<i64> for GroupError {
    fn from(code: i64) -> Self {
        GroupError::UnknownErrorCode {
            code,
            message: "Unknown error".to_string(),
        }
    }
}

impl UnknownError for GroupError {
    fn unknown_error_code(code: i64, message: String) -> Self {
        GroupError::UnknownErrorCode { code, message }
    }
    fn unknown_error_message(message: String) -> Self {
        GroupError::UnknownErrorMessage { message }
    }
    fn unknown_error() -> Self {
        GroupError::UnknownError
    }
}

#[derive(Deserialize, Debug)]
pub struct GroupResponse {
    #[serde(rename = "responseResult")]
    pub response_result: ResponseStatus,
    pub group: Option<Group>,
}

#[derive(Deserialize, Debug)]
pub struct GroupMinimal {
    pub id: Int,
    pub name: String,
    #[serde(rename = "isSystem")]
    pub is_system: Boolean,
    #[serde(rename = "userCount")]
    pub user_count: Option<Int>,
    #[serde(rename = "createdAt")]
    pub created_at: Date,
    #[serde(rename = "updatedAt")]
    pub updated_at: Date,
}

#[derive(Deserialize, Debug)]
pub struct Group {
    pub id: Int,
    pub name: String,
    #[serde(rename = "isSystem")]
    pub is_system: Boolean,
    #[serde(rename = "redirectOnLogin")]
    pub redirect_on_login: Option<String>,
    pub permissions: Vec<String>,
    pub page_rules: Option<Vec<Option<PageRule>>>,
    pub users: Option<Vec<UserMinimal>>,
    #[serde(rename = "createdAt")]
    pub created_at: Date,
    #[serde(rename = "updatedAt")]
    pub updated_at: Date,
}

#[derive(Deserialize, Debug)]
pub struct PageRule {
    pub id: String,
    pub deny: Boolean,
    pub r#match: PageRuleMatch,
    pub roles: Vec<String>,
    pub path: String,
    pub locales: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct PageRuleInput {
    pub id: String,
    pub deny: Boolean,
    pub r#match: PageRuleMatch,
    pub roles: Vec<String>,
    pub path: String,
    pub locales: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub enum PageRuleMatch {
    START,
    EXACT,
    END,
    REGEX,
    TAG,
}

pub mod group_list {
    use super::*;

    pub struct GroupList;

    pub const OPERATION_NAME: &str = "GroupList";
    pub const QUERY : & str = "query GroupList($filter: String, $orderBy: String) {\n  groups {\n    list (filter: $filter, orderBy: $orderBy) {\n      id\n      name\n      isSystem\n      userCount\n      createdAt\n      updatedAt\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub filter: Option<String>,
        #[serde(rename = "orderBy")]
        pub order_by: Option<String>,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub groups: Option<Groups>,
    }

    #[derive(Deserialize)]
    pub struct Groups {
        pub list: Option<Vec<Option<GroupMinimal>>>,
    }

    impl graphql_client::GraphQLQuery for GroupList {
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

pub fn group_list(
    client: &Client,
    url: &str,
    filter: Option<String>,
    order_by: Option<String>,
) -> Result<Vec<GroupMinimal>, GroupError> {
    let variables = group_list::Variables { filter, order_by };
    let response = post_graphql::<group_list::GroupList, _>(client, url, variables);
    if response.is_err() {
        return Err(GroupError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(groups) = data.groups {
            if let Some(list) = groups.list {
                return Ok(list.into_iter().flatten().collect());
            }
        }
    }
    Err(classify_response_error::<GroupError>(response_body.errors))
}

pub mod group_get {
    use super::*;

    pub struct GroupGet;

    pub const OPERATION_NAME: &str = "GroupGet";
    pub const QUERY : & str = "query GroupGet($id: Int!) {\n  groups {\n    single(id: $id) {\n      id\n      name\n      isSystem\n      redirectOnLogin\n      permissions\n      pageRules\n      users\n      createdAt\n      updatedAt\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub id: Int,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub groups: Option<Groups>,
    }

    #[derive(Deserialize)]
    pub struct Groups {
        pub single: Option<Group>,
    }

    impl graphql_client::GraphQLQuery for GroupGet {
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

pub fn group_get(
    client: &Client,
    url: &str,
    id: Int,
) -> Result<Group, GroupError> {
    let variables = group_get::Variables { id };
    let response = post_graphql::<group_get::GroupGet, _>(client, url, variables);
    if response.is_err() {
        return Err(GroupError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(groups) = data.groups {
            if let Some(single) = groups.single {
                return Ok(single);
            }
        }
    }
    Err(classify_response_error::<GroupError>(response_body.errors))
}
