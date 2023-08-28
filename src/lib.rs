use reqwest::blocking::Client;
use reqwest::header::{HeaderValue, AUTHORIZATION};

mod authentication;
mod page;


#[derive(Debug)]
pub enum Credentials {
    Key(String),
    UsernamePassword(String, String, String),
}


#[derive(Debug)]
pub struct Api {
    url: String,
    client: Client,
}


impl Api {
    pub fn new(url: String, credentials: Credentials) -> Self {
        let key = match credentials {
            Credentials::Key(key) => key,
            Credentials::UsernamePassword(username, password, strategy) => {
                let client =
                    Client::builder()
                        .user_agent("wikijs-rs/0.1.0")
                        .build()
                        .unwrap();
                let auth_response = authentication::login(
                    &client,
                    &format!("{}/graphql", url),
                    username,
                    password,
                    strategy,
                ).unwrap();
                auth_response.jwt.unwrap()
            }
        };
        Self {
            url,
            client:
                Client::builder()
                    .user_agent("wikijs-rs/0.1.0")
                    .default_headers(
                        std::iter::once((
                            AUTHORIZATION,
                            HeaderValue::from_str(&format!("Bearer {}", key))
                                        .unwrap()
                            ))
                        .collect(),
                     )
                    .build()
                    .unwrap(),
        }
    }

    pub fn get_page(&self, id: i64) -> Result<page::Page, Box<dyn std::error::Error>> {
        page::get_page(&self.client, &format!("{}/graphql", self.url), id)
    }

    pub fn list_all_page_tags(&self) -> Result<Vec<page::PageTag>, Box<dyn std::error::Error>> {
        page::list_all_page_tags(&self.client, &format!("{}/graphql", self.url))
    }

    pub fn list_all_pages(&self) -> Result<Vec<page::PageListItem>, Box<dyn std::error::Error>> {
        page::list_all_pages(&self.client, &format!("{}/graphql", self.url))
    }

    pub fn get_page_tree(&self, parent: i64) -> Result<Vec<page::PageTreeItem>, Box<dyn std::error::Error>> {
        page::get_page_tree(&self.client, &format!("{}/graphql", self.url), parent)
    }

    pub fn login(&self, username: String, password: String, strategy: String) -> Result<authentication::AuthenticationLoginResponse, Box<dyn std::error::Error>> {
        authentication::login(&self.client, &format!("{}/graphql", self.url), username, password, strategy)
    }
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}
