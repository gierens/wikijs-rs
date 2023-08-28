use reqwest::blocking::Client;
use reqwest::header::{HeaderValue, AUTHORIZATION};

mod page;


pub struct Api {
    url: String,
    // key: String,
    client: Client,
}


impl Api {
    pub fn new(url: String, key: String) -> Self {
        Self {
            url,
            // key: key.clone(),
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
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}
