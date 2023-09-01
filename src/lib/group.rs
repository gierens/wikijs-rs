use serde::Deserialize;

use crate::common::{Boolean, Date, Int, ResponseStatus};
use crate::user::UserMinimal;

#[derive(Deserialize, Debug)]
pub struct GroupResponse {
    #[serde(rename = "responseResult")]
    pub response_result: ResponseStatus,
    pub group: Option<Group>,
}

#[derive(Deserialize, Debug)]
pub struct GroupMinimal {
    pub id: Int,
    pub name: String,
    #[serde(rename = "isSystem")]
    pub is_system: Boolean,
    #[serde(rename = "userCount")]
    pub user_count: Option<Int>,
    #[serde(rename = "createAt")]
    pub created_at: Date,
    #[serde(rename = "updatedAt")]
    pub updated_at: Date,
}

#[derive(Deserialize, Debug)]
pub struct Group {
    pub id: Int,
    pub name: String,
    #[serde(rename = "isSystem")]
    pub is_system: Boolean,
    #[serde(rename = "redirectOnLogin")]
    pub redirect_on_login: Option<String>,
    pub permissions: Vec<String>,
    pub page_rules: Vec<PageRule>,
    pub users: Vec<UserMinimal>,
    #[serde(rename = "createAt")]
    pub created_at: Date,
    #[serde(rename = "updatedAt")]
    pub updated_at: Date,
}

#[derive(Deserialize, Debug)]
pub struct PageRule {
    pub id: String,
    pub deny: Boolean,
    pub r#match: PageRuleMatch,
    pub roles: Vec<String>,
    pub path: String,
    pub locales: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct PageRuleInput {
    pub id: String,
    pub deny: Boolean,
    pub r#match: PageRuleMatch,
    pub roles: Vec<String>,
    pub path: String,
    pub locales: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub enum PageRuleMatch {
    START,
    EXACT,
    END,
    REGEX,
    TAG,
}
