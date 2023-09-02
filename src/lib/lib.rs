// #![doc(
//     html_logo_url = "https://raw.githubusercontent.com/gierens/wikijs-rs/main/logo/logo.svg",
//     html_favicon_url = "https://raw.githubusercontent.com/gierens/wikijs-rs/main/logo/favicon.ico"
// )]
use reqwest::blocking::Client;
use reqwest::header::{HeaderValue, AUTHORIZATION};

pub mod asset;
pub mod authentication;
pub mod common;
pub mod contribute;
pub mod group;
pub mod page;
pub mod user;

#[derive(Debug)]
pub enum Credentials {
    Key(String),
    UsernamePassword(String, String, String),
}

#[derive(Debug)]
pub struct Api {
    pub(crate) url: String,
    pub(crate) client: Client,
}

impl Api {
    pub fn new(url: String, credentials: Credentials) -> Self {
        let key = match credentials {
            Credentials::Key(key) => key,
            Credentials::UsernamePassword(username, password, strategy) => {
                let client = Client::builder()
                    .user_agent("wikijs-rs/0.1.0")
                    .build()
                    .unwrap();
                let auth_response = authentication::login(
                    &client,
                    &format!("{}/graphql", url),
                    username,
                    password,
                    strategy,
                )
                .unwrap();
                auth_response.jwt.unwrap()
            }
        };
        Self {
            url,
            client: Client::builder()
                .user_agent("wikijs-rs/0.1.0")
                .default_headers(
                    std::iter::once((
                        AUTHORIZATION,
                        HeaderValue::from_str(&format!("Bearer {}", key))
                            .unwrap(),
                    ))
                    .collect(),
                )
                .build()
                .unwrap(),
        }
    }

    // asset functions
    pub fn asset_list(
        &self,
        folder_id: i64,
        kind: asset::AssetKind,
    ) -> Result<Vec<asset::AssetListItem>, asset::AssetError> {
        asset::asset_list(
            &self.client,
            &format!("{}/graphql", self.url),
            folder_id,
            kind,
        )
    }

    // page functions
    pub fn page_get(&self, id: i64) -> Result<page::Page, page::PageError> {
        page::page_get(&self.client, &format!("{}/graphql", self.url), id)
    }

    pub fn page_tag_list(&self) -> Result<Vec<page::PageTag>, page::PageError> {
        page::page_tag_list(&self.client, &format!("{}/graphql", self.url))
    }

    pub fn page_list(
        &self,
    ) -> Result<Vec<page::PageListItem>, page::PageError> {
        page::page_list(&self.client, &format!("{}/graphql", self.url))
    }

    pub fn page_tree(
        &self,
        parent: i64,
    ) -> Result<Vec<page::PageTreeItem>, page::PageError> {
        page::page_tree(&self.client, &format!("{}/graphql", self.url), parent)
    }

    pub fn page_delete(&self, id: i64) -> Result<(), page::PageError> {
        page::page_delete(&self.client, &format!("{}/graphql", self.url), id)
    }

    pub fn page_render(&self, id: i64) -> Result<(), page::PageError> {
        page::page_render(&self.client, &format!("{}/graphql", self.url), id)
    }

    pub fn page_create(
        &self,
        content: String,
        description: String,
        editor: String,
        is_published: bool,
        is_private: bool,
        locale: String,
        path: String,
        publish_end_date: Option<common::Date>,
        publish_start_date: Option<common::Date>,
        script_css: Option<String>,
        script_js: Option<String>,
        tags: Vec<Option<String>>,
        title: String,
    ) -> Result<(), page::PageError> {
        page::page_create(
            &self.client,
            &format!("{}/graphql", self.url),
            content,
            description,
            editor,
            is_published,
            is_private,
            locale,
            path,
            publish_end_date,
            publish_start_date,
            script_css,
            script_js,
            tags,
            title,
        )
    }

    // authentication functions
    pub fn login(
        &self,
        username: String,
        password: String,
        strategy: String,
    ) -> Result<
        authentication::AuthenticationLoginResponse,
        Box<dyn std::error::Error>,
    > {
        authentication::login(
            &self.client,
            &format!("{}/graphql", self.url),
            username,
            password,
            strategy,
        )
    }

    // contribute functions
    pub fn contributor_list(
        &self,
    ) -> Result<Vec<contribute::Contributor>, contribute::ContributeError> {
        contribute::contributor_list(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }
}
