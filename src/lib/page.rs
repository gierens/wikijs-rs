use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::{
    classify_response_error, Boolean, Date, Int, ResponseStatus, UnknownError,
};

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
    UnknownErrorCode { code: i64, message: String },
    #[error("Unknown response error: {message}")]
    UnknownErrorMessage { message: String },
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

pub(crate) mod page_get {
    use super::*;

    pub struct PageGet;

    pub const OPERATION_NAME: &str = "PageGet";
    pub const QUERY : & str = "query PageGet($id: Int!) {\n  pages {\n    single (id: $id) {\n      id\n      path\n      hash\n      title\n      description\n      isPrivate\n      isPublished\n      privateNS\n      publishStartDate\n      publishEndDate\n      tags {\n        id\n        tag\n        title\n        createdAt\n        updatedAt\n      }\n      content\n      render\n      toc\n      contentType\n      createdAt\n      updatedAt\n      editor\n      locale\n      scriptCss\n      scriptJs\n      authorId\n      authorName\n      authorEmail\n      creatorId\n      creatorName\n      creatorEmail\n    }\n  }\n}\n" ;

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

    impl graphql_client::GraphQLQuery for PageGet {
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

pub fn page_get(
    client: &Client,
    url: &str,
    id: i64,
) -> Result<Page, PageError> {
    let variables = page_get::Variables { id };
    let response = post_graphql::<page_get::PageGet, _>(client, url, variables);
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

pub(crate) mod page_list {
    use super::*;

    pub struct PageList;

    pub const OPERATION_NAME: &str = "PageList";
    pub const QUERY : & str = "query PageList {\n  pages {\n    list (orderBy: TITLE) {\n      id\n      path\n      locale\n      title\n      description\n      contentType\n      isPublished\n      isPrivate\n      privateNS\n      createdAt\n      updatedAt\n      tags\n    }\n  }\n}\n" ;

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

    impl graphql_client::GraphQLQuery for PageList {
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

pub fn page_list(
    client: &Client,
    url: &str,
) -> Result<Vec<PageListItem>, PageError> {
    let variables = page_list::Variables {};
    let response =
        post_graphql::<page_list::PageList, _>(client, url, variables);
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

pub(crate) mod page_tree {
    use super::*;

    pub struct PageTree;

    pub const OPERATION_NAME: &str = "PageTree";
    pub const QUERY : & str = "query PageTree($parent: Int!) {\n  pages {\n    tree (parent: $parent, mode: ALL, includeAncestors: true, locale: \"en\") {\n      id\n      path\n      depth\n      title\n      isPrivate\n      isFolder\n      privateNS\n      parent\n      pageId\n      locale\n    }\n  }\n}\n" ;

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

    impl graphql_client::GraphQLQuery for PageTree {
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

pub fn page_tree(
    client: &Client,
    url: &str,
    parent: i64,
) -> Result<Vec<PageTreeItem>, PageError> {
    let variables = page_tree::Variables { parent };
    let response =
        post_graphql::<page_tree::PageTree, _>(client, url, variables);
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
            return Ok(pages.tree.unwrap().into_iter().flatten().collect());
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub(crate) mod page_tag_list {
    use super::*;

    pub struct PageTagList;

    pub const OPERATION_NAME: &str = "PageTagList";
    pub const QUERY : & str = "query PageTagList {\n  pages {\n    tags {\n      id\n      tag\n      title\n      createdAt\n      updatedAt\n    }\n  }\n}\n" ;

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

    impl graphql_client::GraphQLQuery for PageTagList {
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

pub fn page_tag_list(
    client: &Client,
    url: &str,
) -> Result<Vec<PageTag>, PageError> {
    let variables = page_tag_list::Variables {};
    let response =
        post_graphql::<page_tag_list::PageTagList, _>(client, url, variables);
    if response.is_err() {
        return Err(PageError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(pages) = data.pages {
            return Ok(pages.tags.into_iter().flatten().collect());
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub mod page_delete {
    use super::*;

    pub struct PageDelete;

    pub const OPERATION_NAME: &str = "PageDelete";
    pub const QUERY : & str = "mutation PageDelete($id: Int!) {\n  pages {\n    delete (id: $id) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub id: Int,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub pages: Option<Pages>,
    }

    #[derive(Deserialize)]
    pub struct Pages {
        pub delete: Option<Delete>,
    }

    #[derive(Deserialize)]
    pub struct Delete {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for PageDelete {
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

pub fn page_delete(
    client: &Client,
    url: &str,
    id: i64,
) -> Result<(), PageError> {
    let variables = page_delete::Variables { id };
    let response =
        post_graphql::<page_delete::PageDelete, _>(client, url, variables);
    if response.is_err() {
        return Err(PageError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(pages) = data.pages {
            if let Some(delete) = pages.delete {
                if let Some(response_result) = delete.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(PageError::from(
                            response_result.error_code,
                        ));
                    }
                }
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub mod page_render {
    use super::*;

    pub struct PageRender;

    pub const OPERATION_NAME: &str = "PageRender";
    pub const QUERY : & str = "mutation PageRender($id: Int!) {\n  pages {\n    render (id: $id) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub id: Int,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub pages: Option<Pages>,
    }

    #[derive(Deserialize)]
    pub struct Pages {
        pub render: Option<Render>,
    }

    #[derive(Deserialize)]
    pub struct Render {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for PageRender {
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

pub fn page_render(
    client: &Client,
    url: &str,
    id: i64,
) -> Result<(), PageError> {
    let variables = page_render::Variables { id };
    let response =
        post_graphql::<page_render::PageRender, _>(client, url, variables);
    if response.is_err() {
        return Err(PageError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(pages) = data.pages {
            if let Some(render) = pages.render {
                if let Some(response_result) = render.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(PageError::from(
                            response_result.error_code,
                        ));
                    }
                }
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub mod page_create {
    use super::*;

    pub struct PageCreate;

    pub const OPERATION_NAME: &str = "PageCreate";
    pub const QUERY : & str = "mutation PageCreate(\n    $content: String!\n    $description: String!\n    $editor: String!\n    $isPublished: Boolean!\n    $isPrivate: Boolean!\n    $locale: String!\n    $path: String!\n    $publishEndDate: Date\n    $publishStartDate: Date\n    $scriptCss: String\n    $scriptJs: String\n    $tags: [String]!\n    $title: String!\n    ) {\n  pages {\n    create (\n      content: $content\n      description: $description\n      editor: $editor\n      isPublished: $isPublished\n      isPrivate: $isPrivate\n      locale: $locale\n      path: $path\n      publishEndDate: $publishEndDate\n      publishStartDate: $publishStartDate\n      scriptCss: $scriptCss\n      scriptJs: $scriptJs\n      tags: $tags\n      title: $title\n    ) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n      page {\n        id\n        path\n        hash\n        title\n        description\n        isPrivate\n        isPublished\n        privateNS\n        publishStartDate\n        publishEndDate\n        tags {\n          id\n          tag\n          title\n          createdAt\n          updatedAt\n        }\n        content\n        render\n        toc\n        contentType\n        createdAt\n        updatedAt\n        editor\n        locale\n        scriptCss\n        scriptJs\n        authorId\n        authorName\n        authorEmail\n        creatorId\n        creatorName\n        creatorEmail\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub content: String,
        pub description: String,
        pub editor: String,
        #[serde(rename = "isPublished")]
        pub is_published: Boolean,
        #[serde(rename = "isPrivate")]
        pub is_private: Boolean,
        pub locale: String,
        pub path: String,
        #[serde(rename = "publishEndDate")]
        pub publish_end_date: Option<Date>,
        #[serde(rename = "publishStartDate")]
        pub publish_start_date: Option<Date>,
        #[serde(rename = "scriptCss")]
        pub script_css: Option<String>,
        #[serde(rename = "scriptJs")]
        pub script_js: Option<String>,
        pub tags: Vec<Option<String>>,
        pub title: String,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub pages: Option<PageCreatePages>,
    }

    #[derive(Deserialize, Debug)]
    pub struct PageCreatePages {
        pub create: Option<Create>,
    }

    #[derive(Deserialize, Debug)]
    pub struct Create {
        #[serde(rename = "responseResult")]
        pub response_result: ResponseStatus,
        pub page: Option<Page>,
    }

    impl graphql_client::GraphQLQuery for PageCreate {
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

#[allow(clippy::too_many_arguments)]
pub fn page_create(
    client: &Client,
    url: &str,
    content: String,
    description: String,
    editor: String,
    is_published: bool,
    is_private: bool,
    locale: String,
    path: String,
    publish_end_date: Option<Date>,
    publish_start_date: Option<Date>,
    script_css: Option<String>,
    script_js: Option<String>,
    tags: Vec<Option<String>>,
    title: String,
) -> Result<(), PageError> {
    let variables = page_create::Variables {
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
    };
    let response =
        post_graphql::<page_create::PageCreate, _>(client, url, variables);
    if response.is_err() {
        return Err(PageError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(pages) = data.pages {
            if let Some(create) = pages.create {
                if create.response_result.succeeded {
                    // unfortunately, the API does not seem to return
                    // the created page so we cannot return it here
                    return Ok(());
                } else {
                    return Err(PageError::from(
                        create.response_result.error_code,
                    ));
                }
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}
