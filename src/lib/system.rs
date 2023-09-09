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

#[derive(Serialize, Debug)]
pub struct SystemFlagInput {
    pub key: String,
    pub value: Boolean,
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

pub mod system_flags_update {
    use super::*;

    pub struct SystemFlagsUpdate;

    pub const OPERATION_NAME: &str = "SystemFlagsUpdate";
    pub const QUERY : & str = "mutation SystemFlagsUpdate($flags: [SystemFlagInput]!) {\n  system {\n    updateFlags(flags: $flags) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub flags: Vec<Option<SystemFlagInput>>,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub system: Option<System>,
    }

    #[derive(Deserialize)]
    pub struct System {
        #[serde(rename = "updateFlags")]
        pub update_flags: Option<UpdateFlags>,
    }

    #[derive(Deserialize)]
    pub struct UpdateFlags {
        #[serde(rename = "responseResult")]
        pub response_result:
            Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for SystemFlagsUpdate {
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

pub fn system_flags_update(
    client: &Client,
    url: &str,
    flags: Vec<SystemFlagInput>,
) -> Result<(), SystemError> {
    let variables = system_flags_update::Variables {
        flags: flags.into_iter().map(Some).collect(),
    };
    let response = post_graphql::<system_flags_update::SystemFlagsUpdate, _>(
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
            if let Some(update_flags) = system.update_flags {
                if let Some(response_result) = update_flags.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(classify_response_status_error::<SystemError>(
                            response_result,
                        ));
                    }
                }
            }
        }
    }
    Err(classify_response_error::<SystemError>(
        response_body.errors,
    ))
}

pub mod telemetry_client_id_reset {
    // TODO the query file needs to be renamed to match this
    use super::*;

    pub struct TelemetryClientIdReset;

    pub const OPERATION_NAME: &str = "TelemetryClientIdReset";
    pub const QUERY : & str = "mutation TelemetryClientIdReset {\n  system {\n    resetTelemetryClientId {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub system: Option<System>,
    }

    #[derive(Deserialize)]
    pub struct System {
        #[serde(rename = "resetTelemetryClientId")]
        pub reset_telemetry_client_id: Option<ResetTelemetryClientId>,
    }

    #[derive(Deserialize)]
    pub struct ResetTelemetryClientId {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for TelemetryClientIdReset {
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

pub fn telemetry_client_id_reset(
    client: &Client,
    url: &str,
) -> Result<(), SystemError> {
    let variables = telemetry_client_id_reset::Variables {};
    let response = post_graphql::<telemetry_client_id_reset::TelemetryClientIdReset, _>(
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
            if let Some(reset_telemetry_client_id) =
                system.reset_telemetry_client_id
            {
                if let Some(response_result) =
                    reset_telemetry_client_id.response_result
                {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(classify_response_status_error::<SystemError>(
                            response_result,
                        ));
                    }
                }
            }
        }
    }
    Err(classify_response_error::<SystemError>(
        response_body.errors,
    ))
}

pub mod telemetry_set {
    use super::*;

    pub struct TelemetrySet;

    pub const OPERATION_NAME: &str = "TelemetrySet";
    pub const QUERY : & str = "mutation TelemetrySet($enabled: Boolean!) {\n  system {\n    setTelemetry(enabled: $enabled) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub enabled: Boolean,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub system: Option<System>,
    }

    #[derive(Deserialize)]
    pub struct System {
        #[serde(rename = "setTelemetry")]
        pub set_telemetry: Option<SetTelemetry>,
    }

    #[derive(Deserialize)]
    pub struct SetTelemetry {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for TelemetrySet {
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

pub fn telemetry_set(
    client: &Client,
    url: &str,
    enabled: bool,
) -> Result<(), SystemError> {
    let variables = telemetry_set::Variables {
        enabled: enabled.into(),
    };
    let response = post_graphql::<telemetry_set::TelemetrySet, _>(
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
            if let Some(set_telemetry) = system.set_telemetry {
                if let Some(response_result) = set_telemetry.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(classify_response_status_error::<SystemError>(
                            response_result,
                        ));
                    }
                }
            }
        }
    }
    Err(classify_response_error::<SystemError>(
        response_body.errors,
    ))
}

pub mod system_upgrade_perform {
    use super::*;

    pub struct SystemUpgradePerform;

    pub const OPERATION_NAME: &str = "SystemUpgradePerform";
    pub const QUERY : & str = "mutation SystemUpgradePerform {\n  system {\n    performUpgrade {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub system: Option<System>,
    }

    #[derive(Deserialize)]
    pub struct System {
        #[serde(rename = "performUpgrade")]
        pub perform_upgrade: Option<PerformUpgrade>,
    }

    #[derive(Deserialize)]
    pub struct PerformUpgrade {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for SystemUpgradePerform {
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

pub fn system_upgrade_perform(
    client: &Client,
    url: &str,
) -> Result<(), SystemError> {
    let variables = system_upgrade_perform::Variables {};
    let response = post_graphql::<system_upgrade_perform::SystemUpgradePerform, _>(
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
            if let Some(perform_upgrade) = system.perform_upgrade {
                if let Some(response_result) = perform_upgrade.response_result
                {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(classify_response_status_error::<SystemError>(
                            response_result,
                        ));
                    }
                }
            }
        }
    }
    Err(classify_response_error::<SystemError>(
        response_body.errors,
    ))
}

