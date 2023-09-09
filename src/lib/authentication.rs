use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use crate::common::{classify_response_error, Boolean, Date, Int, ResponseStatus};
use crate::user::UserError;

#[derive(Deserialize, Debug)]
pub struct AuthenticationLoginResponse {
    #[serde(rename = "responseResult")]
    pub response_result: Option<ResponseStatus>,
    pub jwt: Option<String>,
    #[serde(rename = "mustChangePwd")]
    pub must_change_pwd: Option<Boolean>,
    #[serde(rename = "mustProvideTFA")]
    pub must_provide_tfa: Option<Boolean>,
    #[serde(rename = "mustSetupTFA")]
    pub must_setup_tfa: Option<Boolean>,
    #[serde(rename = "continuationToken")]
    pub continuation_token: Option<String>,
    pub redirect: Option<String>,
    #[serde(rename = "tfaQRImage")]
    pub tfa_qr_image: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ApiKey {
    pub id: Int,
    pub name: String,
    #[serde(rename = "keyShort")]
    pub key_short: String,
    pub expiration: Date,
    #[serde(rename = "createdAt")]
    pub created_at: Date,
    #[serde(rename = "updatedAt")]
    pub updated_at: Date,
    #[serde(rename = "isRevoked")]
    pub is_revoked: Boolean,
}

pub(crate) mod login_mod {
    use super::*;

    pub struct Login;

    pub const OPERATION_NAME: &str = "Login";
    pub const QUERY : & str = "mutation Login($username: String!, $password: String!, $strategy: String!) {\n  authentication {\n    login(username: $username, password: $password, strategy: $strategy) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n      jwt\n      mustChangePwd\n      mustProvideTFA\n      mustSetupTFA\n      continuationToken\n      redirect\n      tfaQRImage\n    }\n  }\n}\n" ;

    #[derive(Serialize, Debug)]
    pub struct Variables {
        pub username: String,
        pub password: String,
        pub strategy: String,
    }

    impl Variables {}

    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub authentication: Option<Authentication>,
    }

    #[derive(Deserialize, Debug)]
    pub struct Authentication {
        pub login: Option<AuthenticationLoginResponse>,
    }

    impl graphql_client::GraphQLQuery for Login {
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

pub fn login(
    client: &Client,
    url: &str,
    username: String,
    password: String,
    strategy: String,
) -> Result<AuthenticationLoginResponse, UserError> {
    let variables = login_mod::Variables {
        username,
        password,
        strategy,
    };
    let response = post_graphql::<login_mod::Login, _>(client, url, variables);
    if response.is_err() {
        return Err(UserError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(authentication) = data.authentication {
            if let Some(login) = authentication.login {
                return Ok(login);
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub mod api_key_list {
    use super::*;

    pub struct ApiKeyList;

    pub const OPERATION_NAME: &str = "ApiKeyList";
    pub const QUERY : & str = "query ApiKeyList {\n  authentication {\n    apiKeys {\n      id\n      name\n      keyShort\n      expiration\n      createdAt\n      updatedAt\n      isRevoked\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub authentication: Option<Authentication>,
    }

    #[derive(Deserialize)]
    pub struct Authentication {
        #[serde(rename = "apiKeys")]
        pub api_keys: Option<Vec<Option<ApiKey>>>,
    }

    impl graphql_client::GraphQLQuery for ApiKeyList {
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

pub fn api_key_list(client: &Client, url: &str) -> Result<Vec<ApiKey>, UserError> {
    let variables = api_key_list::Variables {};
    let response = post_graphql::<api_key_list::ApiKeyList, _>(client, url, variables);
    if response.is_err() {
        return Err(UserError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(authentication) = data.authentication {
            if let Some(api_keys) = authentication.api_keys {
                return Ok(api_keys.into_iter().flatten().collect());
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}
