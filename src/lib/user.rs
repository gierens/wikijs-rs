use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::{
    classify_response_error, Boolean, Date, Int, KnownErrorCodes,
    ResponseStatus, UnknownError,
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
