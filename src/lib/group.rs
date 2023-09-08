use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::{
    classify_response_error, Date, UnknownError, Boolean, Int, ResponseStatus,
    classify_response_status_error, KnownErrorCodes,
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

impl KnownErrorCodes for GroupError {
    fn known_error_codes() -> Vec<i64> {
        Vec::new()
    }

    fn is_known_error_code(_code: i64) -> bool {
        false
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

#[derive(Deserialize, Serialize, Debug)]
pub struct PageRuleInput {
    pub id: String,
    pub deny: Boolean,
    pub r#match: PageRuleMatch,
    pub roles: Vec<String>,
    pub path: String,
    pub locales: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
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

pub mod group_create {
    use super::*;

    pub struct GroupCreate;

    pub const OPERATION_NAME: &str = "GroupCreate";
    pub const QUERY : & str = "mutation GroupCreate($name: String!) {\n  groups {\n    create(name: $name) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n      group {\n        id\n        name\n        isSystem\n        redirectOnLogin\n        permissions\n        pageRules\n        users\n        createdAt\n        updatedAt\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub name: String,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub groups: Option<Groups>,
    }

    #[derive(Deserialize)]
    pub struct Groups {
        pub create: Option<GroupResponse>,
    }

    impl graphql_client::GraphQLQuery for GroupCreate {
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

pub fn group_create(
    client: &Client,
    url: &str,
    name: String,
) -> Result<(), GroupError> {
    let variables = group_create::Variables { name };
    let response = post_graphql::<group_create::GroupCreate, _>(client, url, variables);
    if response.is_err() {
        return Err(GroupError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(groups) = data.groups {
            if let Some(create) = groups.create {
                if create.response_result.succeeded {
                    // TODO check that this really does not return a group
                    return Ok(());
                }
                return Err(GroupError::UnknownErrorCode {
                    code: create.response_result.error_code,
                    message: create.response_result.message.unwrap_or(
                        "Unknown error".to_string(),
                    ),
                });
            }
        }
    }
    Err(classify_response_error::<GroupError>(response_body.errors))
}

pub mod group_update {
    use super::*;

    pub struct GroupUpdate;

    pub const OPERATION_NAME: &str = "GroupUpdate";
    pub const QUERY : & str = "mutation GroupUpdate(\n  $id: Int!\n  $name: String!\n  $redirectOnLogin: String!\n  $permissions: [String]!\n  $pageRules: [PageRuleInput]!\n) {\n  groups {\n    update(\n      id: $id\n      name: $name\n      redirectOnLogin: $redirectOnLogin\n      permissions: $permissions\n      pageRules: $pageRules \n    ) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub id: Int,
        pub name: String,
        #[serde(rename = "redirectOnLogin")]
        pub redirect_on_login: String,
        pub permissions: Vec<Option<String>>,
        #[serde(rename = "pageRules")]
        pub page_rules: Vec<Option<PageRuleInput>>,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub groups: Option<Groups>,
    }

    #[derive(Deserialize)]
    pub struct Groups {
        pub update: Option<Update>,
    }

    #[derive(Deserialize)]
    pub struct Update {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for GroupUpdate {
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

pub fn group_update(
    client: &Client,
    url: &str,
    id: Int,
    name: String,
    redirect_on_login: String,
    permissions: Vec<String>,
    page_rules: Vec<PageRuleInput>,
) -> Result<(), GroupError> {
    let variables = group_update::Variables {
        id,
        name,
        redirect_on_login,
        permissions: permissions.into_iter().map(|p| Some(p)).collect(),
        page_rules: page_rules.into_iter().map(|p| Some(p)).collect(),
    };
    let response = post_graphql::<group_update::GroupUpdate, _>(client, url, variables);
    if response.is_err() {
        return Err(GroupError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(groups) = data.groups {
            if let Some(update) = groups.update {
                if let Some(response_result) = update.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(classify_response_status_error::<GroupError>(
                            response_result,
                        ));
                    }
                }
            }
        }
    }
    Err(classify_response_error::<GroupError>(response_body.errors))
}
