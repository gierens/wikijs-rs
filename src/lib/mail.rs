use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::{
    classify_response_error, Boolean, Int, KnownErrorCodes, UnknownError,
};

#[derive(Debug, Error, PartialEq)]
pub enum MailError {
    #[error("An unexpected error occurred during mail operation.")]
    MailGenericError,
    #[error("The mail configuration is incomplete or invalid.")]
    MailNotConfigured,
    #[error("Mail template failed to load.")]
    MailTemplateFailed,
    #[error("The recipient email address is invalid.")]
    MailInvalidRecipient,
    #[error("Unknown response error code: {code}: {message}")]
    UnknownErrorCode { code: i64, message: String },
    #[error("Unknown response error: {message}")]
    UnknownErrorMessage { message: String },
    #[error("Unknown response error.")]
    UnknownError,
}

impl From<i64> for MailError {
    fn from(code: i64) -> Self {
        match code {
            3001 => MailError::MailGenericError,
            3002 => MailError::MailNotConfigured,
            3003 => MailError::MailTemplateFailed,
            3004 => MailError::MailInvalidRecipient,
            _ => MailError::UnknownErrorCode {
                code,
                message: "Unknown error".to_string(),
            },
        }
    }
}

impl UnknownError for MailError {
    fn unknown_error_code(code: i64, message: String) -> Self {
        MailError::UnknownErrorCode { code, message }
    }
    fn unknown_error_message(message: String) -> Self {
        MailError::UnknownErrorMessage { message }
    }
    fn unknown_error() -> Self {
        MailError::UnknownError
    }
}

impl KnownErrorCodes for MailError {
    fn known_error_codes() -> Vec<i64> {
        vec![3001, 3002, 3003, 3004]
    }

    fn is_known_error_code(code: i64) -> bool {
        (3001..=3004).contains(&code)
    }
}

#[derive(Deserialize, Debug)]
pub struct MailConfig {
    #[serde(rename = "senderName")]
    pub sender_name: Option<String>,
    #[serde(rename = "senderEmail")]
    pub sender_email: Option<String>,
    pub host: Option<String>,
    pub port: Option<Int>,
    pub name: Option<String>,
    pub secure: Option<Boolean>,
    #[serde(rename = "verifySSL")]
    pub verify_ssl: Option<Boolean>,
    pub user: Option<String>,
    pub pass: Option<String>,
    #[serde(rename = "useDKIM")]
    pub use_dkim: Option<Boolean>,
    #[serde(rename = "dkimDomainName")]
    pub dkim_domain_name: Option<String>,
    #[serde(rename = "dkimKeySelector")]
    pub dkim_key_selector: Option<String>,
    #[serde(rename = "dkimPrivateKey")]
    pub dkim_private_key: Option<String>,
}

pub mod mail_config_get {
    use super::*;

    pub struct MailConfigGet;

    pub const OPERATION_NAME: &str = "MailConfigGet";
    pub const QUERY : & str = "query MailConfigGet {\n  mail {\n    config {\n      senderName\n      senderEmail\n      host\n      port\n      name\n      secure\n      verifySSL\n      user\n      pass\n      useDKIM\n      dkimDomainName\n      dkimKeySelector\n      dkimPrivateKey\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub mail: Option<Mail>,
    }

    #[derive(Deserialize)]
    pub struct Mail {
        pub config: Option<MailConfig>,
    }

    impl graphql_client::GraphQLQuery for MailConfigGet {
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

pub fn mail_config_get(
    client: &Client,
    url: &str,
) -> Result<MailConfig, MailError> {
    let variables = mail_config_get::Variables {};
    let response = post_graphql::<mail_config_get::MailConfigGet, _>(
        client,
        url,
        variables,
    );
    if response.is_err() {
        return Err(MailError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(mail) = data.mail {
            if let Some(config) = mail.config {
                return Ok(config);
            }
        }
    }
    Err(classify_response_error::<MailError>(
        response_body.errors,
    ))
}
