use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::{
    classify_response_error, Date, Int, KnownErrorCodes, UnknownError,
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
        (6001..=6013).contains(&code)
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
