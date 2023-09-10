use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::{
    classify_response_error, classify_response_status_error, Boolean, Date,
    Int, KnownErrorCodes, ResponseStatus, UnknownError,
};
use crate::group::Group;

#[derive(Debug, Error, PartialEq)]
pub enum UserError {
    #[error("An unexpected error occurred during login.")]
    AuthGenericError,
    #[error("Invalid email / username or password.")]
    AuthLoginFailed,
    #[error("Invalid authentication provider.")]
    AuthProviderInvalid,
    #[error("An account already exists using this email address.")]
    AuthAccountAlreadyExists,
    #[error("Incorrect TFA Security Code.")]
    AuthTFAFailed,
    #[error("Invalid TFA Security Code or Login Token.")]
    AuthTFAInvalid,
    #[error("Invalid Brute Force Instance.")]
    BruteInstanceIsInvalid,
    #[error("Too many attempts! Try again later.")]
    BruteTooManyAttempts,
    #[error("An unexpected error occurred during user creation.")]
    UserCreationFailed,
    #[error("Registration is disabled. Contact your system administrator.")]
    AuthRegistrationDisabled,
    #[error(
        "You are not authorized to register. Your domain is not whitelisted."
    )]
    AuthRegistrationDomainUnauthorized,
    #[error("Input data is invalid.")]
    InputInvalid,
    #[error("Your account has been disabled.")]
    AuthAccountBanned,
    #[error("You must verify your account before your can login.")]
    AuthAccountNotVerified,
    #[error("Invalid validation token.")]
    AuthValidationTokenInvalid,
    #[error("This user does not exist.")]
    UserNotFound,
    #[error("Cannot delete user because of content relational constraints.")]
    UserDeleteForeignConstraint,
    #[error("Cannot delete a protected system account.")]
    UserDeleteProtected,
    #[error("You must be authenticated to access this resource.")]
    AuthRequired,
    #[error("Password is incorrect.")]
    AuthPasswordInvalid,
    #[error("Unknown response error code: {code}: {message}")]
    UnknownErrorCode { code: i64, message: String },
    #[error("Unknown response error: {message}")]
    UnknownErrorMessage { message: String },
    #[error("Unknown response error.")]
    UnknownError,
}

impl From<i64> for UserError {
    fn from(code: i64) -> Self {
        match code {
            1001 => UserError::AuthGenericError,
            1002 => UserError::AuthLoginFailed,
            1003 => UserError::AuthProviderInvalid,
            1004 => UserError::AuthAccountAlreadyExists,
            1005 => UserError::AuthTFAFailed,
            1006 => UserError::AuthTFAInvalid,
            1007 => UserError::BruteInstanceIsInvalid,
            1008 => UserError::BruteTooManyAttempts,
            1009 => UserError::UserCreationFailed,
            1010 => UserError::AuthRegistrationDisabled,
            1011 => UserError::AuthRegistrationDomainUnauthorized,
            1012 => UserError::InputInvalid,
            1013 => UserError::AuthAccountBanned,
            1014 => UserError::AuthAccountNotVerified,
            1015 => UserError::AuthValidationTokenInvalid,
            1016 => UserError::UserNotFound,
            1017 => UserError::UserDeleteForeignConstraint,
            1018 => UserError::UserDeleteProtected,
            1019 => UserError::AuthRequired,
            1020 => UserError::AuthPasswordInvalid,
            _ => UserError::UnknownErrorCode {
                code,
                message: "Unknown error".to_string(),
            },
        }
    }
}

impl UnknownError for UserError {
    fn unknown_error_code(code: i64, message: String) -> Self {
        UserError::UnknownErrorCode { code, message }
    }
    fn unknown_error_message(message: String) -> Self {
        UserError::UnknownErrorMessage { message }
    }
    fn unknown_error() -> Self {
        UserError::UnknownError
    }
}

