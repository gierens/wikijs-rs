use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::{
    classify_response_error, KnownErrorCodes, UnknownError, Boolean,
    classify_response_status_error, ResponseStatus, Int, Date,
};

#[derive(Debug, Error, PartialEq)]
pub enum SystemError {
    #[error("An unexpected error occurred.")]
    SystemGenericError,
    #[error("SSL is not enabled.")]
    SystemSSLDisabled,
    #[error("Current provider does not support SSL certificate renewal.")]
    SystemSSLRenewInvalidProvider,
    #[error("Let's Encrypt is not initialized.")]
    SystemSSLLEUnavailable,
    #[error("Unknown response error code: {code}: {message}")]
    UnknownErrorCode { code: i64, message: String },
    #[error("Unknown response error: {message}")]
    UnknownErrorMessage { message: String },
    #[error("Unknown response error.")]
    UnknownError,
}

impl From<i64> for SystemError {
    fn from(code: i64) -> Self {
        match code {
            7001 => SystemError::SystemGenericError,
            7002 => SystemError::SystemSSLDisabled,
            7003 => SystemError::SystemSSLRenewInvalidProvider,
            7004 => SystemError::SystemSSLLEUnavailable,
            _ => SystemError::UnknownErrorCode {
                code,
                message: "Unknown error".to_string(),
            },
        }
    }
}

impl UnknownError for SystemError {
    fn unknown_error_code(code: i64, message: String) -> Self {
        SystemError::UnknownErrorCode { code, message }
    }
    fn unknown_error_message(message: String) -> Self {
        SystemError::UnknownErrorMessage { message }
    }
    fn unknown_error() -> Self {
        SystemError::UnknownError
    }
}

impl KnownErrorCodes for SystemError {
    fn known_error_codes() -> Vec<i64> {
        vec![7001, 7002, 7003, 7004]
    }

    fn is_known_error_code(code: i64) -> bool {
        (7001..=7004).contains(&code)
    }
}

#[derive(Deserialize, Debug)]
pub struct SystemFlag {
    pub key: String,
    pub value: Boolean,
}

#[derive(Deserialize, Debug)]
pub struct SystemInfo {
    #[serde(rename = "configFile")]
    pub config_file: Option<String>,
    #[serde(rename = "cpuCores")]
    pub cpu_cores: Option<Int>,
    #[serde(rename = "currentVersion")]
    pub current_version: Option<String>,
    #[serde(rename = "dbHost")]
    pub db_host: Option<String>,
    #[serde(rename = "dbType")]
    pub db_type: Option<String>,
    #[serde(rename = "dbVersion")]
    pub db_version: Option<String>,
    #[serde(rename = "groupsTotal")]
    pub groups_total: Option<Int>,
    pub hostname: Option<String>,
    #[serde(rename = "httpPort")]
    pub http_port: Option<Int>,
    #[serde(rename = "httpRedirection")]
    pub http_redirection: Option<Boolean>,
    #[serde(rename = "httpsPort")]
    pub https_port: Option<Int>,
    #[serde(rename = "latestVersion")]
    pub latest_version: Option<String>,
    #[serde(rename = "latestVersionReleaseDate")]
    pub latest_version_release_date: Option<Date>,
    #[serde(rename = "nodeVersion")]
    pub node_version: Option<String>,
    #[serde(rename = "operatingSystem")]
    pub operating_system: Option<String>,
    #[serde(rename = "pagesTotal")]
    pub pages_total: Option<Int>,
    pub platform: Option<String>,
    #[serde(rename = "ramTotal")]
    pub ram_total: Option<String>,
    #[serde(rename = "sslDomain")]
    pub ssl_domain: Option<String>,
    #[serde(rename = "sslExpirationDate")]
    pub ssl_expiration_date: Option<Date>,
    #[serde(rename = "sslProvider")]
    pub ssl_provider: Option<String>,
    #[serde(rename = "sslStatus")]
    pub ssl_status: Option<String>,
    #[serde(rename = "sslSubscriberEmail")]
    pub ssl_subscriber_email: Option<String>,
    #[serde(rename = "tagsTotal")]
    pub tags_total: Option<Int>,
    pub telemetry: Option<Boolean>,
    #[serde(rename = "telemetryClientId")]
    pub telemetry_client_id: Option<String>,
    #[serde(rename = "upgradeCapable")]
    pub upgrade_capable: Option<Boolean>,
    #[serde(rename = "usersTotal")]
    pub users_total: Option<Int>,
    #[serde(rename = "workingDirectory")]
    pub working_directory: Option<String>,
}

#[derive(Deserialize)]
pub struct SystemExtension {
    pub key: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "isInstalled")]
    pub is_installed: Boolean,
    #[serde(rename = "isCompatible")]
    pub is_compatible: Boolean,
}

pub mod system_flag_list {
    use super::*;

    pub struct SystemFlagList;

