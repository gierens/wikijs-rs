use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::{
    classify_response_error, Date, Int, KnownErrorCodes, UnknownError,
    Boolean, KeyValuePair, KeyValuePairInput, ResponseStatus,
    classify_response_status_error
};

#[derive(Debug, Error, PartialEq)]
pub enum CommentError {
    #[error("An unexpected error occurred.")]
    CommentGenericError,
    #[error("You are not authorized to post a comment on this page.")]
    CommentPostForbidden,
    #[error("Comment content is missing or too short.")]
    CommentContentMissing,
    #[error("You are not authorized to manage comments on this page.")]
    CommentManageForbidden,
    #[error("This comment does not exist.")]
    CommentNotFound,
    #[error("You are not authorized to view comments for this page.")]
    CommentViewForbidden,
    #[error("Unknown response error code: {code}: {message}")]
    UnknownErrorCode { code: i64, message: String },
    #[error("Unknown response error: {message}")]
    UnknownErrorMessage { message: String },
    #[error("Unknown response error.")]
    UnknownError,
}

impl From<i64> for CommentError {
    fn from(code: i64) -> Self {
        match code {
            8001 => CommentError::CommentGenericError,
            8002 => CommentError::CommentPostForbidden,
            8003 => CommentError::CommentContentMissing,
            8004 => CommentError::CommentManageForbidden,
            8005 => CommentError::CommentNotFound,
            8006 => CommentError::CommentViewForbidden,
            _ => CommentError::UnknownErrorCode {
                code,
                message: "Unknown error".to_string(),
            },
        }
    }
}

impl UnknownError for CommentError {
    fn unknown_error_code(code: i64, message: String) -> Self {
        CommentError::UnknownErrorCode { code, message }
    }
    fn unknown_error_message(message: String) -> Self {
        CommentError::UnknownErrorMessage { message }
    }
    fn unknown_error() -> Self {
        CommentError::UnknownError
    }
}

impl KnownErrorCodes for CommentError {
    fn known_error_codes() -> Vec<i64> {
        vec![8001, 8002, 8003, 8004, 8005, 8006]
    }

    fn is_known_error_code(code: i64) -> bool {
        (8001..=8006).contains(&code)
    }
}

#[derive(Deserialize)]
pub struct Comment {
    pub id: Int,
    pub content: String,
    pub render: String,
    #[serde(rename = "authorId")]
    pub author_id: Int,
    #[serde(rename = "authorName")]
    pub author_name: String,
    #[serde(rename = "authorEmail")]
    pub author_email: String,
    #[serde(rename = "authorIP")]
    pub author_ip: String,
    #[serde(rename = "createdAt")]
    pub created_at: Date,
    #[serde(rename = "updatedAt")]
    pub updated_at: Date,
}

#[derive(Deserialize, Debug)]
pub struct CommentProvider {
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

#[derive(Serialize, Debug)]
pub struct CommentProviderInput {
    #[serde(rename = "isEnabled")]
    pub is_enabled: Boolean,
    pub key: String,
    pub config: Option<Vec<Option<KeyValuePairInput>>>,
}

#[derive(Deserialize, Debug)]
pub struct CommentCreateResponse {
    #[serde(rename = "responseResult")]
    pub response_result: Option<ResponseStatus>,
    pub id: Option<Int>,
}

pub mod comment_list {
    use super::*;

    pub struct CommentList;

    pub const OPERATION_NAME: &str = "CommentList";
    pub const QUERY : & str = "query CommentList($locale: String!, $path: String!) {\n  comments {\n    list(locale: $locale, path: $path) {\n      id\n      content\n      render\n      authorId\n      authorName\n      authorEmail\n      authorIP\n      createdAt\n      updatedAt\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub locale: String,
        pub path: String,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub comments: Option<Comments>,
    }

    #[derive(Deserialize)]
    pub struct Comments {
        pub list: Vec<Option<Comment>>,
    }