pub mod system_user_import_from_v1 {
    use super::*;

    pub struct SystemUserImportFromV1;

    pub const OPERATION_NAME: &str = "SystemUserImportFromV1";
    pub const QUERY : & str = "mutation SystemUserImportFromV1(\n  $mongoDbConnString: String!\n  $groupCode: SystemImportUsersGroupMode!\n) {\n  system {\n    importUsersFromV1(\n      mongoDbConnString: $mongoDbConnString\n      groupCode: $groupCode\n    ) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "mongoDbConnString")]
        pub mongo_db_conn_string: String,

        #[serde(rename = "groupCode")]
        pub group_code: SystemImportUsersGroupMode,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub system: Option<System>,
    }

    #[derive(Deserialize)]
    pub struct System {
        #[serde(rename = "importUsersFromV1")]
        pub import_users_from_v1: Option<ImportUsersFromV1>,
    }

    #[derive(Deserialize)]
    pub struct ImportUsersFromV1 {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for SystemUserImportFromV1 {
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

pub fn system_user_import_from_v1(
    client: &Client,
    url: &str,
    mongo_db_conn_string: String,
    group_code: SystemImportUsersGroupMode,
) -> Result<(), SystemError> {
    let variables = system_user_import_from_v1::Variables {
        mongo_db_conn_string,
        group_code,
    };
    let response = post_graphql::<system_user_import_from_v1::SystemUserImportFromV1, _>(
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
            if let Some(import_users_from_v1) =
                system.import_users_from_v1
            {
                if let Some(response_result) =
                    import_users_from_v1.response_result
                {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(classify_response_status_error::<SystemError>(
                            response_result,
                        ));
                    }
                }
            }
        }
    }
    Err(classify_response_error::<SystemError>(
        response_body.errors,
    ))
}

// TODO this should be renamed
pub mod system_https_redirection_set {
    use super::*;

    pub struct SystemHttpsRedirectionSet;

    pub const OPERATION_NAME: &str = "SystemHttpsRedirectionSet";
    pub const QUERY : & str = "mutation SystemHttpsRedirectionSet($enabled: Boolean!) {\n  system {\n    setHTTPSRedirection(enabled: $enabled) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub enabled: Boolean,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub system: Option<System>,
    }

    #[derive(Deserialize)]
    pub struct System {
        #[serde(rename = "setHTTPSRedirection")]
        pub set_https_redirection: Option<SetHttpsRedirection>,
    }

    #[derive(Deserialize)]
    pub struct SetHttpsRedirection {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for SystemHttpsRedirectionSet {
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

pub fn https_redirection_set(
    client: &Client,
    url: &str,
    enabled: bool,
) -> Result<(), SystemError> {
    let variables = system_https_redirection_set::Variables {
        enabled: enabled.into(),
    };
    let response = post_graphql::<system_https_redirection_set::SystemHttpsRedirectionSet, _>(
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
            if let Some(set_https_redirection) =
                system.set_https_redirection
            {
                if let Some(response_result) =
                    set_https_redirection.response_result
                {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(classify_response_status_error::<SystemError>(
                            response_result,
                        ));
                    }
                }
            }
        }
    }
    Err(classify_response_error::<SystemError>(
        response_body.errors,
    ))
}

// TODO this should be renamed
pub mod system_https_certificate_renew {
    use super::*;

    pub struct SystemHttpsCertificateRenew;

    pub const OPERATION_NAME: &str = "SystemHttpsCertificateRenew";
    pub const QUERY : & str = "mutation SystemHttpsCertificateRenew {\n  system {\n    renewHTTPSCertificate {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize)]
    pub struct ResponseData { pub system: Option<System>,
    }

    #[derive(Deserialize)]
    pub struct System {
        #[serde(rename = "renewHTTPSCertificate")]
        pub renew_https_certificate: Option<RenewHttpsCertificate>,
    }

    #[derive(Deserialize)]
    pub struct RenewHttpsCertificate {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for SystemHttpsCertificateRenew {
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

pub fn https_certificate_renew(
    client: &Client,
    url: &str,
) -> Result<(), SystemError> {
    let variables = system_https_certificate_renew::Variables {};
    let response = post_graphql::<system_https_certificate_renew::SystemHttpsCertificateRenew, _>(
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
            if let Some(renew_https_certificate) =
                system.renew_https_certificate
            {
                if let Some(response_result) =
                    renew_https_certificate.response_result
                {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(classify_response_status_error::<SystemError>(
                            response_result,
                        ));
                    }
                }
            }
        }
    }
    Err(classify_response_error::<SystemError>(
        response_body.errors,
    ))
}
