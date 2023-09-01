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