    impl graphql_client::GraphQLQuery for CommentList {
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

pub fn comment_list(
    client: &Client,
    url: &str,
    locale: String,
    path: String,
) -> Result<Vec<Comment>, CommentError> {
    let variables = comment_list::Variables { locale, path };
    let response =
        post_graphql::<comment_list::CommentList, _>(client, url, variables);
    if response.is_err() {
        return Err(CommentError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(comments) = data.comments {
            return Ok(comments.list.into_iter().flatten().collect());
        }
    }
    Err(classify_response_error::<CommentError>(
        response_body.errors,
    ))
}

pub mod comment_provider_list {
    use super::*;

    pub struct CommentProviderList;

    pub const OPERATION_NAME: &str = "CommentProviderList";
    pub const QUERY : & str = "query CommentProviderList {\n  comments {\n    providers {\n      isEnabled\n      key\n      title\n      description\n      logo\n      website\n      isAvailable\n      config {\n        key\n        value\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub comments: Option<Comments>,
    }

    #[derive(Deserialize)]
    pub struct Comments {
        pub providers: Option<Vec<Option<CommentProvider>>>,
    }

    impl graphql_client::GraphQLQuery for CommentProviderList {
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

pub fn comment_provider_list(
    client: &Client,
    url: &str,
) -> Result<Vec<CommentProvider>, CommentError> {
    let variables = comment_provider_list::Variables {};
    let response =
        post_graphql::<comment_provider_list::CommentProviderList, _>(
            client, url, variables,
        );
    if response.is_err() {
        return Err(CommentError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(comments) = data.comments {
            if let Some(providers) = comments.providers {
                return Ok(providers.into_iter().flatten().collect());
            }
        }
    }
    Err(classify_response_error::<CommentError>(
        response_body.errors,
    ))
}

pub mod comment_get {
    use super::*;

    pub struct CommentGet;

    pub const OPERATION_NAME: &str = "CommentGet";
    pub const QUERY : & str = "query CommentGet($id: Int!) {\n  comments {\n    single (id: $id) {\n      id\n      content\n      render\n      authorId\n      authorName\n      authorEmail\n      authorIP\n      createdAt\n      updatedAt\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub id: Int,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub comments: Option<Comments>,
    }

    #[derive(Deserialize)]
    pub struct Comments {
        pub single: Option<Comment>,
    }

    impl graphql_client::GraphQLQuery for CommentGet {
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

pub fn comment_get(
    client: &Client,
    url: &str,
    id: Int,
) -> Result<Comment, CommentError> {
    let variables = comment_get::Variables { id };
    let response =
        post_graphql::<comment_get::CommentGet, _>(client, url, variables);
    if response.is_err() {
        return Err(CommentError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(comments) = data.comments {
            if let Some(comment) = comments.single {
                return Ok(comment);
            }
        }
    }
    Err(classify_response_error::<CommentError>(
        response_body.errors,
    ))
}

pub mod comment_provider_update {
    use super::*;

    pub struct CommentProviderUpdate;

    pub const OPERATION_NAME: &str = "CommentProviderUpdate";
    pub const QUERY : & str = "mutation CommentProviderUpdate($providers: [CommentProviderInput]) {\n  comments {\n    updateProviders(providers: $providers) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub providers: Option<Vec<Option<CommentProviderInput>>>,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub comments: Option<Comments>,
    }

    #[derive(Deserialize)]
    pub struct Comments {
        #[serde(rename = "updateProviders")]
        pub update_providers: Option<UpdateProviders>,
    }

    #[derive(Deserialize)]
    pub struct UpdateProviders {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for CommentProviderUpdate {
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

pub fn comment_provider_update(
    client: &Client,
    url: &str,
    providers: Vec<CommentProviderInput>,
) -> Result<(), CommentError> {
    let variables = comment_provider_update::Variables {
        providers: Some(providers.into_iter().map(Some).collect()),
    };
    let response = post_graphql::<
        comment_provider_update::CommentProviderUpdate,
        _,
    >(client, url, variables);
    if response.is_err() {
        return Err(CommentError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(comments) = data.comments {
            if let Some(update_providers) = comments.update_providers {
                if let Some(response_result) = update_providers.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(classify_response_status_error::<
                            CommentError,
                        >(response_result));
                    }
                }
            }
        }
    }
    Err(classify_response_error::<CommentError>(
        response_body.errors,
    ))
}

pub mod comment_create {
    use super::*;

    pub struct CommentCreate;

    pub const OPERATION_NAME: &str = "CommentCreate";
    pub const QUERY : & str = "mutation CommentCreate(\n  $pageId: Int!\n  $replyTo: Int\n  $content: String!\n  $guestName: String\n  $guestEmail: String\n) {\n  comments {\n    create (\n      pageId: $pageId\n      replyTo: $replyTo\n      content: $content\n      guestName: $guestName\n      guestEmail: $guestEmail\n    ) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n      id\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "pageId")]
        pub page_id: Int,
        #[serde(rename = "replyTo")]
        pub reply_to: Option<Int>,
        pub content: String,
        #[serde(rename = "guestName")]
        pub guest_name: Option<String>,
        #[serde(rename = "guestEmail")]
        pub guest_email: Option<String>,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub comments: Option<Comments>,
    }

    #[derive(Deserialize)]
    pub struct Comments {
        pub create: Option<CommentCreateResponse>,
    }

    impl graphql_client::GraphQLQuery for CommentCreate {
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

pub fn comment_create(
    client: &Client,
    url: &str,
    page_id: Int,
    reply_to: Option<Int>,
    content: String,
    guest_name: Option<String>,
    guest_email: Option<String>,
) -> Result<(), CommentError> {
    let variables = comment_create::Variables {
        page_id,
        reply_to,
        content,
        guest_name,
        guest_email,
    };
    let response =
        post_graphql::<comment_create::CommentCreate, _>(client, url, variables);
    if response.is_err() {
        return Err(CommentError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(comments) = data.comments {
            if let Some(create) = comments.create {
                if let Some(response_result) = create.response_result {
                    if response_result.succeeded {
                        // TODO check if this does really not return an id
                        return Ok(());
                    } else {
                        return Err(classify_response_status_error::<
                            CommentError,
                        >(response_result));
                    }
                }
            }
        }
    }
    Err(classify_response_error::<CommentError>(
        response_body.errors,
    ))
}
