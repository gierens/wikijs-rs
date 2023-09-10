use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::{
    classify_response_error, classify_response_status_error, Boolean, Int,
    KnownErrorCodes, ResponseStatus, UnknownError,
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
        client, url, variables,
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
    Err(classify_response_error::<MailError>(response_body.errors))
}

pub mod mail_send_test {
    use super::*;

    pub struct MailSendTest;

    pub const OPERATION_NAME: &str = "MailSendTest";
    pub const QUERY : & str = "mutation MailSendTest(\n  $recipientEmail: String!\n) {\n  mail {\n    sendTest(\n      recipientEmail: $recipientEmail\n    ) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "recipientEmail")]
        pub recipient_email: String,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub mail: Option<Mail>,
    }

    #[derive(Deserialize)]
    pub struct Mail {
        #[serde(rename = "sendTest")]
        pub send_test: Option<SendTest>,
    }

    #[derive(Deserialize)]
    pub struct SendTest {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for MailSendTest {
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

pub fn mail_send_test(
    client: &Client,
    url: &str,
    recipient_email: String,
) -> Result<(), MailError> {
    let variables = mail_send_test::Variables { recipient_email };
    let response =
        post_graphql::<mail_send_test::MailSendTest, _>(client, url, variables);
    if response.is_err() {
        return Err(MailError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(mail) = data.mail {
            if let Some(send_test) = mail.send_test {
                if let Some(response_result) = send_test.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(
                            classify_response_status_error::<MailError>(
                                response_result,
                            ),
                        );
                    }
                }
            }
        }
    }
    Err(classify_response_error::<MailError>(response_body.errors))
}

pub mod mail_config_update {
    use super::*;

    pub struct MailConfigUpdate;

    pub const OPERATION_NAME: &str = "MailConfigUpdate";
    pub const QUERY : & str = "mutation MailConfigUpdate(\n  $senderName: String!\n  $senderEmail: String!\n  $host: String!\n  $port: Int!\n  $name: String!\n  $secure: Boolean!\n  $verifySSL: Boolean!\n  $user: String!\n  $pass: String!\n  $useDKIM: Boolean!\n  $dkimDomainName: String!\n  $dkimKeySelector: String!\n  $dkimPrivateKey: String!\n) {\n  mail {\n    updateConfig(\n      senderName: $senderName\n      senderEmail: $senderEmail\n      host: $host\n      port: $port\n      name: $name\n      secure: $secure\n      verifySSL: $verifySSL\n      user: $user\n      pass: $pass\n      useDKIM: $useDKIM\n      dkimDomainName: $dkimDomainName\n      dkimKeySelector: $dkimKeySelector\n      dkimPrivateKey: $dkimPrivateKey\n    ) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "senderName")]
        pub sender_name: String,
        #[serde(rename = "senderEmail")]
        pub sender_email: String,
        pub host: String,
        pub port: Int,
        pub name: String,
        pub secure: Boolean,
        #[serde(rename = "verifySSL")]
        pub verify_ssl: Boolean,
        pub user: String,
        pub pass: String,
        #[serde(rename = "useDKIM")]
        pub use_dkim: Boolean,
        #[serde(rename = "dkimDomainName")]
        pub dkim_domain_name: String,
        #[serde(rename = "dkimKeySelector")]
        pub dkim_key_selector: String,
        #[serde(rename = "dkimPrivateKey")]
        pub dkim_private_key: String,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub mail: Option<Mail>,
    }

    #[derive(Deserialize)]
    pub struct Mail {
        #[serde(rename = "updateConfig")]
        pub update_config: Option<UpdateConfig>,
    }
    #[derive(Deserialize)]
    pub struct UpdateConfig {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for MailConfigUpdate {
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

#[allow(clippy::too_many_arguments)]
pub fn mail_config_update(
    client: &Client,
    url: &str,
    sender_name: String,
    sender_email: String,
    host: String,
    port: Int,
    name: String,
    secure: Boolean,
    verify_ssl: Boolean,
    user: String,
    pass: String,
    use_dkim: Boolean,
    dkim_domain_name: String,
    dkim_key_selector: String,
    dkim_private_key: String,
) -> Result<(), MailError> {
    let variables = mail_config_update::Variables {
        sender_name,
        sender_email,
        host,
        port,
        name,
        secure,
        verify_ssl,
        user,
        pass,
        use_dkim,
        dkim_domain_name,
        dkim_key_selector,
        dkim_private_key,
    };
    let response = post_graphql::<mail_config_update::MailConfigUpdate, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(MailError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(mail) = data.mail {
            if let Some(update_config) = mail.update_config {
                if let Some(response_result) = update_config.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(
                            classify_response_status_error::<MailError>(
                                response_result,
                            ),
                        );
                    }
                }
            }
        }
    }
    Err(classify_response_error::<MailError>(response_body.errors))
}