    pub const OPERATION_NAME: &str = "SystemFlagList";
    pub const QUERY : & str = "query SystemFlagList {\n  system {\n    flags {\n      key\n      value\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub system: Option<System>,
    }

    #[derive(Deserialize, Debug)]
    pub struct System {
        pub flags: Option<Vec<Option<SystemFlag>>>,
    }

    impl graphql_client::GraphQLQuery for SystemFlagList {
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

pub fn system_flag_list(
    client: &Client,
    url: &str,
) -> Result<Vec<SystemFlag>, SystemError> {
    let variables = system_flag_list::Variables {};
    let response = post_graphql::<system_flag_list::SystemFlagList, _>(
        client,
        url,
        variables,
    );
    if response.is_err() {
        return Err(SystemError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(system) = data.system {
            if let Some(flags) = system.flags {
                return Ok(flags
                    .into_iter()
                    .filter_map(|x| x)
                    .collect::<Vec<_>>());
            }
        }
    }
    Err(classify_response_error::<SystemError>(
        response_body.errors,
    ))
}

pub mod system_info_get {
    use super::*;

    pub struct SystemInfoGet;

    pub const OPERATION_NAME: &str = "SystemInfoGet";
    pub const QUERY : & str = "query SystemInfoGet {\n  system {\n    info {\n      configFile\n      cpuCores\n      currentVersion\n      dbHost\n      dbType\n      dbVersion\n      groupsTotal\n      hostname\n      httpPort\n      httpRedirection\n      httpsPort\n      latestVersion\n      latestVersionReleaseDate\n      nodeVersion\n      operatingSystem\n      pagesTotal\n      platform\n      ramTotal\n      sslDomain\n      sslExpirationDate\n      sslProvider\n      sslStatus\n      sslSubscriberEmail\n      tagsTotal\n      telemetry\n      telemetryClientId\n      upgradeCapable\n      usersTotal\n      workingDirectory\n    }\n  }\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub system: Option<System>,
    }

    #[derive(Deserialize)]
    pub struct System {
        pub info: Option<SystemInfo>,
    }

    impl graphql_client::GraphQLQuery for SystemInfoGet {
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

pub fn system_info_get(
    client: &Client,
    url: &str,
) -> Result<SystemInfo, SystemError> {
    let variables = system_info_get::Variables {};
    let response = post_graphql::<system_info_get::SystemInfoGet, _>(
        client,
        url,
        variables,
    );
    if response.is_err() {
        return Err(SystemError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(system) = data.system {
            if let Some(info) = system.info {
                return Ok(info);
            }
        }
    }
    Err(classify_response_error::<SystemError>(
        response_body.errors,
    ))
}

pub mod system_extension_list {
    use super::*;

    pub struct SystemExtensionList;

    pub const OPERATION_NAME: &str = "SystemExtensionList";
    pub const QUERY : & str = "query SystemExtensionList {\n  system {\n    extensions {\n      key\n      title\n      description\n      isInstalled\n      isCompatible\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub system: Option<System>,
    }

    #[derive(Deserialize)]
    pub struct System {
        pub extensions:
            Option<Vec<Option<SystemExtension>>>,
    }

    impl graphql_client::GraphQLQuery for SystemExtensionList {
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

pub fn system_extension_list(
    client: &Client,
    url: &str,
) -> Result<Vec<SystemExtension>, SystemError> {
    let variables = system_extension_list::Variables {};
    let response = post_graphql::<system_extension_list::SystemExtensionList, _>(
        client,
        url,
        variables,
    );
    if response.is_err() {
        return Err(SystemError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(system) = data.system {
            if let Some(extensions) = system.extensions {
                return Ok(extensions
                    .into_iter()
                    .filter_map(|x| x)
                    .collect::<Vec<_>>());
            }
        }
    }
    Err(classify_response_error::<SystemError>(
        response_body.errors,
    ))
}

pub mod system_export_status_get {
    use super::*;

    pub struct SystemExportStatusGet;

    pub const OPERATION_NAME: &str = "SystemExportStatusGet";
    pub const QUERY : & str = "query SystemExportStatusGet{\n  system {\n    exportStatus {\n      status\n      progress\n      message\n      startedAt\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub system: Option<System>,
    }

    #[derive(Deserialize)]
    pub struct System {
        #[serde(rename = "exportStatus")]
        pub export_status: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for SystemExportStatusGet {
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

pub fn system_export_status_get(
    client: &Client,
    url: &str,
) -> Result<(), SystemError> {
    let variables = system_export_status_get::Variables {};
    let response = post_graphql::<system_export_status_get::SystemExportStatusGet, _>(
        client,
        url,
        variables,
    );
    if response.is_err() {
        return Err(SystemError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(system) = data.system {
            if let Some(export_status) = system.export_status {
                if export_status.succeeded {
                    return Ok(());
                } else {
                    return Err(classify_response_status_error::<SystemError>(
                        export_status,
                    ));
                }
            }
        }
    }
    Err(classify_response_error::<SystemError>(
        response_body.errors,
    ))
}
