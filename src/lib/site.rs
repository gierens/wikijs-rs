use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::{
    classify_response_error, classify_response_status_error, Boolean, Int,
    KnownErrorCodes, ResponseStatus, UnknownError,
};

#[derive(Debug, Error, PartialEq)]
pub enum SiteError {
    #[error("Unknown response error code: {code}: {message}")]
    UnknownErrorCode { code: i64, message: String },
    #[error("Unknown response error: {message}")]
    UnknownErrorMessage { message: String },
    #[error("Unknown response error.")]
    UnknownError,
}

impl From<i64> for SiteError {
    fn from(code: i64) -> Self {
        SiteError::UnknownErrorCode {
            code,
            message: "Unknown error".to_string(),
        }
    }
}

impl UnknownError for SiteError {
    fn unknown_error_code(code: i64, message: String) -> Self {
        SiteError::UnknownErrorCode { code, message }
    }
    fn unknown_error_message(message: String) -> Self {
        SiteError::UnknownErrorMessage { message }
    }
    fn unknown_error() -> Self {
        SiteError::UnknownError
    }
}

impl KnownErrorCodes for SiteError {
    fn known_error_codes() -> Vec<i64> {
        Vec::new()
    }

    fn is_known_error_code(_code: i64) -> bool {
        false
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SiteConfig {
    pub host: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub robots: Option<Vec<Option<String>>>,
    #[serde(rename = "analyticsService")]
    pub analytics_service: Option<String>,
    #[serde(rename = "analyticsId")]
    pub analytics_id: Option<String>,
    pub company: Option<String>,
    #[serde(rename = "contentLicense")]
    pub content_license: Option<String>,
    #[serde(rename = "footerOverride")]
    pub footer_override: Option<String>,
    #[serde(rename = "logoUrl")]
    pub logo_url: Option<String>,
    #[serde(rename = "pageExtensions")]
    pub page_extensions: Option<String>,
    #[serde(rename = "authAutoLogin")]
    pub auth_auto_login: Option<Boolean>,
    #[serde(rename = "authEnforce2FA")]
    pub auth_enforce2_fa: Option<Boolean>,
    #[serde(rename = "authHideLocal")]
    pub auth_hide_local: Option<Boolean>,
    #[serde(rename = "authLoginBgUrl")]
    pub auth_login_bg_url: Option<String>,
    #[serde(rename = "authJwtAudience")]
    pub auth_jwt_audience: Option<String>,
    #[serde(rename = "authJwtExpiration")]
    pub auth_jwt_expiration: Option<String>,
    #[serde(rename = "authJwtRenewablePeriod")]
    pub auth_jwt_renewable_period: Option<String>,
    #[serde(rename = "editFab")]
    pub edit_fab: Option<Boolean>,
    #[serde(rename = "editMenuBar")]
    pub edit_menu_bar: Option<Boolean>,
    #[serde(rename = "editMenuBtn")]
    pub edit_menu_btn: Option<Boolean>,
    #[serde(rename = "editMenuExternalBtn")]
    pub edit_menu_external_btn: Option<Boolean>,
    #[serde(rename = "editMenuExternalName")]
    pub edit_menu_external_name: Option<String>,
    #[serde(rename = "editMenuExternalIcon")]
    pub edit_menu_external_icon: Option<String>,
    #[serde(rename = "editMenuExternalUrl")]
    pub edit_menu_external_url: Option<String>,
    #[serde(rename = "featurePageRatings")]
    pub feature_page_ratings: Option<Boolean>,
    #[serde(rename = "featurePageComments")]
    pub feature_page_comments: Option<Boolean>,
    #[serde(rename = "featurePersonalWikis")]
    pub feature_personal_wikis: Option<Boolean>,
    #[serde(rename = "securityOpenRedirect")]
    pub security_open_redirect: Option<Boolean>,
    #[serde(rename = "securityIframe")]
    pub security_iframe: Option<Boolean>,
    #[serde(rename = "securityReferrerPolicy")]
    pub security_referrer_policy: Option<Boolean>,
    #[serde(rename = "securityTrustProxy")]
    pub security_trust_proxy: Option<Boolean>,
    #[serde(rename = "securitySRI")]
    pub security_sri: Option<Boolean>,
    #[serde(rename = "securityHSTS")]
    pub security_hsts: Option<Boolean>,
    #[serde(rename = "securityHSTSDuration")]
    pub security_hsts_duration: Option<Int>,
    #[serde(rename = "securityCSP")]
    pub security_csp: Option<Boolean>,
    #[serde(rename = "securityCSPDirectives")]
    pub security_csp_directives: Option<String>,
    #[serde(rename = "uploadMaxFileSize")]
    pub upload_max_file_size: Option<Int>,
    #[serde(rename = "uploadMaxFiles")]
    pub upload_max_files: Option<Int>,
    #[serde(rename = "uploadScanSVG")]
    pub upload_scan_svg: Option<Boolean>,
    #[serde(rename = "uploadForceDownload")]
    pub upload_force_download: Option<Boolean>,
}

pub mod site_config_get {
    use super::*;

    pub struct SiteConfigGet;

    pub const OPERATION_NAME: &str = "SiteConfigGet";
    pub const QUERY : & str = "query SiteConfigGet {\n  site {\n    config {\n      host\n      title\n      description\n      robots\n      analyticsService\n      analyticsId\n      company\n      contentLicense\n      footerOverride\n      logoUrl\n      pageExtensions\n      authAutoLogin\n      authEnforce2FA\n      authHideLocal\n      authLoginBgUrl\n      authJwtAudience\n      authJwtExpiration\n      authJwtRenewablePeriod\n      editFab\n      editMenuBar\n      editMenuBtn\n      editMenuExternalBtn\n      editMenuExternalName\n      editMenuExternalIcon\n      editMenuExternalUrl\n      featurePageRatings\n      featurePageComments\n      featurePersonalWikis\n      securityOpenRedirect\n      securityIframe\n      securityReferrerPolicy\n      securityTrustProxy\n      securitySRI\n      securityHSTS\n      securityHSTSDuration\n      securityCSP\n      securityCSPDirectives\n      uploadMaxFileSize\n      uploadMaxFiles\n      uploadScanSVG\n      uploadForceDownload\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub site: Option<Site>,
    }

    #[derive(Deserialize)]
    pub struct Site {
        pub config: Option<SiteConfig>,
    }

    impl graphql_client::GraphQLQuery for SiteConfigGet {
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

pub fn site_config_get(
    client: &Client,
    url: &str,
) -> Result<SiteConfig, SiteError> {
    let variables = site_config_get::Variables {};
    let response = post_graphql::<site_config_get::SiteConfigGet, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(SiteError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(site) = data.site {
            if let Some(config) = site.config {
                return Ok(config);
            }
        }
    }
    Err(classify_response_error::<SiteError>(response_body.errors))
}

pub mod site_config_update {
    use super::*;

    pub struct SiteConfigUpdate;

    pub const OPERATION_NAME: &str = "SiteConfigUpdate";
    pub const QUERY : & str = "mutation SiteConfigUpdate(\n  $host: String\n  $title: String\n  $description: String\n  $robots: [String]\n  $analyticsService: String\n  $analyticsId: String\n  $company: String\n  $contentLicense: String\n  $footerOverride: String\n  $logoUrl: String\n  $pageExtensions: String\n  $authAutoLogin: Boolean\n  $authEnforce2FA: Boolean\n  $authHideLocal: Boolean\n  $authLoginBgUrl: String\n  $authJwtAudience: String\n  $authJwtExpiration: String\n  $authJwtRenewablePeriod: String\n  $editFab: Boolean\n  $editMenuBar: Boolean\n  $editMenuBtn: Boolean\n  $editMenuExternalBtn: Boolean\n  $editMenuExternalName: String\n  $editMenuExternalIcon: String\n  $editMenuExternalUrl: String\n  $featurePageRatings: Boolean\n  $featurePageComments: Boolean\n  $featurePersonalWikis: Boolean\n  $securityOpenRedirect: Boolean\n  $securityIframe: Boolean\n  $securityReferrerPolicy: Boolean\n  $securityTrustProxy: Boolean\n  $securitySRI: Boolean\n  $securityHSTS: Boolean\n  $securityHSTSDuration: Int\n  $securityCSP: Boolean\n  $securityCSPDirectives: String\n  $uploadMaxFileSize: Int\n  $uploadMaxFiles: Int\n  $uploadScanSVG: Boolean\n  $uploadForceDownload: Boolean\n) {\n  site {\n    updateConfig(\n      host: $host\n      title: $title\n      description: $description\n      robots: $robots\n      analyticsService: $analyticsService\n      analyticsId: $analyticsId\n      company: $company\n      contentLicense: $contentLicense\n      footerOverride: $footerOverride\n      logoUrl: $logoUrl\n      pageExtensions: $pageExtensions\n      authAutoLogin: $authAutoLogin\n      authEnforce2FA: $authEnforce2FA\n      authHideLocal: $authHideLocal\n      authLoginBgUrl: $authLoginBgUrl\n      authJwtAudience: $authJwtAudience\n      authJwtExpiration: $authJwtExpiration\n      authJwtRenewablePeriod: $authJwtRenewablePeriod\n      editFab: $editFab\n      editMenuBar: $editMenuBar\n      editMenuBtn: $editMenuBtn\n      editMenuExternalBtn: $editMenuExternalBtn\n      editMenuExternalName: $editMenuExternalName\n      editMenuExternalIcon: $editMenuExternalIcon\n      editMenuExternalUrl: $editMenuExternalUrl\n      featurePageRatings: $featurePageRatings\n      featurePageComments: $featurePageComments\n      featurePersonalWikis: $featurePersonalWikis\n      securityOpenRedirect: $securityOpenRedirect\n      securityIframe: $securityIframe\n      securityReferrerPolicy: $securityReferrerPolicy\n      securityTrustProxy: $securityTrustProxy\n      securitySRI: $securitySRI\n      securityHSTS: $securityHSTS\n      securityHSTSDuration: $securityHSTSDuration\n      securityCSP: $securityCSP\n      securityCSPDirectives: $securityCSPDirectives\n      uploadMaxFileSize: $uploadMaxFileSize\n      uploadMaxFiles: $uploadMaxFiles\n      uploadScanSVG: $uploadScanSVG\n      uploadForceDownload: $uploadForceDownload\n    ) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub site: Option<Site>,
    }
    #[derive(Deserialize)]
    pub struct Site {
        #[serde(rename = "updateConfig")]
        pub update_config: Option<UpdateConfig>,
    }
    #[derive(Deserialize)]
    pub struct UpdateConfig {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for SiteConfigUpdate {
        type Variables = SiteConfig;
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

pub fn site_config_update(
    client: &Client,
    url: &str,
    config: SiteConfig,
) -> Result<(), SiteError> {
    let variables = config;
    let response = post_graphql::<site_config_update::SiteConfigUpdate, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(SiteError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(site) = data.site {
            if let Some(update_config) = site.update_config {
                if let Some(response_result) = update_config.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(
                            classify_response_status_error::<SiteError>(
                                response_result,
                            ),
                        );
                    }
                }
            }
        }
    }
    Err(classify_response_error::<SiteError>(response_body.errors))
}
