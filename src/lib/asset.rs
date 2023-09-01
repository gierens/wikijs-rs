use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::{classify_response_error, Date, Int, UnknownError};

#[derive(Error, Debug, PartialEq)]
pub enum AssetError {
    #[error("An unexpected error occurred during asset operation.")]
    AssetGenericError,
    #[error("An asset folder with the same name already exists.")]
    AssetFolderExists,
    #[error("You are not authorized to delete this asset.")]
    AssetDeleteForbidden,
    #[error("This asset does not exist or is invalid.")]
    AssetInvalid,
    #[error(
        "An asset with the same filename in the same folder already exists."
    )]
    AssetRenameCollision,
    #[error("You are not authorized to rename this asset.")]
    AssetRenameForbidden,
    #[error("The new asset filename is invalid.")]
    AssetRenameInvalid,
    #[error("The file extension cannot be changed on an existing asset.")]
    AssetRenameInvalidExt,
    #[error(
        "You are not authorized to rename this asset to the requested name."
    )]
    AssetRenameTargetForbidden,
    #[error("Unknown response error code: {code}: {message}")]
    UnknownErrorCode { code: i64, message: String },
    #[error("Unknown response error: {message}")]
    UnknownErrorMessage { message: String },
    #[error("Unknown response error.")]
    UnknownError,
}

impl From<i64> for AssetError {
    fn from(code: i64) -> Self {
        match code {
            2001 => AssetError::AssetGenericError,
            2002 => AssetError::AssetFolderExists,
            2003 => AssetError::AssetDeleteForbidden,
            2004 => AssetError::AssetInvalid,
            2005 => AssetError::AssetRenameCollision,
            2006 => AssetError::AssetRenameForbidden,
            2007 => AssetError::AssetRenameInvalid,
            2008 => AssetError::AssetRenameInvalidExt,
            2009 => AssetError::AssetRenameTargetForbidden,
            _ => AssetError::UnknownErrorCode {
                code,
                message: "Unknown error".to_string(),
            },
        }
    }
}

impl UnknownError for AssetError {
    fn unknown_error_code(code: i64, message: String) -> Self {
        AssetError::UnknownErrorCode { code, message }
    }
    fn unknown_error_message(message: String) -> Self {
        AssetError::UnknownErrorMessage { message }
    }
    fn unknown_error() -> Self {
        AssetError::UnknownError
    }
}

#[derive(Deserialize, Debug)]
pub struct AssetListItem {
    pub id: Int,
    pub filename: String,
    pub ext: String,
    pub kind: AssetKind,
    pub mime: String,
    #[serde(rename = "fileSize")]
    pub file_size: Int,
    pub metadata: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Date,
    #[serde(rename = "updatedAt")]
    pub updated_at: Date,
    pub folder: Option<AssetFolder>,
    pub author: Option<Int>,
}

#[derive(Deserialize, Debug)]
pub struct AssetFolder {
    pub id: Int,
    pub slug: String,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AssetKind {
    IMAGE,
    BINARY,
    ALL,
}

impl ToString for AssetKind {
    fn to_string(&self) -> String {
        match self {
            AssetKind::IMAGE => "IMAGE".to_string(),
            AssetKind::BINARY => "BINARY".to_string(),
            AssetKind::ALL => "ALL".to_string(),
        }
    }
}

pub(crate) mod asset_list {
    use super::*;

    pub struct AssetList;

    pub const OPERATION_NAME: &str = "AssetList";
    pub const QUERY : & str = "query AssetList($folderId: Int!, $kind: AssetKind!) {\n  assets {\n    list (folderId: $folderId, kind: $kind) {\n      id\n      filename\n      ext\n      kind\n      mime\n      fileSize\n      metadata\n      createdAt\n      updatedAt\n      folder {\n        id\n        slug\n        name\n      }\n      author {\n        id\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "folderId")]
        pub folder_id: Int,
        pub kind: AssetKind,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub assets: Option<Assets>,
    }

    #[derive(Deserialize)]
    pub struct Assets {
        pub list: Option<Vec<Option<AssetListItem>>>,
    }

    impl graphql_client::GraphQLQuery for AssetList {
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

pub fn asset_list(
    client: &Client,
    url: &str,
    folder_id: Int,
    kind: AssetKind,
) -> Result<Vec<AssetListItem>, AssetError> {
    let variables = asset_list::Variables { folder_id, kind };
    let response =
        post_graphql::<asset_list::AssetList, _>(client, url, variables);
    if response.is_err() {
        return Err(AssetError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(assets) = data.assets {
            if let Some(list) = assets.list {
                return Ok(list
                    .into_iter()
                    .flatten()
                    .collect::<Vec<AssetListItem>>());
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}
