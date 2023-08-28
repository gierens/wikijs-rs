use serde::{Deserialize, Serialize};
use reqwest::blocking::Client;
use graphql_client::reqwest::post_graphql_blocking as post_graphql;

pub type Boolean = bool;
pub type Int = i64;
pub type Date = String;

#[derive(Deserialize, Debug)]
pub struct Page {
    pub id: Int,
    pub path: String,
    pub hash: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "isPrivate")]
    pub is_private: Boolean,
    #[serde(rename = "isPublished")]
    pub is_published: Boolean,
    #[serde(rename = "privateNS")]
    pub private_ns: Option<String>,
    #[serde(rename = "publishStartDate")]
    pub publish_start_date: Date,
    #[serde(rename = "publishEndDate")]
    pub publish_end_date: Date,
    pub tags: Vec<Option<PageTag>>,
    pub content: String,
    pub toc: Option<String>,
    pub render: Option<String>,
    #[serde(rename = "contentType")]
    pub content_type: String,
    #[serde(rename = "createdAt")]
    pub created_at: Date,
    #[serde(rename = "updatedAt")]
    pub updated_at: Date,
    pub editor: String,
    pub locale: String,
    #[serde(rename = "scriptCss")]
    pub script_css: Option<String>,
    #[serde(rename = "scriptJs")]
    pub script_js: Option<String>,
    #[serde(rename = "authorId")]
    pub author_id: Int,
    #[serde(rename = "authorName")]
    pub author_name: String,
    #[serde(rename = "authorEmail")]
    pub author_email: String,
    #[serde(rename = "creatorId")]
    pub creator_id: Int,
    #[serde(rename = "creatorName")]
    pub creator_name: String,
    #[serde(rename = "creatorEmail")]
    pub creator_email: String,
}

#[derive(Deserialize, Debug)]
pub struct PageListItem {
    pub id: Int,
    pub path: String,
    pub locale: String,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "contentType")]
    pub content_type: String,
    #[serde(rename = "isPublished")]
    pub is_published: Boolean,
    #[serde(rename = "isPrivate")]
    pub is_private: Boolean,
    #[serde(rename = "privateNS")]
    pub private_ns: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Date,
    #[serde(rename = "updatedAt")]
    pub updated_at: Date,
    pub tags: Option<Vec<Option<String>>>,
}

#[derive(Deserialize, Debug)]
pub struct PageTag {
    pub id: Int,
    pub tag: String,
    pub title: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Date,
    #[serde(rename = "updatedAt")]
    pub updated_at: Date,
}

pub(crate) mod get_page_mod {
    use super::*;

    pub struct GetPage;

    pub const OPERATION_NAME: &str = "GetPage";
    pub const QUERY : & str = "query GetPage($id: Int!) {\n  pages {\n    single (id: $id) {\n      id\n      path\n      hash\n      title\n      description\n      isPrivate\n      isPublished\n      privateNS\n      publishStartDate\n      publishEndDate\n      tags {\n        id\n        tag\n        title\n        createdAt\n        updatedAt\n      }\n      content\n      render\n      toc\n      contentType\n      createdAt\n      updatedAt\n      editor\n      locale\n      scriptCss\n      scriptJs\n      authorId\n      authorName\n      authorEmail\n      creatorId\n      creatorName\n      creatorEmail\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub id: Int,
    }

    impl Variables {}

    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub pages: Option<Pages>,
    }

    #[derive(Deserialize, Debug)]
    pub struct Pages {
        pub single: Option<Page>,
    }

    impl graphql_client::GraphQLQuery for GetPage {
        type Variables = Variables;
        type ResponseData = ResponseData;
        fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
            graphql_client::QueryBody {
                variables,
                query: QUERY,
                operation_name: OPERATION_NAME,
            }
        }
    }
}

pub fn get_page(client: &Client, url: &str, id: i64) -> Result<Page, Box<dyn std::error::Error>> {
    let variables = get_page_mod::Variables { id };
    let response_body = post_graphql::<get_page_mod::GetPage, _>(
        client,
        url,
        variables
    )?;

    println!("{:#?}", response_body);
    Ok(response_body.data.unwrap().pages.unwrap().single.unwrap())
}

pub(crate) mod list_all_pages_mod {
    use super::*;

    pub struct ListAllPages;

    pub const OPERATION_NAME: &str = "ListAllPages";
    pub const QUERY : & str = "query ListAllPages {\n  pages {\n    list (orderBy: TITLE) {\n      id\n      path\n      locale\n      title\n      description\n      contentType\n      isPublished\n      isPrivate\n      privateNS\n      createdAt\n      updatedAt\n      tags\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub pages: Option<ListAllPagesPages>,
    }

    #[derive(Deserialize)]
    pub struct ListAllPagesPages {
        pub list: Vec<PageListItem>,
    }

    impl graphql_client::GraphQLQuery for ListAllPages {
        type Variables = Variables;
        type ResponseData = ResponseData;
        fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
            graphql_client::QueryBody {
                variables,
                query: QUERY,
                operation_name: OPERATION_NAME,
            }
        }
    }
}

pub fn list_all_pages(client: &Client, url: &str) -> Result<Vec<PageListItem>, Box<dyn std::error::Error>> {
    let variables = list_all_pages_mod::Variables {};
    let response_body = post_graphql::<list_all_pages_mod::ListAllPages, _>(
        client,
        url,
        variables
    )?;

    Ok(response_body.data.unwrap().pages.unwrap().list)
}

pub(crate) mod list_all_page_tags_mod {
    use super::*;

    pub struct ListAllPageTags;

    pub const OPERATION_NAME: &str = "ListAllPageTags";
    pub const QUERY : & str = "query ListAllPageTags {\n  pages {\n    tags {\n      id\n      tag\n      title\n      createdAt\n      updatedAt\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub pages: Option<Pages>,
    }

    #[derive(Deserialize)]
    pub struct Pages {
        pub tags: Vec<Option<PageTag>>,
    }

    impl graphql_client::GraphQLQuery for ListAllPageTags {
        type Variables = list_all_page_tags_mod::Variables;
        type ResponseData = list_all_page_tags_mod::ResponseData;
        fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
            graphql_client::QueryBody {
                variables,
                query: list_all_page_tags_mod::QUERY,
                operation_name: list_all_page_tags_mod::OPERATION_NAME,
            }
        }
    }
}

pub fn list_all_page_tags(client: &Client, url: &str) -> Result<Vec<PageTag>, Box<dyn std::error::Error>> {
    let variables = list_all_page_tags_mod::Variables {};
    let response_body = post_graphql::<list_all_page_tags_mod::ListAllPageTags, _>(
        client,
        url,
        variables
    )?;

    Ok(response_body.data.unwrap().pages.unwrap().tags.into_iter().flatten().collect())
}
