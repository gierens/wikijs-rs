use serde::{Deserialize, Serialize};
use reqwest::blocking::Client;
use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use thiserror::Error;

use crate::error::{UnknownError, classify_response_error};

#[derive(Error, Debug, PartialEq)]
pub enum PageError {
    #[error("An unexpected error occurred during a page operation.")]
    PageGenericError,
    #[error("Cannot create this page because an entry already exists at the same path.")]
    PageDuplicateCreate,
    #[error("This page does not exist.")]
    PageNotFound,
    #[error("Page content cannot be empty.")]
    PageEmptyContent,
    #[error("Page path cannot contains illegal characters.")]
    PageIllegalPath,
    #[error("Destination page path already exists.")]
    PagePathCollision,
    #[error("You are not authorized to move this page.")]
    PageMoveForbidden,
    #[error("You are not authorized to create this page.")]
    PageCreateForbidden,
    #[error("You are not authorized to update this page.")]
    PageUpdateForbidden,
    #[error("You are not authorized to delete this page.")]
    PageDeleteForbidden,
    #[error("You are not authorized to restore this page version.")]
    PageRestoreForbidden,
    #[error("You are not authorized to view the history of this page.")]
    PageHistoryForbidden,
    #[error("You are not authorized to view this page.")]
    PageViewForbidden,
    #[error("Unknown response error code: {code}: {message}")]
    UnknownErrorCode {
        code: i64,
        message: String,
    },
    #[error("Unknown response error: {message}")]
    UnknownErrorMessage {
        message: String,
    },
    #[error("Unknown response error.")]
    UnknownError,
}

impl From<i64> for PageError {
    fn from(code: i64) -> Self {
        match code {
            6001 => PageError::PageGenericError,
            6002 => PageError::PageDuplicateCreate,
            6003 => PageError::PageNotFound,
            6004 => PageError::PageEmptyContent,
            6005 => PageError::PageIllegalPath,
            6006 => PageError::PagePathCollision,
            6007 => PageError::PageMoveForbidden,
            6008 => PageError::PageCreateForbidden,
            6009 => PageError::PageUpdateForbidden,
            6010 => PageError::PageDeleteForbidden,
            6011 => PageError::PageRestoreForbidden,
            6012 => PageError::PageHistoryForbidden,
            6013 => PageError::PageViewForbidden,
            _ => PageError::UnknownErrorCode {
                code,
                message: "Unknown error".to_string(),
            },
        }
    }
}

impl UnknownError for PageError {
    fn unknown_error_code(code: i64, message: String) -> Self {
        PageError::UnknownErrorCode { code, message }
    }
    fn unknown_error_message(message: String) -> Self {
        PageError::UnknownErrorMessage { message }
    }
    fn unknown_error() -> Self {
        PageError::UnknownError
    }
}

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
pub struct PageTreeItem {
    pub id: Int,
    pub path: String,
    pub depth: Int,
    pub title: String,
    #[serde(rename = "isPrivate")]
    pub is_private: Boolean,
    #[serde(rename = "isFolder")]
    pub is_folder: Boolean,
    #[serde(rename = "privateNS")]
    pub private_ns: Option<String>,
    pub parent: Option<Int>,
    #[serde(rename = "pageId")]
    pub page_id: Option<Int>,
    pub locale: String,
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

pub fn get_page(client: &Client, url: &str, id: i64) -> Result<Page, PageError> {
    let variables = get_page_mod::Variables { id };
    let response = post_graphql::<get_page_mod::GetPage, _>(
        client,
        url,
        variables
    );
    if response.is_err() {
        return Err(PageError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if response_body.data.is_some() {
        let data = response_body.data.unwrap();
        if data.pages.is_some() {
            let pages = data.pages.unwrap();
            if pages.single.is_some() {
                let page = pages.single.unwrap();
                return Ok(page);
            }
        }
    }
    Err(classify_response_error(response_body.errors))
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
        pub pages: Option<Pages>,
    }

    #[derive(Deserialize)]
    pub struct Pages {
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

pub fn list_all_pages(client: &Client, url: &str) -> Result<Vec<PageListItem>, PageError> {
    let variables = list_all_pages_mod::Variables {};
    let response = post_graphql::<list_all_pages_mod::ListAllPages, _>(
        client,
        url,
        variables
    );
    if response.is_err() {
        return Err(PageError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if response_body.data.is_some() {
        let data = response_body.data.unwrap();
        if data.pages.is_some() {
            let pages = data.pages.unwrap();
            return Ok(pages.list);
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub(crate) mod get_page_tree_mod {
    use super::*;

    pub struct GetPageTree;

    pub const OPERATION_NAME: &str = "GetPageTree";
    pub const QUERY : & str = "query GetPageTree($parent: Int!) {\n  pages {\n    tree (parent: $parent, mode: ALL, includeAncestors: true, locale: \"en\") {\n      id\n      path\n      depth\n      title\n      isPrivate\n      isFolder\n      privateNS\n      parent\n      pageId\n      locale\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub parent: Int,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub pages: Option<Pages>,
    }

    #[derive(Deserialize)]
    pub struct Pages {
        pub tree: Option<Vec<Option<PageTreeItem>>>,
    }

    impl graphql_client::GraphQLQuery for GetPageTree {
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

pub fn get_page_tree(client: &Client, url: &str, parent: i64) -> Result<Vec<PageTreeItem>, PageError> {
    let variables = get_page_tree_mod::Variables { parent };
    let response = post_graphql::<get_page_tree_mod::GetPageTree, _>(
        client,
        url,
        variables
    );
    if response.is_err() {
        return Err(PageError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if response_body.data.is_some() {
        let data = response_body.data.unwrap();
        if data.pages.is_some() {
            let pages = data.pages.unwrap();
            return Ok(pages.tree.unwrap().into_iter().filter_map(|x| x).collect());
        }
    }
    Err(classify_response_error(response_body.errors))
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

pub fn list_all_page_tags(client: &Client, url: &str) -> Result<Vec<PageTag>, PageError> {
    let variables = list_all_page_tags_mod::Variables {};
    let response = post_graphql::<list_all_page_tags_mod::ListAllPageTags, _>(
        client,
        url,
        variables
    );
    if response.is_err() {
        return Err(PageError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if response_body.data.is_some() {
        let data = response_body.data.unwrap();
        if data.pages.is_some() {
            let pages = data.pages.unwrap();
            return Ok(pages.tags.into_iter().filter_map(|x| x).collect());
        }
    }
    Err(classify_response_error(response_body.errors))
}