impl KnownErrorCodes for UserError {
    fn known_error_codes() -> Vec<i64> {
        vec![
            1001, 1002, 1003, 1004, 1005, 1006, 1007, 1008, 1010, 1011, 1012,
            1013, 1014, 1015, 1016, 1017, 1018, 1019, 1020,
        ]
    }

    fn is_known_error_code(code: i64) -> bool {
        (1001..=1020).contains(&code)
    }
}

#[derive(Deserialize, Debug)]
pub struct UserResponse {
    #[serde(rename = "responseResult")]
    pub response_result: ResponseStatus,
    pub user: Option<User>,
}

#[derive(Deserialize, Debug)]
pub struct UserLastLogin {
    pub id: Int,
    pub name: String,
    #[serde(rename = "lastLoginAt")]
    pub last_login_at: Date,
}

#[derive(Deserialize, Debug)]
pub struct UserMinimal {
    pub id: Int,
    pub name: String,
    pub email: String,
    #[serde(rename = "providerKey")]
    pub provider_key: String,
    #[serde(rename = "isSystem")]
    pub is_system: Boolean,
    #[serde(rename = "isActive")]
    pub is_active: Boolean,
    #[serde(rename = "createdAt")]
    pub created_at: Date,
    #[serde(rename = "lastLoginAt")]
    pub last_login_at: Option<Date>,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: Int,
    pub name: String,
    pub email: String,
    #[serde(rename = "providerKey")]
    pub provider_key: String,
    #[serde(rename = "providerName")]
    pub provider_name: Option<String>,
    #[serde(rename = "providerId")]
    pub provider_id: Option<String>,
    #[serde(rename = "providerIs2FACapable")]
    pub provider_is_2fa_capable: Option<Boolean>,
    #[serde(rename = "isSystem")]
    pub is_system: Boolean,
    #[serde(rename = "isActive")]
    pub is_active: Boolean,
    #[serde(rename = "isVerified")]
    pub is_verified: Boolean,
    pub location: String,
    #[serde(rename = "jobTitle")]
    pub job_title: String,
    pub timezone: String,
    #[serde(rename = "dateFormat")]
    pub date_format: String,
    pub appearance: String,
    #[serde(rename = "createdAt")]
    pub created_at: Date,
    #[serde(rename = "updatedAt")]
    pub updated_at: Date,
    #[serde(rename = "lastLoginAt")]
    pub last_login_at: Option<Date>,
    #[serde(rename = "tfaIsActive")]
    pub tfa_is_active: Boolean,
    pub groups: Vec<Option<Group>>,
}

#[derive(Deserialize, Debug)]
pub struct UserProfile {
    pub id: Int,
    pub name: String,
    pub email: String,
    #[serde(rename = "providerKey")]
    pub provider_key: Option<String>,
    #[serde(rename = "providerName")]
    pub provider_name: Option<String>,
    #[serde(rename = "isSystem")]
    pub is_system: Boolean,
    #[serde(rename = "isVerified")]
    pub is_verified: Boolean,
    pub location: String,
    #[serde(rename = "jobTitle")]
    pub job_title: String,
    pub timezone: String,
    #[serde(rename = "dateFormat")]
    pub date_format: String,
    pub appearance: String,
    #[serde(rename = "createdAt")]
    pub created_at: Date,
    #[serde(rename = "updatedAt")]
    pub updated_at: Date,
    #[serde(rename = "lastLoginAt")]
    pub last_login_at: Option<Date>,
    pub groups: Vec<String>,
    #[serde(rename = "pagesTotal")]
    pub pages_total: Int,
}

#[derive(Deserialize, Debug)]
pub struct UserTokenResponse {
    #[serde(rename = "responseResult")]
    pub response_result: ResponseStatus,
    pub jwt: Option<String>,
}

pub mod user_get {
    use super::*;

    pub struct UserGet;

