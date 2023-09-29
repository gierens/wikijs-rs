use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::{
    classify_response_error, classify_response_status_error, Boolean, Date,
    Int, KnownErrorCodes, ResponseStatus, UnknownError,
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

impl KnownErrorCodes for PageError {
    fn known_error_codes() -> Vec<i64> {
        vec![
            6001, 6002, 6003, 6004, 6005, 6006, 6007, 6008, 6009, 6010, 6011,
            6012, 6013,
        ]
    }

    fn is_known_error_code(code: i64) -> bool {
        (6001..=6013).contains(&code)
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
#[allow(dead_code)]
pub(crate) struct PageMinimal {
    pub(crate) id: Int,
    pub(crate) path: String,
    pub(crate) content: String,
    #[serde(rename = "createdAt")]
    pub(crate) created_at: Date,
    #[serde(rename = "updatedAt")]
    pub(crate) updated_at: Date,
    pub(crate) editor: String,
    pub(crate) locale: String,
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

#[derive(Serialize, Debug)]
pub enum PageTreeMode {
    FOLDERS,
    PAGES,
    ALL,
}

#[derive(Serialize, Debug)]
pub enum PageOrderBy {
    CREATED,
    ID,
    PATH,
    TITLE,
    UPDATED,
}

#[derive(Serialize, Debug)]
pub enum PageOrderByDirection {
    ASC,
    DESC,
}

#[derive(Deserialize, Debug)]
pub struct PageHistoryResult {
    pub trail: Option<Vec<Option<PageHistory>>>,
    pub total: Int,
}

#[derive(Deserialize, Debug)]
pub struct PageHistory {
    #[serde(rename = "versionId")]
    pub version_id: Int,
    #[serde(rename = "versionDate")]
    pub version_date: Date,
    #[serde(rename = "authorId")]
    pub author_id: Int,
    #[serde(rename = "authorName")]
    pub author_name: String,
    #[serde(rename = "actionType")]
    pub action_type: String,
    #[serde(rename = "valueBefore")]
    pub value_before: Option<String>,
    #[serde(rename = "valueAfter")]
    pub value_after: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PageVersion {
    pub action: String,
    #[serde(rename = "authorId")]
    pub author_id: String,
    #[serde(rename = "authorName")]
    pub author_name: String,
    pub content: String,
    #[serde(rename = "contentType")]
    pub content_type: String,
    #[serde(rename = "createdAt")]
    pub created_at: Date,
    #[serde(rename = "versionDate")]
    pub version_date: Date,
    pub description: String,
    pub editor: String,
    #[serde(rename = "isPrivate")]
    pub is_private: Boolean,
    #[serde(rename = "isPublished")]
    pub is_published: Boolean,
    pub locale: String,
    #[serde(rename = "pageId")]
    pub page_id: Int,
    pub path: String,
    #[serde(rename = "publishEndDate")]
    pub publish_end_date: Date,
    #[serde(rename = "publishStartDate")]
    pub publish_start_date: Date,
    pub tags: Vec<Option<String>>,
    pub title: String,
    #[serde(rename = "versionId")]
    pub version_id: Int,
}

#[derive(Deserialize, Debug)]
pub struct PageSearchResponse {
    pub results: Vec<Option<PageSearchResult>>,
    pub suggestions: Vec<Option<String>>,
    #[serde(rename = "totalHits")]
    pub total_hits: Int,
}

#[derive(Deserialize, Debug)]
pub struct PageSearchResult {
    pub id: String,
    pub title: String,
    pub description: String,
    pub path: String,
    pub locale: String,
}

#[derive(Deserialize, Debug)]
pub struct PageLinkItem {
    pub id: Int,
    pub path: String,
    pub title: String,
    pub links: Vec<Option<String>>,
}

#[derive(Deserialize, Debug)]
pub struct PageConflictLatest {
    pub id: Int,
    #[serde(rename = "authorId")]
    pub author_id: String,
    #[serde(rename = "authorName")]
    pub author_name: String,
    pub content: String,
    #[serde(rename = "createdAt")]
    pub created_at: Date,
    pub description: String,
    #[serde(rename = "isPublished")]
    pub is_published: Boolean,
    pub locale: String,
    pub path: String,
    pub tags: Option<Vec<Option<String>>>,
    pub title: String,
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
    pub const QUERY : & str = "query PageList(\n  $limit: Int\n  $orderBy: PageOrderBy\n  $orderByDirection: PageOrderByDirection\n  $tags: [String!]\n  $locale: String\n  $creatorId: Int\n  $authorId: Int\n) {\n  pages {\n    list (\n      limit: $limit\n      orderBy: $orderBy\n      orderByDirection: $orderByDirection\n      tags: $tags\n      locale: $locale\n      creatorId: $creatorId\n      authorId: $authorId\n    ) {\n      id\n      path\n      locale\n      title\n      description\n      contentType\n      isPublished\n      isPrivate\n      privateNS\n      createdAt\n      updatedAt\n      tags\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub limit: Option<Int>,
        #[serde(rename = "orderBy")]
        pub order_by: Option<PageOrderBy>,
        #[serde(rename = "orderByDirection")]
        pub order_by_direction: Option<PageOrderByDirection>,
        pub tags: Option<Vec<String>>,
        pub locale: Option<String>,
        #[serde(rename = "creatorId")]
        pub creator_id: Option<Int>,
        #[serde(rename = "authorId")]
        pub author_id: Option<Int>,
    }

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

#[allow(clippy::too_many_arguments)]
pub fn page_list(
    client: &Client,
    url: &str,
    limit: Option<Int>,
    order_by: Option<PageOrderBy>,
    order_by_direction: Option<PageOrderByDirection>,
    tags: Option<Vec<String>>,
    locale: Option<String>,
    creator_id: Option<Int>,
    author_id: Option<Int>,
) -> Result<Vec<PageListItem>, PageError> {
    let variables = page_list::Variables {
        limit,
        order_by,
        order_by_direction,
        tags,
        locale,
        creator_id,
        author_id,
    };
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
    pub const QUERY : & str = "query PageTree(\n    $parent: Int!\n    $mode: PageTreeMode!\n    $includeAncestors: Boolean!\n    $locale: String!\n    ) {\n  pages {\n    tree (\n      parent: $parent,\n      mode: $mode,\n      includeAncestors: $includeAncestors,\n      locale: $locale\n    ) {\n      id\n      path\n      depth\n      title\n      isPrivate\n      isFolder\n      privateNS\n      parent\n      pageId\n      locale\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub parent: Int,
        pub mode: PageTreeMode,
        #[serde(rename = "includeAncestors")]
        pub include_ancestors: Boolean,
        pub locale: String,
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

// TODO this should be renamed internally too
pub fn page_tree_get(
    client: &Client,
    url: &str,
    parent: i64,
    mode: PageTreeMode,
    include_ancestors: bool,
    locale: String,
) -> Result<Vec<PageTreeItem>, PageError> {
    let variables = page_tree::Variables {
        parent,
        mode,
        include_ancestors,
        locale,
    };
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

pub(crate) mod page_delete {
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

pub(crate) mod page_render {
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

pub(crate) mod page_create {
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
                    return Err(classify_response_status_error(
                        create.response_result,
                    ));
                }
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub(crate) mod page_get_by_path {
    use super::*;

    pub struct PageGetByPath;

    pub const OPERATION_NAME: &str = "PageGetByPath";
    pub const QUERY : & str = "query PageGetByPath($path: String!, $locale: String!) {\n  pages {\n    singleByPath (path: $path, locale: $locale) {\n      id\n      path\n      hash\n      title\n      description\n      isPrivate\n      isPublished\n      privateNS\n      publishStartDate\n      publishEndDate\n      tags {\n        id\n        tag\n        title\n        createdAt\n        updatedAt\n      }\n      content\n      render\n      toc\n      contentType\n      createdAt\n      updatedAt\n      editor\n      locale\n      scriptCss\n      scriptJs\n      authorId\n      authorName\n      authorEmail\n      creatorId\n      creatorName\n      creatorEmail\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub path: String,
        pub locale: String,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub pages: Option<Pages>,
    }

    #[derive(Deserialize)]
    pub struct Pages {
        #[serde(rename = "singleByPath")]
        pub single_by_path: Option<Page>,
    }

    impl graphql_client::GraphQLQuery for PageGetByPath {
        type Variables = page_get_by_path::Variables;
        type ResponseData = page_get_by_path::ResponseData;
        fn build_query(
            variables: Self::Variables,
        ) -> ::graphql_client::QueryBody<Self::Variables> {
            graphql_client::QueryBody {
                variables,
                query: page_get_by_path::QUERY,
                operation_name: page_get_by_path::OPERATION_NAME,
            }
        }
    }
}

pub fn page_get_by_path(
    client: &Client,
    url: &str,
    path: String,
    locale: String,
) -> Result<Page, PageError> {
    let variables = page_get_by_path::Variables { path, locale };
    let response = post_graphql::<page_get_by_path::PageGetByPath, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(PageError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(pages) = data.pages {
            if let Some(page) = pages.single_by_path {
                return Ok(page);
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub(crate) mod page_update {
    use super::*;

    pub struct PageUpdate;

    pub const OPERATION_NAME: &str = "PageUpdate";
    pub const QUERY : & str = "mutation PageUpdate(\n    $id: Int!\n    $content: String\n    $description: String\n    $editor: String\n    $isPrivate: Boolean\n    $isPublished: Boolean\n    $locale: String\n    $path: String\n    $publishEndDate: Date\n    $publishStartDate: Date\n    $scriptCss: String\n    $scriptJs: String\n    $tags: [String]\n    $title: String\n    ) {\n  pages {\n    update (\n      id: $id\n      content: $content\n      description: $description\n      editor: $editor\n      isPrivate: $isPrivate\n      isPublished: $isPublished\n      locale: $locale\n      path: $path\n      publishEndDate: $publishEndDate\n      publishStartDate: $publishStartDate\n      scriptCss: $scriptCss\n      scriptJs: $scriptJs\n      tags: $tags\n      title: $title\n    ) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n      page {\n        id\n        path\n        hash\n        title\n        description\n        isPrivate\n        isPublished\n        privateNS\n        publishStartDate\n        publishEndDate\n        tags {\n          id\n          tag\n          title\n          createdAt\n          updatedAt\n        }\n        content\n        render\n        toc\n        contentType\n        createdAt\n        updatedAt\n        editor\n        locale\n        scriptCss\n        scriptJs\n        authorId\n        authorName\n        authorEmail\n        creatorId\n        creatorName\n        creatorEmail\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub id: Int,
        pub content: Option<String>,
        pub description: Option<String>,
        pub editor: Option<String>,
        #[serde(rename = "isPrivate")]
        pub is_private: Option<Boolean>,
        #[serde(rename = "isPublished")]
        pub is_published: Option<Boolean>,
        pub locale: Option<String>,
        pub path: Option<String>,
        #[serde(rename = "publishEndDate")]
        pub publish_end_date: Option<Date>,
        #[serde(rename = "publishStartDate")]
        pub publish_start_date: Option<Date>,
        #[serde(rename = "scriptCss")]
        pub script_css: Option<String>,
        #[serde(rename = "scriptJs")]
        pub script_js: Option<String>,
        pub tags: Option<Vec<Option<String>>>,
        pub title: Option<String>,
    }

    impl Variables {}

    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub pages: Option<Pages>,
    }

    #[derive(Deserialize, Debug)]
    pub struct Pages {
        pub update: Option<Update>,
    }

    #[derive(Deserialize, Debug)]
    pub struct Update {
        #[serde(rename = "responseResult")]
        pub response_result: ResponseStatus,
        pub page: Option<Page>,
    }

    impl graphql_client::GraphQLQuery for PageUpdate {
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
pub fn page_update(
    client: &Client,
    url: &str,
    id: i64,
    content: Option<String>,
    description: Option<String>,
    editor: Option<String>,
    is_private: Option<bool>,
    is_published: Option<bool>,
    locale: Option<String>,
    path: Option<String>,
    publish_end_date: Option<Date>,
    publish_start_date: Option<Date>,
    script_css: Option<String>,
    script_js: Option<String>,
    tags: Option<Vec<Option<String>>>,
    title: Option<String>,
) -> Result<(), PageError> {
    let page = page_get(client, url, id)?;
    let variables = page_update::Variables {
        id,
        content: content.or(Some(page.content)),
        description: description.or(Some(page.description)),
        editor: editor.or(Some(page.editor)),
        is_private: is_private.or(Some(page.is_private)),
        is_published: is_published.or(Some(page.is_published)),
        locale: locale.or(Some(page.locale)),
        path: path.or(Some(page.path)),
        publish_end_date: publish_end_date.or(Some(page.publish_end_date)),
        publish_start_date: publish_start_date
            .or(Some(page.publish_start_date)),
        script_css: script_css.or(page.script_css),
        script_js: script_js.or(page.script_js),
        tags: tags.or(Some(
            page.tags.into_iter().map(|t| t.map(|t| t.tag)).collect(),
        )),
        title: title.or(Some(page.title)),
    };
    let response =
        post_graphql::<page_update::PageUpdate, _>(client, url, variables);
    if response.is_err() {
        return Err(PageError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(pages) = data.pages {
            if let Some(update) = pages.update {
                if update.response_result.succeeded {
                    // unfortunately, the API does not seem to return
                    // the updated page so we cannot return it here
                    return Ok(());
                } else {
                    return Err(classify_response_status_error(
                        update.response_result,
                    ));
                }
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}

// pub(crate) mod page_update_content {
//     use super::*;
//
//     pub struct PageUpdateContent;
//
//     pub const OPERATION_NAME: &str = "PageUpdateContent";
//     pub const QUERY : & str = "mutation PageUpdateContent(\n    $id: Int!\n    $content: String!\n    ) {\n  pages {\n    update (\n      id: $id\n      content: $content\n    ) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n      page {\n        id\n        path\n        hash\n        title\n        description\n        isPrivate\n        isPublished\n        privateNS\n        publishStartDate\n        publishEndDate\n        tags {\n          id\n          tag\n          title\n          createdAt\n          updatedAt\n        }\n        content\n        render\n        toc\n        contentType\n        createdAt\n        updatedAt\n        editor\n        locale\n        scriptCss\n        scriptJs\n        authorId\n        authorName\n        authorEmail\n        creatorId\n        creatorName\n        creatorEmail\n      }\n    }\n  }\n}\n" ;
//
//     #[derive(Serialize, Debug)]
//     pub struct Variables {
//         pub id: Int,
//         pub content: String,
//     }
//
//     impl Variables {}
//
//     #[derive(Deserialize, Debug)]
//     pub struct ResponseData {
//         pub pages: Option<Pages>,
//     }
//
//     #[derive(Deserialize, Debug)]
//     pub struct Pages {
//         pub update: Option<Update>,
//     }
//
//     #[derive(Deserialize, Debug)]
//     pub struct Update {
//         #[serde(rename = "responseResult")]
//         pub response_result: ResponseStatus,
//         pub page: Option<Page>,
//     }
//
//     impl graphql_client::GraphQLQuery for PageUpdateContent {
//         type Variables = Variables;
//         type ResponseData = ResponseData;
//         fn build_query(
//             variables: Self::Variables,
//         ) -> ::graphql_client::QueryBody<Self::Variables> {
//             graphql_client::QueryBody {
//                 variables,
//                 query: QUERY,
//                 operation_name: OPERATION_NAME,
//             }
//         }
//     }
// }
//
// pub fn page_update_content(
//     client: &Client,
//     url: &str,
//     id: i64,
//     content: String,
// ) -> Result<(), PageError> {
//     let variables = page_update_content::Variables { id, content };
//     let response = post_graphql::<page_update_content::PageUpdateContent, _>(
//         client, url, variables,
//     );
//     if response.is_err() {
//         return Err(PageError::UnknownErrorMessage {
//             message: response.err().unwrap().to_string(),
//         });
//     }
//     let response_body = response.unwrap();
//     if let Some(data) = response_body.data {
//         if let Some(pages) = data.pages {
//             if let Some(update) = pages.update {
//                 if update.response_result.succeeded {
//                     // unfortunately, the API does not seem to return
//                     // the updated page so we cannot return it here
//                     return Ok(());
//                 } else {
//                     return Err(classify_response_status_error(
//                         update.response_result,
//                     ));
//                 }
//             }
//         }
//     }
//     Err(classify_response_error(response_body.errors))
// }

pub(crate) mod page_history_get {
    use super::*;

    pub struct PageHistoryGet;

    pub const OPERATION_NAME: &str = "PageHistoryGet";
    pub const QUERY : & str = "query PageHistoryGet(\n  $id: Int!\n  $offsetPage: Int\n  $offsetSize: Int\n) {\n  pages {\n    history(\n      id: $id\n      offsetPage: $offsetPage\n      offsetSize: $offsetSize\n    ) {\n      trail {\n        versionId\n        versionDate\n        authorId\n        authorName\n        actionType\n        valueBefore\n        valueAfter\n      }\n      total\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub id: Int,
        #[serde(rename = "offsetPage")]
        pub offset_page: Option<Int>,
        #[serde(rename = "offsetSize")]
        pub offset_size: Option<Int>,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub pages: Option<Pages>,
    }

    #[derive(Deserialize)]
    pub struct Pages {
        pub history: Option<PageHistoryResult>,
    }

    impl graphql_client::GraphQLQuery for PageHistoryGet {
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

pub fn page_history_get(
    client: &Client,
    url: &str,
    id: i64,
    offset_page: Option<i64>,
    offset_size: Option<i64>,
) -> Result<PageHistoryResult, PageError> {
    let variables = page_history_get::Variables {
        id,
        offset_page,
        offset_size,
    };
    let response = post_graphql::<page_history_get::PageHistoryGet, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(PageError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(pages) = data.pages {
            if let Some(history) = pages.history {
                return Ok(history);
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub(crate) mod page_version_get {
    use super::*;

    pub struct PageVersionGet;

    pub const OPERATION_NAME: &str = "PageVersionGet";
    pub const QUERY : & str = "query PageVersionGet(\n  $pageId: Int!\n  $versionId: Int!\n) {\n  pages {\n    version(\n      pageId: $pageId\n      versionId: $versionId\n    ) {\n      action\n      authorId\n      authorName\n      content\n      contentType\n      createdAt\n      versionDate\n      description\n      editor\n      isPrivate\n      isPublished\n      locale\n      pageId\n      path\n      publishEndDate\n      publishStartDate\n      tags\n      title\n      versionId\n    }\n  }\n}\n" ;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "pageId")]
        pub page_id: Int,
        #[serde(rename = "versionId")]
        pub version_id: Int,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub pages: Option<Pages>,
    }
    #[derive(Deserialize)]
    pub struct Pages {
        pub version: Option<PageVersion>,
    }

    impl graphql_client::GraphQLQuery for PageVersionGet {
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

pub fn page_version_get(
    client: &Client,
    url: &str,
    page_id: i64,
    version_id: i64,
) -> Result<PageVersion, PageError> {
    let variables = page_version_get::Variables {
        page_id,
        version_id,
    };
    let response = post_graphql::<page_version_get::PageVersionGet, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(PageError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(pages) = data.pages {
            if let Some(version) = pages.version {
                return Ok(version);
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub(crate) mod page_search {
    use super::*;

    pub struct PageSearch;

    pub const OPERATION_NAME: &str = "PageSearch";
    pub const QUERY : & str = "query PageSearch(\n  $query: String!\n  $path: String\n  $locale: String\n) {\n  pages {\n    search(\n      query: $query\n      path: $path\n      locale: $locale\n    ) {\n      results {\n        id\n        title\n        description\n        path\n        locale\n      }\n      suggestions\n      totalHits\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub query: String,
        pub path: Option<String>,
        pub locale: Option<String>,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub pages: Option<Pages>,
    }

    #[derive(Deserialize)]
    pub struct Pages {
        pub search: PageSearchResponse,
    }

    impl graphql_client::GraphQLQuery for PageSearch {
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

pub fn page_search(
    client: &Client,
    url: &str,
    query: String,
    path: Option<String>,
    locale: Option<String>,
) -> Result<PageSearchResponse, PageError> {
    let variables = page_search::Variables {
        query,
        path,
        locale,
    };
    let response =
        post_graphql::<page_search::PageSearch, _>(client, url, variables);
    if response.is_err() {
        return Err(PageError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(pages) = data.pages {
            return Ok(pages.search);
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub(crate) mod page_link_get {
    use super::*;

    pub struct PageLinkGet;

    pub const OPERATION_NAME: &str = "PageLinkGet";
    pub const QUERY : & str = "query PageLinkGet(\n    $locale: String!\n    ) {\n  pages {\n    links (\n      locale: $locale\n    ) {\n      id\n      path\n      title\n      links\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub locale: String,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub pages: Option<Pages>,
    }

    #[derive(Deserialize)]
    pub struct Pages {
        pub links: Option<Vec<Option<PageLinkItem>>>,
    }

    impl graphql_client::GraphQLQuery for PageLinkGet {
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

// TODO internals should be renamed accordingly
pub fn page_link_list(
    client: &Client,
    url: &str,
    locale: String,
) -> Result<Vec<PageLinkItem>, PageError> {
    let variables = page_link_get::Variables { locale };
    let response =
        post_graphql::<page_link_get::PageLinkGet, _>(client, url, variables);
    if response.is_err() {
        return Err(PageError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(pages) = data.pages {
            if let Some(links) = pages.links {
                return Ok(links.into_iter().flatten().collect());
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub(crate) mod page_conflict_check {
    use super::*;

    pub struct PageConflictCheck;

    pub const OPERATION_NAME: &str = "PageConflictCheck";
    pub const QUERY : & str = "query PageConflictCheck(\n  $id: Int!\n  $checkoutDate: Date!\n) {\n  pages {\n    checkConflicts (\n      id: $id\n      checkoutDate: $checkoutDate\n    )\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub id: Int,
        #[serde(rename = "checkoutDate")]
        pub checkout_date: Date,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub pages: Option<Pages>,
    }

    #[derive(Deserialize)]
    pub struct Pages {
        #[serde(rename = "checkConflicts")]
        pub check_conflicts: Boolean,
    }

    impl graphql_client::GraphQLQuery for PageConflictCheck {
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

pub fn page_conflict_check(
    client: &Client,
    url: &str,
    id: i64,
    checkout_date: Date,
) -> Result<Boolean, PageError> {
    let variables = page_conflict_check::Variables { id, checkout_date };
    let response = post_graphql::<page_conflict_check::PageConflictCheck, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(PageError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(pages) = data.pages {
            return Ok(pages.check_conflicts);
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub(crate) mod page_conflict_latest {
    use super::*;

    pub struct PageConflictLatestFunction;

    pub const OPERATION_NAME: &str = "PageConflictLatest";
    pub const QUERY : & str = "query PageConflictLatest (\n  $id: Int!\n) {\n  pages {\n    conflictLatest (\n      id: $id\n    ) {\n      id\n      authorId\n      authorName\n      content\n      createdAt\n      description\n      isPublished\n      locale\n      path\n      tags\n      title\n      updatedAt\n    }\n  }\n}\n" ;

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
        #[serde(rename = "conflictLatest")]
        pub conflict_latest: PageConflictLatest,
    }

    impl graphql_client::GraphQLQuery for PageConflictLatestFunction {
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

pub fn page_conflict_latest(
    client: &Client,
    url: &str,
    id: i64,
) -> Result<PageConflictLatest, PageError> {
    let variables = page_conflict_latest::Variables { id };
    let response = post_graphql::<
        page_conflict_latest::PageConflictLatestFunction,
        _,
    >(client, url, variables);
    if response.is_err() {
        return Err(PageError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(pages) = data.pages {
            return Ok(pages.conflict_latest);
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub(crate) mod page_convert {
    use super::*;

    pub struct PageConvert;

    pub const OPERATION_NAME: &str = "PageConvert";
    pub const QUERY : & str = "mutation PageConvert($id: Int!, $editor: String!) {\n  pages {\n    convert (id: $id, editor: $editor) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub id: Int,
        pub editor: String,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub pages: Option<Pages>,
    }

    #[derive(Deserialize)]
    pub struct Pages {
        pub convert: Option<Convert>,
    }
    #[derive(Deserialize)]
    pub struct Convert {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for PageConvert {
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

pub fn page_convert(
    client: &Client,
    url: &str,
    id: i64,
    editor: String,
) -> Result<(), PageError> {
    let variables = page_convert::Variables { id, editor };
    let response =
        post_graphql::<page_convert::PageConvert, _>(client, url, variables);
    if response.is_err() {
        return Err(PageError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();

    if let Some(data) = response_body.data {
        if let Some(pages) = data.pages {
            if let Some(convert) = pages.convert {
                if let Some(response_result) = convert.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(classify_response_status_error(
                            response_result,
                        ));
                    }
                }
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub(crate) mod page_move {
    use super::*;

    pub struct PageMove;

    pub const OPERATION_NAME: &str = "PageMove";
    pub const QUERY : & str = "mutation PageMove(\n  $id: Int!\n  $destinationPath: String!\n  $destinationLocale: String!\n) {\n  pages {\n    move (\n      id: $id\n      destinationPath: $destinationPath\n      destinationLocale: $destinationLocale\n    ) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub id: Int,
        #[serde(rename = "destinationPath")]
        pub destination_path: String,
        #[serde(rename = "destinationLocale")]
        pub destination_locale: String,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub pages: Option<Pages>,
    }
    #[derive(Deserialize)]
    pub struct Pages {
        #[serde(rename = "move")]
        pub move_: Option<Move>,
    }

    #[derive(Deserialize)]
    pub struct Move {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for PageMove {
        type Variables = Variables;
        type ResponseData = ResponseData;
        fn build_query(
            variables: Self::Variables,
        ) -> ::graphql_client::QueryBody<Self::Variables> {
            ::graphql_client::QueryBody {
                variables,
                query: QUERY,
                operation_name: OPERATION_NAME,
            }
        }
    }
}

pub fn page_move(
    client: &Client,
    url: &str,
    id: i64,
    destination_path: String,
    destination_locale: String,
) -> Result<(), PageError> {
    let variables = page_move::Variables {
        id,
        destination_path,
        destination_locale,
    };
    let response =
        post_graphql::<page_move::PageMove, _>(client, url, variables);
    if response.is_err() {
        return Err(PageError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }

    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(pages) = data.pages {
            if let Some(move_) = pages.move_ {
                if let Some(response_result) = move_.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(classify_response_status_error(
                            response_result,
                        ));
                    }
                }
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub(crate) mod page_tag_delete {
    use super::*;

    pub struct PageTagDelete;

    pub const OPERATION_NAME: &str = "PageTagDelete";
    pub const QUERY : & str = "mutation PageTagDelete($id: Int!) {\n  pages {\n    deleteTag (id: $id) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

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
        #[serde(rename = "deleteTag")]
        pub delete_tag: Option<DeleteTag>,
    }
    #[derive(Deserialize)]
    pub struct DeleteTag {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for PageTagDelete {
        type Variables = Variables;
        type ResponseData = ResponseData;
        fn build_query(
            variables: Self::Variables,
        ) -> ::graphql_client::QueryBody<Self::Variables> {
            ::graphql_client::QueryBody {
                variables,
                query: QUERY,
                operation_name: OPERATION_NAME,
            }
        }
    }
}

pub fn page_tag_delete(
    client: &Client,
    url: &str,
    id: i64,
) -> Result<(), PageError> {
    let variables = page_tag_delete::Variables { id };
    let response = post_graphql::<page_tag_delete::PageTagDelete, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(PageError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }

    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(pages) = data.pages {
            if let Some(delete_tag) = pages.delete_tag {
                if let Some(response_result) = delete_tag.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(classify_response_status_error(
                            response_result,
                        ));
                    }
                }
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub(crate) mod page_tag_update {
    use super::*;

    pub struct PageTagUpdate;

    pub const OPERATION_NAME: &str = "PageTagUpdate";
    pub const QUERY : & str = "mutation PageTagUpdate(\n  $id: Int!\n  $tag: String!\n  $title: String!\n) {\n  pages {\n    updateTag (\n      id: $id\n      tag: $tag\n      title: $title\n    ) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub id: Int,
        pub tag: String,
        pub title: String,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub pages: Option<Pages>,
    }

    #[derive(Deserialize)]
    pub struct Pages {
        #[serde(rename = "updateTag")]
        pub update_tag: Option<UpdateTag>,
    }

    #[derive(Deserialize)]
    pub struct UpdateTag {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for PageTagUpdate {
        type Variables = Variables;
        type ResponseData = ResponseData;
        fn build_query(
            variables: Self::Variables,
        ) -> ::graphql_client::QueryBody<Self::Variables> {
            ::graphql_client::QueryBody {
                variables,
                query: QUERY,
                operation_name: OPERATION_NAME,
            }
        }
    }
}

pub fn page_tag_update(
    client: &Client,
    url: &str,
    id: i64,
    tag: String,
    title: String,
) -> Result<(), PageError> {
    let variables = page_tag_update::Variables { id, tag, title };
    let response = post_graphql::<page_tag_update::PageTagUpdate, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(PageError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }

    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(pages) = data.pages {
            if let Some(update_tag) = pages.update_tag {
                if let Some(response_result) = update_tag.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(classify_response_status_error(
                            response_result,
                        ));
                    }
                }
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub(crate) mod page_cache_flush {
    use super::*;

    pub struct PageCacheFlush;

    pub const OPERATION_NAME: &str = "PageCacheFlush";
    pub const QUERY : & str = "mutation PageCacheFlush {\n  pages {\n    flushCache {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub pages: Option<Pages>,
    }

    #[derive(Deserialize)]
    pub struct Pages {
        #[serde(rename = "flushCache")]
        pub flush_cache: Option<FlushCache>,
    }

    #[derive(Deserialize)]
    pub struct FlushCache {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for PageCacheFlush {
        type Variables = Variables;
        type ResponseData = ResponseData;
        fn build_query(
            _variables: Self::Variables,
        ) -> ::graphql_client::QueryBody<Self::Variables> {
            ::graphql_client::QueryBody {
                variables: Variables {},
                query: QUERY,
                operation_name: OPERATION_NAME,
            }
        }
    }
}

pub fn page_cache_flush(client: &Client, url: &str) -> Result<(), PageError> {
    let variables = page_cache_flush::Variables {};
    let response = post_graphql::<page_cache_flush::PageCacheFlush, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(PageError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }

    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(pages) = data.pages {
            if let Some(flush_cache) = pages.flush_cache {
                if let Some(response_result) = flush_cache.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(classify_response_status_error(
                            response_result,
                        ));
                    }
                }
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub mod page_migrate_to_locale {
    use super::*;

    pub struct PageMigrateToLocale;

    pub const OPERATION_NAME: &str = "PageMigrateToLocale";
    pub const QUERY : & str = "mutation PageMigrateToLocale(\n  $sourceLocale: String!\n  $targetLocale: String!\n) {\n  pages {\n    migrateToLocale(\n      sourceLocale: $sourceLocale\n      targetLocale: $targetLocale\n    ) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "sourceLocale")]
        pub source_locale: String,
        #[serde(rename = "targetLocale")]
        pub target_locale: String,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub pages: Option<Pages>,
    }

    #[derive(Deserialize)]
    pub struct Pages {
        #[serde(rename = "migrateToLocale")]
        pub migrate_to_locale: Option<MigrateToLocale>,
    }
    #[derive(Deserialize)]
    pub struct MigrateToLocale {
        #[serde(rename = "responseResult")]
        pub response_result: ResponseStatus,
    }

    impl graphql_client::GraphQLQuery for PageMigrateToLocale {
        type Variables = Variables;
        type ResponseData = ResponseData;
        fn build_query(
            variables: Self::Variables,
        ) -> ::graphql_client::QueryBody<Self::Variables> {
            ::graphql_client::QueryBody {
                variables,
                query: QUERY,
                operation_name: OPERATION_NAME,
            }
        }
    }
}

pub fn page_migrate_to_locale(
    client: &Client,
    url: &str,
    source_locale: String,
    target_locale: String,
) -> Result<(), PageError> {
    let variables = page_migrate_to_locale::Variables {
        source_locale,
        target_locale,
    };
    let response = post_graphql::<page_migrate_to_locale::PageMigrateToLocale, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(PageError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }

    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(pages) = data.pages {
            if let Some(migrate_to_locale) = pages.migrate_to_locale {
                if migrate_to_locale.response_result.succeeded {
                    return Ok(());
                } else {
                    return Err(classify_response_status_error(
                        migrate_to_locale.response_result,
                    ));
                }
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub mod page_tree_rebuild {
    use super::*;

    pub struct PageTreeRebuild;

    pub const OPERATION_NAME: &str = "PageTreeRebuild";
    pub const QUERY : & str = "mutation PageTreeRebuild {\n  pages {\n    rebuildTree {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub pages: Option<Pages>,
    }

    #[derive(Deserialize)]
    pub struct Pages {
        #[serde(rename = "rebuildTree")]
        pub rebuild_tree: Option<RebuildTree>,
    }

    #[derive(Deserialize)]
    pub struct RebuildTree {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for PageTreeRebuild {
        type Variables = Variables;
        type ResponseData = ResponseData;
        fn build_query(
            _variables: Self::Variables,
        ) -> ::graphql_client::QueryBody<Self::Variables> {
            ::graphql_client::QueryBody {
                variables: Variables {},
                query: QUERY,
                operation_name: OPERATION_NAME,
            }
        }
    }
}

pub fn page_tree_rebuild(client: &Client, url: &str) -> Result<(), PageError> {
    let variables = page_tree_rebuild::Variables {};
    let response = post_graphql::<page_tree_rebuild::PageTreeRebuild, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(PageError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }

    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(pages) = data.pages {
            if let Some(rebuild_tree) = pages.rebuild_tree {
                if let Some(response_result) = rebuild_tree.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(classify_response_status_error(
                            response_result,
                        ));
                    }
                }
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub mod page_restore {
    use super::*;

    pub struct PageRestore;

    pub const OPERATION_NAME: &str = "PageRestore";
    pub const QUERY : & str = "mutation PageRestore(\n  $pageId: Int!\n  $versionId: Int!\n) {\n  pages {\n    restore (\n      pageId: $pageId\n      versionId: $versionId \n    ) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "pageId")]
        pub page_id: Int,
        #[serde(rename = "versionId")]
        pub version_id: Int,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub pages: Option<Pages>,
    }

    #[derive(Deserialize)]
    pub struct Pages {
        pub restore: Option<Restore>,
    }

    #[derive(Deserialize)]
    pub struct Restore {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for PageRestore {
        type Variables = Variables;
        type ResponseData = ResponseData;
        fn build_query(
            variables: Self::Variables,
        ) -> ::graphql_client::QueryBody<Self::Variables> {
            ::graphql_client::QueryBody {
                variables,
                query: QUERY,
                operation_name: OPERATION_NAME,
            }
        }
    }
}

pub fn page_restore(
    client: &Client,
    url: &str,
    page_id: i64,
    version_id: i64,
) -> Result<(), PageError> {
    let variables = page_restore::Variables {
        page_id,
        version_id,
    };
    let response =
        post_graphql::<page_restore::PageRestore, _>(client, url, variables);
    if response.is_err() {
        return Err(PageError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }

    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(pages) = data.pages {
            if let Some(restore) = pages.restore {
                if let Some(response_result) = restore.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(classify_response_status_error(
                            response_result,
                        ));
                    }
                }
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub mod page_history_purge {
    use super::*;

    pub struct PageHistoryPurge;

    pub const OPERATION_NAME: &str = "PageHistoryPurge";
    pub const QUERY : & str = "mutation PageHistoryPurge(\n  $olderThan: String!\n) {\n  pages {\n    purgeHistory (\n      olderThan: $olderThan\n    ) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "olderThan")]
        pub older_than: String,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub pages: Option<Pages>,
    }

    #[derive(Deserialize)]
    pub struct Pages {
        #[serde(rename = "purgeHistory")]
        pub purge_history: Option<PurgeHistory>,
    }
    #[derive(Deserialize)]
    pub struct PurgeHistory {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for PageHistoryPurge {
        type Variables = Variables;
        type ResponseData = ResponseData;
        fn build_query(
            variables: Self::Variables,
        ) -> ::graphql_client::QueryBody<Self::Variables> {
            ::graphql_client::QueryBody {
                variables,
                query: QUERY,
                operation_name: OPERATION_NAME,
            }
        }
    }
}

pub fn page_history_purge(
    client: &Client,
    url: &str,
    older_than: String,
) -> Result<(), PageError> {
    let variables = page_history_purge::Variables { older_than };
    let response = post_graphql::<page_history_purge::PageHistoryPurge, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(PageError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }

    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(pages) = data.pages {
            if let Some(purge_history) = pages.purge_history {
                if let Some(response_result) = purge_history.response_result {
                    if response_result.succeeded {
                        return Ok(());
                    } else {
                        return Err(classify_response_status_error(
                            response_result,
                        ));
                    }
                }
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub(crate) mod page_get_updated_at {
    use super::*;

    pub struct PageGetUpdatedAt;

    pub const OPERATION_NAME: &str = "PageGetUpdatedAt";
    pub const QUERY : & str = "query PageGetUpdatedAt($id: Int!) {\n  pages {\n    single (id: $id) {\n      updatedAt\n    }\n  }\n}\n" ;

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
        pub single: Option<Single>,
    }

    #[derive(Deserialize)]
    pub struct Single {
        #[serde(rename = "updatedAt")]
        pub updated_at: Date,
    }

    impl graphql_client::GraphQLQuery for PageGetUpdatedAt {
        type Variables = Variables;
        type ResponseData = ResponseData;
        fn build_query(
            variables: Self::Variables,
        ) -> ::graphql_client::QueryBody<Self::Variables> {
            ::graphql_client::QueryBody {
                variables,
                query: QUERY,
                operation_name: OPERATION_NAME,
            }
        }
    }
}

pub(crate) fn page_get_updated_at(
    client: &Client,
    url: &str,
    id: i64,
) -> Result<Date, PageError> {
    let variables = page_get_updated_at::Variables { id };
    let response = post_graphql::<page_get_updated_at::PageGetUpdatedAt, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(PageError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }

    let response_body = response.unwrap();

    if let Some(data) = response_body.data {
        if let Some(pages) = data.pages {
            if let Some(single) = pages.single {
                return Ok(single.updated_at);
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub(crate) mod page_get_minimal {
    use super::*;

    pub struct PageGetMinimal;

    pub const OPERATION_NAME: &str = "PageGetMinimal";
    pub const QUERY : & str = "query PageGetMinimal($id: Int!) {\n  pages {\n    single (id: $id) {\n      id\n      path\n      content\n      createdAt\n      updatedAt\n      editor\n      locale\n    }\n  }\n}\n" ;
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
        pub(crate) single: Option<PageMinimal>,
    }

    impl graphql_client::GraphQLQuery for PageGetMinimal {
        type Variables = Variables;
        type ResponseData = ResponseData;
        fn build_query(
            variables: Self::Variables,
        ) -> ::graphql_client::QueryBody<Self::Variables> {
            ::graphql_client::QueryBody {
                variables,
                query: QUERY,
                operation_name: OPERATION_NAME,
            }
        }
    }
}

pub(crate) fn page_get_minimal(
    client: &Client,
    url: &str,
    id: i64,
) -> Result<PageMinimal, PageError> {
    let variables = page_get_minimal::Variables { id };
    let response = post_graphql::<page_get_minimal::PageGetMinimal, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(PageError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }

    let response_body = response.unwrap();

    if let Some(data) = response_body.data {
        if let Some(pages) = data.pages {
            if let Some(single) = pages.single {
                return Ok(single);
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}
