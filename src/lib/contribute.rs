use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::{classify_response_error, Date, UnknownError};

#[derive(Clone, Error, Debug, PartialEq)]
pub enum ContributeError {
    #[error("Unknown response error code: {code}: {message}")]
    UnknownErrorCode { code: i64, message: String },
    #[error("Unknown response error: {message}")]
    UnknownErrorMessage { message: String },
    #[error("Unknown response error.")]
    UnknownError,
}

impl From<i64> for ContributeError {
    fn from(code: i64) -> Self {
        ContributeError::UnknownErrorCode {
            code,
            message: "Unknown error".to_string(),
        }
    }
}

impl UnknownError for ContributeError {
    fn unknown_error_code(code: i64, message: String) -> Self {
        ContributeError::UnknownErrorCode { code, message }
    }
    fn unknown_error_message(message: String) -> Self {
        ContributeError::UnknownErrorMessage { message }
    }
    fn unknown_error() -> Self {
        ContributeError::UnknownError
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct Contributor {
    pub id: String,
    pub source: String,
    pub name: String,
    pub joined: Date,
    pub website: Option<String>,
    pub twitter: Option<String>,
    pub avatar: Option<String>,
}

pub mod contributor_list {
    use super::*;

    pub struct ContributorList;

    pub const OPERATION_NAME: &str = "ContributorList";
    pub const QUERY : & str = "query ContributorList {\n  contribute {\n    contributors {\n      id\n      source\n      name\n      joined\n      website\n      twitter\n      avatar\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub contribute: Option<Contribute>,
    }

    #[derive(Deserialize)]
    pub struct Contribute {
        pub contributors: Option<Vec<Option<Contributor>>>,
    }

    impl graphql_client::GraphQLQuery for ContributorList {
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

pub fn contributor_list(
    client: &Client,
    url: &str,
) -> Result<Vec<Contributor>, ContributeError> {
    let variables = contributor_list::Variables {};
    let response = post_graphql::<contributor_list::ContributorList, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(ContributeError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if response_body.data.is_some() {
        let data = response_body.data.unwrap();
        if data.contribute.is_some() {
            let contribute = data.contribute.unwrap();
            return Ok(contribute
                .contributors
                .unwrap()
                .into_iter()
                .flatten()
                .collect());
        }
    }
    Err(classify_response_error(response_body.errors))
}
