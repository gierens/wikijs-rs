use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use crate::common::{classify_response_error, Boolean, Date, Int, ResponseStatus, KeyValuePair, classify_response_status_error};
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

#[derive(Deserialize, Debug)]
pub struct AuthenticationStrategy {
    pub key: String,
    pub props: Option<Vec<Option<KeyValuePair>>>,
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "isAvailable")]
    pub is_available: Option<Boolean>,
    #[serde(rename = "useForm")]
    pub use_form: Boolean,
    #[serde(rename = "usernameType")]
    pub username_type: Option<String>,
    pub logo: Option<String>,
    pub color: Option<String>,
    pub website: Option<String>,
    pub icon: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct AuthenticationActiveStrategy {
    pub key: String,
    pub strategy: AuthenticationStrategy,
    #[serde(rename = "displayName")]
    pub display_name : String,
    pub order: Int,
    # [serde (rename = "isEnabled")] pub is_enabled: Boolean,
    pub config: Option<Vec<Option<KeyValuePair>>>,
    # [serde (rename = "selfRegistration")]
    pub self_registration: Boolean,
    # [serde (rename = "domainWhitelist")]
    pub domain_whitelist: Vec<Option<String>>,
    # [serde (rename = "autoEnrollGroups")]
    pub auto_enroll_groups: Vec<Option<Int>>,
}

#[derive(Deserialize, Debug)]
pub struct AuthenticationCreateApiKeyResponse {
    #[serde(rename = "responseResult")]
    pub response_result: Option<ResponseStatus>,
    pub key: Option<String>,
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

pub mod api_state_get {
    use super::*;

    pub struct ApiStateGet;

    pub const OPERATION_NAME: &str = "ApiStateGet";
    pub const QUERY: &str =
        "query ApiStateGet {\n  authentication {\n    apiState\n  }\n}\n";

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub authentication: Option<Authentication>,
    }

    #[derive(Deserialize)]
    pub struct Authentication {
        #[serde(rename = "apiState")]
        pub api_state: Boolean,
    }

    impl graphql_client::GraphQLQuery for ApiStateGet {
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

pub fn api_state_get(client: &Client, url: &str) -> Result<Boolean, UserError> {
    let variables = api_state_get::Variables {};
    let response = post_graphql::<api_state_get::ApiStateGet, _>(client, url, variables);
    if response.is_err() {
        return Err(UserError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(authentication) = data.authentication {
            return Ok(authentication.api_state);
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub mod authentication_strategy_list {
    use super::*;

    pub struct AuthenticationStrategyList;

    pub const OPERATION_NAME: &str = "AuthenticationStrategyList";
    pub const QUERY : & str = "query AuthenticationStrategyList {\n  authentication {\n    strategies {\n      key\n      props {\n        key\n        value\n      }\n      title\n      description\n      isAvailable\n      useForm\n      usernameType\n      logo\n      color\n      website\n      icon\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub authentication: Option<Authentication>,
    }

    #[derive(Deserialize)]
    pub struct Authentication {
        pub strategies: Option<Vec<Option<AuthenticationStrategy>>>,
    }

    impl graphql_client::GraphQLQuery for AuthenticationStrategyList {
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

pub fn authentication_strategy_list(
    client: &Client,
    url: &str,
) -> Result<Vec<AuthenticationStrategy>, UserError> {
    let variables = authentication_strategy_list::Variables {};
    let response =
        post_graphql::<authentication_strategy_list::AuthenticationStrategyList, _>(
            client, url, variables,
        );
    if response.is_err() {
        return Err(UserError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(authentication) = data.authentication {
            if let Some(strategies) = authentication.strategies {
                return Ok(strategies.into_iter().flatten().collect());
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub mod authentication_active_strategy_list {
    use super::*;

    pub struct AuthenticationActiveStrategyList;

    pub const OPERATION_NAME: &str = "AuthenticationActiveStrategyList";
    pub const QUERY : & str = "query AuthenticationActiveStrategyList($enabledOnly: Boolean) {\n  authentication {\n    activeStrategies(enabledOnly: $enabledOnly) {\n      key\n      strategy {\n        key\n        props {\n          key\n          value\n        }\n        title\n        description\n        isAvailable\n        useForm\n        usernameType\n        logo\n        color\n        website\n        icon\n      }\n      displayName\n      order\n      isEnabled\n      config {\n        key\n        value\n      }\n      selfRegistration\n      domainWhitelist\n      autoEnrollGroups\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "enabledOnly")]
        pub enabled_only: Option<Boolean>,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub authentication:
            Option<Authentication>,
    }

    #[derive(Deserialize)]
    pub struct Authentication {
        #[serde (rename = "activeStrategies")]
        pub active_strategies : Option <Vec<Option<AuthenticationActiveStrategy>>>,
    }

    impl graphql_client::GraphQLQuery for AuthenticationActiveStrategyList {
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

pub fn authentication_active_strategy_list(
    client: &Client,
    url: &str,
    enabled_only: Option<Boolean>,
) -> Result<Vec<AuthenticationActiveStrategy>, UserError> {
    let variables = authentication_active_strategy_list::Variables { enabled_only };
    let response = post_graphql::<
        authentication_active_strategy_list::AuthenticationActiveStrategyList,
        _,
    >(client, url, variables);
    if response.is_err() {
        return Err(UserError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(authentication) = data.authentication {
            if let Some(active_strategies) = authentication.active_strategies {
                return Ok(active_strategies.into_iter().flatten().collect());
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub mod api_key_create {
    use super::*;

    pub struct ApiKeyCreate;

    pub const OPERATION_NAME: &str = "ApiKeyCreate";
    pub const QUERY : & str = "mutation ApiKeyCreate (\n  $name: String!\n  $expiration: String\n  $fullAccess: Boolean!\n  $group: Int\n) {\n  authentication {\n    createApiKey(\n      name: $name\n      expiration: $expiration\n      fullAccess: $fullAccess\n      group: $group\n    ) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n      key\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub name: String,
        pub expiration: Option<String>,
        #[serde(rename = "fullAccess")]
        pub full_access: Boolean,
        pub group: Option<Int>,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub authentication: Option<Authentication>,
    }

    #[derive(Deserialize)]
    pub struct Authentication {
        #[serde(rename = "createApiKey")]
        pub create_api_key: Option<AuthenticationCreateApiKeyResponse>,
    }

    impl graphql_client::GraphQLQuery for ApiKeyCreate {
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

pub fn api_key_create(
    client: &Client,
    url: &str,
    name: String,
    expiration: Option<String>,
    full_access: Boolean,
    group: Option<Int>,
) -> Result<String, UserError> {
    let variables = api_key_create::Variables {
        name,
        expiration,
        full_access,
        group,
    };
    let response = post_graphql::<api_key_create::ApiKeyCreate, _>(client, url, variables);
    if response.is_err() {
        return Err(UserError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(authentication) = data.authentication {
            if let Some(create_api_key) = authentication.create_api_key {
                if let Some(response_result) = create_api_key.response_result {
                    if response_result.succeeded {
                        if let Some(key) = create_api_key.key {
                            return Ok(key);
                        } else {
                            return Err(UserError::UnknownErrorMessage {
                                message: "No key returned.".to_string(),
                            });
                        }
                    } else {
                        return Err(classify_response_status_error(
                                response_result
                                ));
                    }
                }
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}
