use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use crate::common::{classify_response_error, Boolean, ResponseStatus};
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