    pub const OPERATION_NAME: &str = "UserGet";
    pub const QUERY : & str = "query UserGet($id: Int!) {\n  users {\n    single (id: $id) {\n      id\n      name\n      email\n      providerKey\n      providerName\n      providerId\n      providerIs2FACapable\n      isSystem\n      isActive\n      isVerified\n      location\n      jobTitle\n      timezone\n      dateFormat\n      appearance\n      createdAt\n      updatedAt\n      lastLoginAt\n      tfaIsActive\n      groups {\n        id\n        name\n        isSystem\n        redirectOnLogin\n        permissions\n        pageRules {\n          id\n          deny\n          match\n          roles\n          path\n          locales\n        }\n        users {\n          id\n          name\n          email\n          providerKey\n          isSystem\n          isActive\n          createdAt\n          lastLoginAt\n        }\n        createdAt\n        updatedAt\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub id: Int,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub users: Option<Users>,
    }

    #[derive(Deserialize)]
    pub struct Users {
        pub single: Option<User>,
    }

    impl graphql_client::GraphQLQuery for UserGet {
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

pub fn user_get(
    client: &Client,
    url: &str,
    id: i64,
) -> Result<User, UserError> {
    let variables = user_get::Variables { id };
    let response = post_graphql::<user_get::UserGet, _>(client, url, variables);
    if response.is_err() {
        return Err(UserError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(users) = data.users {
            if let Some(user) = users.single {
                return Ok(user);
            }
        }
    }
    Err(classify_response_error::<UserError>(response_body.errors))
}

pub mod user_list {
    use super::*;

    pub struct UserList;

    pub const OPERATION_NAME: &str = "UserList";
    pub const QUERY : & str = "query UserList($filter: String, $orderBy: String) {\n  users {\n    list (filter: $filter, orderBy: $orderBy) {\n      id\n      name\n      email providerKey\n      isSystem\n      isActive\n      createdAt\n      lastLoginAt\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub filter: Option<String>,
        #[serde(rename = "orderBy")]
        pub order_by: Option<String>,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub users: Option<Users>,
    }

    #[derive(Deserialize)]
    pub struct Users {
        pub list: Option<Vec<Option<UserMinimal>>>,
    }

    impl graphql_client::GraphQLQuery for UserList {
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

pub fn user_list(
    client: &Client,
    url: &str,
    filter: Option<String>,
    order_by: Option<String>,
) -> Result<Vec<UserMinimal>, UserError> {
    let variables = user_list::Variables { filter, order_by };
    let response =
        post_graphql::<user_list::UserList, _>(client, url, variables);
    if response.is_err() {
        return Err(UserError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(users) = data.users {
            if let Some(list) = users.list {
                return Ok(list.into_iter().flatten().collect());
            }
        }
    }
    Err(classify_response_error::<UserError>(response_body.errors))
}

pub mod user_activate {
    use super::*;

    pub struct UserActivate;

    pub const OPERATION_NAME: &str = "UserActivate";
    pub const QUERY : & str = "mutation UserActivate($id: Int!) {\n  users {\n    activate (id: $id) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub id: Int,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub users: Option<Users>,
    }

    #[derive(Deserialize)]
    pub struct Users {
        pub activate: Option<Activate>,
    }

    #[derive(Deserialize)]
    pub struct Activate {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for UserActivate {
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

pub fn user_activate(
    client: &Client,
    url: &str,
    id: i64,
) -> Result<(), UserError> {
    let variables = user_activate::Variables { id };
    let response =
        post_graphql::<user_activate::UserActivate, _>(client, url, variables);
    if response.is_err() {
        return Err(UserError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(users) = data.users {
            if let Some(activate) = users.activate {
                if let Some(response_result) = activate.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(
                            classify_response_status_error::<UserError>(
                                response_result,
                            ),
                        );
                    }
                }
            }
        }
    }
    Err(classify_response_error::<UserError>(response_body.errors))
}

pub mod user_deactivate {
    use super::*;

    pub struct UserDeactivate;

    pub const OPERATION_NAME: &str = "UserDeactivate";
    pub const QUERY : & str = "mutation UserDeactivate($id: Int!) {\n  users {\n    deactivate (id: $id) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub id: Int,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub users: Option<Users>,
    }

    #[derive(Deserialize)]
    pub struct Users {
        pub deactivate: Option<Deactivate>,
    }

    #[derive(Deserialize)]
    pub struct Deactivate {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for UserDeactivate {
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

pub fn user_deactivate(
    client: &Client,
    url: &str,
    id: i64,
) -> Result<(), UserError> {
    let variables = user_deactivate::Variables { id };
    let response = post_graphql::<user_deactivate::UserDeactivate, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(UserError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(users) = data.users {
            if let Some(deactivate) = users.deactivate {
                if let Some(response_result) = deactivate.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(
                            classify_response_status_error::<UserError>(
                                response_result,
                            ),
                        );
                    }
                }
            }
        }
    }
    Err(classify_response_error::<UserError>(response_body.errors))
}

pub mod user_delete {
    use super::*;

    pub struct UserDelete;

    pub const OPERATION_NAME: &str = "UserDelete";
    pub const QUERY : & str = "mutation UserDelete($id: Int!, $replaceId: Int!) {\n  users {\n    delete (id: $id, replaceId: $replaceId) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize)]
    pub struct Variables {
        pub id: Int,
        #[serde(rename = "replaceId")]
        pub replace_id: Int,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub users: Option<Users>,
    }

    #[derive(Deserialize)]
    pub struct Users {
        pub delete: Option<Delete>,
    }

    #[derive(Deserialize)]
    pub struct Delete {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for UserDelete {
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

pub fn user_delete(
    client: &Client,
    url: &str,
    id: i64,
    replace_id: i64,
) -> Result<(), UserError> {
    let variables = user_delete::Variables { id, replace_id };
    let response =
        post_graphql::<user_delete::UserDelete, _>(client, url, variables);
    if response.is_err() {
        return Err(UserError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(users) = data.users {
            if let Some(delete) = users.delete {
                if let Some(response_result) = delete.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(
                            classify_response_status_error::<UserError>(
                                response_result,
                            ),
                        );
                    }
                }
            }
        }
    }
    Err(classify_response_error::<UserError>(response_body.errors))
}

pub mod user_tfa_disable {
    use super::*;

    pub struct UserTfaDisable;

    pub const OPERATION_NAME: &str = "UserTfaDisable";
    pub const QUERY : & str = "mutation UserTfaDisable($id: Int!) {\n  users {\n    disableTFA (id: $id) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub id: Int,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub users: Option<Users>,
    }

    #[derive(Deserialize)]
    pub struct Users {
        #[serde(rename = "disableTFA")]
        pub disable_tfa: Option<DisableTfa>,
    }
    #[derive(Deserialize)]
    pub struct DisableTfa {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for UserTfaDisable {
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

pub fn user_tfa_disable(
    client: &Client,
    url: &str,
    id: i64,
) -> Result<(), UserError> {
    let variables = user_tfa_disable::Variables { id };
    let response = post_graphql::<user_tfa_disable::UserTfaDisable, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(UserError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(users) = data.users {
            if let Some(disable_tfa) = users.disable_tfa {
                if let Some(response_result) = disable_tfa.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(
                            classify_response_status_error::<UserError>(
                                response_result,
                            ),
                        );
                    }
                }
            }
        }
    }
    Err(classify_response_error::<UserError>(response_body.errors))
}

pub mod user_tfa_enable {
    use super::*;

    pub struct UserTfaEnable;

    pub const OPERATION_NAME: &str = "UserTfaEnable";
    pub const QUERY : & str = "mutation UserTfaEnable($id: Int!) {\n  users {\n    enableTFA (id: $id) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub id: Int,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub users: Option<Users>,
    }

    #[derive(Deserialize)]
    pub struct Users {
        #[serde(rename = "enableTFA")]
        pub enable_tfa: Option<EnableTfa>,
    }

    #[derive(Deserialize)]
    pub struct EnableTfa {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for UserTfaEnable {
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

pub fn user_tfa_enable(
    client: &Client,
    url: &str,
    id: i64,
) -> Result<(), UserError> {
    let variables = user_tfa_enable::Variables { id };
    let response = post_graphql::<user_tfa_enable::UserTfaEnable, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(UserError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(users) = data.users {
            if let Some(enable_tfa) = users.enable_tfa {
                if let Some(response_result) = enable_tfa.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(
                            classify_response_status_error::<UserError>(
                                response_result,
                            ),
                        );
                    }
                }
            }
        }
    }
    Err(classify_response_error::<UserError>(response_body.errors))
}

pub mod user_verify {
    use super::*;

    pub struct UserVerify;

    pub const OPERATION_NAME: &str = "UserVerify";
    pub const QUERY : & str = "mutation UserVerify($id: Int!) {\n  users {\n    verify (id: $id) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub id: Int,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub users: Option<Users>,
    }
    #[derive(Deserialize)]
    pub struct Users {
        pub verify: Option<Verify>,
    }
    #[derive(Deserialize)]
    pub struct Verify {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for UserVerify {
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

pub fn user_verify(
    client: &Client,
    url: &str,
    id: i64,
) -> Result<(), UserError> {
    let variables = user_verify::Variables { id };
    let response =
        post_graphql::<user_verify::UserVerify, _>(client, url, variables);
    if response.is_err() {
        return Err(UserError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(users) = data.users {
            if let Some(verify) = users.verify {
                if let Some(response_result) = verify.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(
                            classify_response_status_error::<UserError>(
                                response_result,
                            ),
                        );
                    }
                }
            }
        }
    }
    Err(classify_response_error::<UserError>(response_body.errors))
}

pub mod user_search {
    use super::*;

    pub struct UserSearch;

    pub const OPERATION_NAME: &str = "UserSearch";
    pub const QUERY : & str = "query UserSearch($query: String!) {\n  users {\n    search(query: $query) {\n      id\n      name\n      email providerKey\n      isSystem\n      isActive\n      createdAt\n      lastLoginAt\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub query: String,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub users: Option<Users>,
    }

    #[derive(Deserialize)]
    pub struct Users {
        pub search: Option<Vec<Option<UserMinimal>>>,
    }

    impl graphql_client::GraphQLQuery for UserSearch {
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

pub fn user_search(
    client: &Client,
    url: &str,
    query: String,
) -> Result<Vec<UserMinimal>, UserError> {
    let variables = user_search::Variables { query };
    let response =
        post_graphql::<user_search::UserSearch, _>(client, url, variables);
    if response.is_err() {
        return Err(UserError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(users) = data.users {
            if let Some(search) = users.search {
                return Ok(search.into_iter().flatten().collect());
            }
        }
    }
    Err(classify_response_error::<UserError>(response_body.errors))
}

pub mod user_profile_get {
    use super::*;

    pub struct UserProfileGet;

    pub const OPERATION_NAME: &str = "UserProfileGet";
    pub const QUERY : & str = "query UserProfileGet {\n  users {\n    profile {\n      id\n      name\n      email\n      providerKey\n      providerName\n      isSystem\n      isVerified\n      location\n      jobTitle\n      timezone\n      dateFormat\n      appearance\n      createdAt\n      updatedAt\n      lastLoginAt\n      groups\n      pagesTotal\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub users: Option<Users>,
    }
    #[derive(Deserialize)]
    pub struct Users {
        pub profile: Option<UserProfile>,
    }

    impl graphql_client::GraphQLQuery for UserProfileGet {
        type Variables = Variables;
        type ResponseData = ResponseData;
        fn build_query(
            _variables: Self::Variables,
        ) -> ::graphql_client::QueryBody<Self::Variables> {
            graphql_client::QueryBody {
                variables: Variables {},
                query: QUERY,
                operation_name: OPERATION_NAME,
            }
        }
    }
}

pub fn user_profile_get(
    client: &Client,
    url: &str,
) -> Result<UserProfile, UserError> {
    let variables = user_profile_get::Variables {};
    let response = post_graphql::<user_profile_get::UserProfileGet, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(UserError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(users) = data.users {
            if let Some(profile) = users.profile {
                return Ok(profile);
            }
        }
    }
    Err(classify_response_error::<UserError>(response_body.errors))
}

pub mod user_last_login_list {
    use super::*;

    pub struct UserLastLoginList;

    pub const OPERATION_NAME: &str = "UserLastLoginList";
    pub const QUERY : & str = "query UserLastLoginList {\n  users {\n    lastLogins {\n      id\n      name\n      lastLoginAt\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub users: Option<Users>,
    }

    #[derive(Deserialize)]
    pub struct Users {
        #[serde(rename = "lastLogins")]
        pub last_logins: Option<Vec<Option<UserLastLogin>>>,
    }

    impl graphql_client::GraphQLQuery for UserLastLoginList {
        type Variables = Variables;
        type ResponseData = ResponseData;
        fn build_query(
            _variables: Self::Variables,
        ) -> ::graphql_client::QueryBody<Self::Variables> {
            graphql_client::QueryBody {
                variables: Variables {},
                query: QUERY,
                operation_name: OPERATION_NAME,
            }
        }
    }
}

pub fn user_last_login_list(
    client: &Client,
    url: &str,
) -> Result<Vec<UserLastLogin>, UserError> {
    let variables = user_last_login_list::Variables {};
    let response = post_graphql::<user_last_login_list::UserLastLoginList, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(UserError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(users) = data.users {
            if let Some(last_logins) = users.last_logins {
                return Ok(last_logins.into_iter().flatten().collect());
            }
        }
    }
    Err(classify_response_error::<UserError>(response_body.errors))
}

pub mod user_create {
    use super::*;

    pub struct UserCreate;

    pub const OPERATION_NAME: &str = "UserCreate";
    pub const QUERY : & str = "mutation UserCreate(\n  $email: String!\n  $name: String!\n  $passwordRaw: String\n  $providerKey: String!\n  $groups: [Int]!\n  $mustChangePassword: Boolean\n  $sendWelcomeEmail: Boolean\n) {\n  users {\n    create (\n      email: $email\n      name: $name\n      passwordRaw: $passwordRaw\n      providerKey: $providerKey\n      groups: $groups\n      mustChangePassword: $mustChangePassword\n      sendWelcomeEmail: $sendWelcomeEmail\n    ) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n      user {\n        id\n        name\n        email\n        providerKey\n        providerName\n        providerId\n        providerIs2FACapable\n        isSystem\n        isActive\n        isVerified\n        location\n        jobTitle\n        timezone\n        dateFormat\n        appearance\n        createdAt\n        updatedAt\n        lastLoginAt\n        tfaIsActive\n        groups {\n          id\n          name\n          isSystem\n          redirectOnLogin\n          permissions\n          pageRules {\n            id\n            deny\n            match\n            roles\n            path\n            locales\n          }\n          users {\n            id\n            name\n            email\n            providerKey\n            isSystem\n            isActive\n            createdAt\n            lastLoginAt\n          }\n          createdAt\n          updatedAt\n        }\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub email: String,
        pub name: String,
        #[serde(rename = "passwordRaw")]
        pub password_raw: Option<String>,
        #[serde(rename = "providerKey")]
        pub provider_key: String,
        pub groups: Vec<Option<Int>>,
        #[serde(rename = "mustChangePassword")]
        pub must_change_password: Option<Boolean>,
        #[serde(rename = "sendWelcomeEmail")]
        pub send_welcome_email: Option<Boolean>,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub users: Option<Users>,
    }

    #[derive(Deserialize)]
    pub struct Users {
        pub create: Option<UserResponse>,
    }

    impl graphql_client::GraphQLQuery for UserCreate {
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

pub fn user_create(
    client: &Client,
    url: &str,
    email: String,
    name: String,
    password_raw: Option<String>,
    provider_key: String,
    groups: Vec<Option<i64>>,
    must_change_password: Option<bool>,
    send_welcome_email: Option<bool>,
) -> Result<(), UserError> {
    let variables = user_create::Variables {
        email,
        name,
        password_raw,
        provider_key,
        groups,
        must_change_password,
        send_welcome_email,
    };
    let response =
        post_graphql::<user_create::UserCreate, _>(client, url, variables);
    if response.is_err() {
        return Err(UserError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(users) = data.users {
            if let Some(create) = users.create {
                if create.response_result.succeeded {
                    // TODO check that this really does not return the user
                    return Ok(());
                } else {
                    return Err(classify_response_status_error::<UserError>(
                        create.response_result,
                    ));
                }
            }
        }
    }
    Err(classify_response_error::<UserError>(response_body.errors))
}

pub mod user_update {
    use super::*;

    pub struct UserUpdate;

    pub const OPERATION_NAME: &str = "UserUpdate";
    pub const QUERY : & str = "mutation UserUpdate(\n  $id: Int!\n  $email: String\n  $name: String\n  $newPassword: String\n  $groups: [Int]\n  $location: String\n  $jobTitle: String\n  $timezone: String\n  $dateFormat: String\n  $appearance: String\n) {\n  users {\n    update (\n      id: $id\n      email: $email\n      name: $name\n      newPassword: $newPassword\n      groups: $groups\n      location: $location\n      jobTitle: $jobTitle\n      timezone: $timezone\n      dateFormat: $dateFormat\n      appearance: $appearance\n    ) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub id: Int,
        pub email: Option<String>,
        pub name: Option<String>,
        #[serde(rename = "newPassword")]
        pub new_password: Option<String>,
        pub groups: Option<Vec<Option<Int>>>,
        pub location: Option<String>,
        #[serde(rename = "jobTitle")]
        pub job_title: Option<String>,
        pub timezone: Option<String>,
        #[serde(rename = "dateFormat")]
        pub date_format: Option<String>,
        pub appearance: Option<String>,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub users: Option<Users>,
    }

    #[derive(Deserialize)]
    pub struct Users {
        pub update: Option<Update>,
    }

    #[derive(Deserialize)]
    pub struct Update {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for UserUpdate {
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

pub fn user_update(
    client: &Client,
    url: &str,
    id: i64,
    email: Option<String>,
    name: Option<String>,
    new_password: Option<String>,
    groups: Option<Vec<Option<i64>>>,
    location: Option<String>,
    job_title: Option<String>,
    timezone: Option<String>,
    date_format: Option<String>,
    appearance: Option<String>,
) -> Result<(), UserError> {
    let variables = user_update::Variables {
        id,
        email,
        name,
        new_password,
        groups,
        location,
        job_title,
        timezone,
        date_format,
        appearance,
    };
    let response =
        post_graphql::<user_update::UserUpdate, _>(client, url, variables);
    if response.is_err() {
        return Err(UserError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();

    if let Some(data) = response_body.data {
        if let Some(users) = data.users {
            if let Some(update) = users.update {
                if let Some(response_result) = update.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(
                            classify_response_status_error::<UserError>(
                                response_result,
                            ),
                        );
                    }
                }
            }
        }
    }
    Err(classify_response_error::<UserError>(response_body.errors))
}

pub mod user_profile_update {
    use super::*;

    pub struct UserProfileUpdate;

    pub const OPERATION_NAME: &str = "UserProfileUpdate";
    pub const QUERY : & str = "mutation UserProfileUpdate(\n  $name: String!\n  $location: String!\n  $jobTitle: String!\n  $timezone: String!\n  $dateFormat: String!\n  $appearance: String!\n) {\n  users {\n    updateProfile (\n      name: $name\n      location: $location\n      jobTitle: $jobTitle\n      timezone: $timezone\n      dateFormat: $dateFormat\n      appearance: $appearance\n    ) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n      jwt\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub name: String,
        pub location: String,
        #[serde(rename = "jobTitle")]
        pub job_title: String,
        pub timezone: String,
        #[serde(rename = "dateFormat")]
        pub date_format: String,
        pub appearance: String,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub users: Option<Users>,
    }

    #[derive(Deserialize)]
    pub struct Users {
        #[serde(rename = "updateProfile")]
        pub update_profile: Option<UserTokenResponse>,
    }

    impl graphql_client::GraphQLQuery for UserProfileUpdate {
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

pub fn user_profile_update(
    client: &Client,
    url: &str,
    name: String,
    location: String,
    job_title: String,
    timezone: String,
    date_format: String,
    appearance: String,
) -> Result<Option<String>, UserError> {
    let variables = user_profile_update::Variables {
        name,
        location,
        job_title,
        timezone,
        date_format,
        appearance,
    };
    let response = post_graphql::<user_profile_update::UserProfileUpdate, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(UserError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();

    if let Some(data) = response_body.data {
        if let Some(users) = data.users {
            if let Some(update_profile) = users.update_profile {
                if update_profile.response_result.succeeded {
                    return Ok(update_profile.jwt);
                } else {
                    return Err(classify_response_status_error::<UserError>(
                        update_profile.response_result,
                    ));
                }
            }
        }
    }
    Err(classify_response_error::<UserError>(response_body.errors))
}

pub mod user_password_change {
    use super::*;

    pub struct UserPasswordChange;

    pub const OPERATION_NAME: &str = "UserPasswordChange";
    pub const QUERY : & str = "mutation UserPasswordChange($current: String!, $new: String!) {\n  users {\n    changePassword(current: $current, new: $new) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n      jwt\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub current: String,
        pub new: String,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub users: Option<Users>,
    }

    #[derive(Deserialize)]
    pub struct Users {
        #[serde(rename = "changePassword")]
        pub change_password: Option<UserTokenResponse>,
    }

    impl graphql_client::GraphQLQuery for UserPasswordChange {
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

pub fn user_password_change(
    client: &Client,
    url: &str,
    current: String,
    new: String,
) -> Result<Option<String>, UserError> {
    let variables = user_password_change::Variables { current, new };
    let response = post_graphql::<user_password_change::UserPasswordChange, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(UserError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();

    if let Some(data) = response_body.data {
        if let Some(users) = data.users {
            if let Some(change_password) = users.change_password {
                if change_password.response_result.succeeded {
                    return Ok(change_password.jwt);
                } else {
                    return Err(classify_response_status_error::<UserError>(
                        change_password.response_result,
                    ));
                }
            }
        }
    }
    Err(classify_response_error::<UserError>(response_body.errors))
}

pub mod user_password_reset {
    use super::*;

    pub struct UserPasswordReset;

    pub const OPERATION_NAME: &str = "UserPasswordReset";
    pub const QUERY : & str = "mutation UserPasswordReset($id: Int!) {\n  users {\n    resetPassword(id: $id) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub id: Int,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub users: Option<Users>,
    }

    #[derive(Deserialize)]
    pub struct Users {
        #[serde(rename = "resetPassword")]
        pub reset_password: Option<ResetPassword>,
    }
    #[derive(Deserialize)]
    pub struct ResetPassword {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for UserPasswordReset {
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

pub fn user_password_reset(
    client: &Client,
    url: &str,
    id: i64,
) -> Result<(), UserError> {
    let variables = user_password_reset::Variables { id };
    let response = post_graphql::<user_password_reset::UserPasswordReset, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(UserError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }

    let response_body = response.unwrap();

    if let Some(data) = response_body.data {
        if let Some(users) = data.users {
            if let Some(reset_password) = users.reset_password {
                if let Some(response_result) = reset_password.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(
                            classify_response_status_error::<UserError>(
                                response_result,
                            ),
                        );
                    }
                }
            }
        }
    }
    Err(classify_response_error::<UserError>(response_body.errors))
}
