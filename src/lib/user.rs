use serde::Deserialize;

use crate::common::{Boolean, Date, Int, ResponseStatus};
use crate::group::Group;

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
    #[serde(rename = "createAt")]
    pub created_at: Date,
    #[serde(rename = "updatedAt")]
    pub updated_at: Date,
    #[serde(rename = "lastLoginAt")]
    pub last_login_at: Option<Date>,
    #[serde(rename = "tfaIsActive")]
    pub tfa_is_active: Boolean,
    pub groups: Vec<Group>,
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
