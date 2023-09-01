// use graphql_client::reqwest::post_graphql_blocking as post_graphql;
// use reqwest::blocking::Client;
// use serde::
use serde::Deserialize;
use thiserror::Error;

use crate::common::{Date, Int, UnknownError};
use crate::user::User;

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
    #[error("An asset with the same filename in the same folder already exists.")]
    AssetRenameCollision,
    #[error("You are not authorized to rename this asset.")]
    AssetRenameForbidden,
    #[error("The new asset filename is invalid.")]
    AssetRenameInvalid,
    #[error("The file extension cannot be changed on an existing asset.")]
    AssetRenameInvalidExt,
    #[error("You are not authorized to rename this asset to the requested name.")]
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
    pub author: Option<User>,
}

#[derive(Deserialize, Debug)]
pub struct AssetFolder {
    pub id: Int,
    pub slug: String,
    pub name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub enum AssetKind {
    IMAGE,
    BINARY,
    ALL,
}

