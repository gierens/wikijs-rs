use graphql_client::reqwest::post_graphql_blocking as post_graphql;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::{
    classify_response_error, classify_response_status_error, Date, Int,
    KnownErrorCodes, ResponseStatus, UnknownError,
};

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

impl KnownErrorCodes for AssetError {
    fn known_error_codes() -> Vec<i64> {
        vec![2001, 2002, 2003, 2004, 2005, 2006, 2007, 2008, 2009]
    }

    fn is_known_error_code(code: i64) -> bool {
        matches!(code, 2001..=2009)
    }
}

#[derive(Deserialize, Debug)]
pub struct AssetItem {
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
        pub list: Option<Vec<Option<AssetItem>>>,
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

// TODO an asset_get function might be interesting as well
pub fn asset_list(
    client: &Client,
    url: &str,
    folder_id: Int,
    kind: AssetKind,
) -> Result<Vec<AssetItem>, AssetError> {
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
                    .collect::<Vec<AssetItem>>());
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub mod asset_folder_list {
    use super::*;

    pub struct AssetFolderList;

    pub const OPERATION_NAME: &str = "AssetFolderList";
    pub const QUERY : & str = "query AssetFolderList($parentFolderId: Int!) {\n  assets {\n    folders (parentFolderId: $parentFolderId) {\n      id\n      slug\n      name\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "parentFolderId")]
        pub parent_folder_id: Int,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub assets: Option<Assets>,
    }

    #[derive(Deserialize)]
    pub struct Assets {
        pub folders: Option<Vec<Option<AssetFolder>>>,
    }

    impl graphql_client::GraphQLQuery for AssetFolderList {
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

pub fn asset_folder_list(
    client: &Client,
    url: &str,
    parent_folder_id: Int,
) -> Result<Vec<AssetFolder>, AssetError> {
    let variables = asset_folder_list::Variables { parent_folder_id };
    let response = post_graphql::<asset_folder_list::AssetFolderList, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(AssetError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(assets) = data.assets {
            if let Some(folders) = assets.folders {
                return Ok(folders
                    .into_iter()
                    .flatten()
                    .collect::<Vec<AssetFolder>>());
            }
        }
    }
    Err(classify_response_error(response_body.errors))
}

pub mod asset_folder_create {

    use super::*;

    pub struct AssetFolderCreate;

    pub const OPERATION_NAME: &str = "AssetFolderCreate";
    pub const QUERY : & str = "mutation AssetFolderCreate(\n  $parentFolderId: Int!\n  $slug: String!\n  $name: String\n) {\n  assets {\n    createFolder(\n      parentFolderId: $parentFolderId\n      slug: $slug\n      name: $name\n    ) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "parentFolderId")]
        pub parent_folder_id: Int,
        pub slug: String,
        pub name: Option<String>,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub assets: Option<Assets>,
    }

    #[derive(Deserialize)]
    pub struct Assets {
        #[serde(rename = "createFolder")]
        pub create_folder: Option<CreateFolder>,
    }

    #[derive(Deserialize)]
    pub struct CreateFolder {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for AssetFolderCreate {
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

pub fn asset_folder_create(
    client: &Client,
    url: &str,
    parent_folder_id: Int,
    slug: String,
    name: Option<String>,
) -> Result<(), AssetError> {
    let variables = asset_folder_create::Variables {
        parent_folder_id,
        slug,
        name,
    };
    let response = post_graphql::<asset_folder_create::AssetFolderCreate, _>(
        client, url, variables,
    );
    if response.is_err() {
        return Err(AssetError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(assets) = data.assets {
            if let Some(create_folder) = assets.create_folder {
                if let Some(response_result) = create_folder.response_result {
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

pub mod asset_rename {
    use super::*;

    pub struct AssetRename;

    pub const OPERATION_NAME: &str = "AssetRename";
    pub const QUERY : & str = "mutation AssetRename($id: Int!, $filename: String!) {\n  assets {\n    renameAsset(id: $id, filename: $filename) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub id: Int,
        pub filename: String,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub assets: Option<Assets>,
    }

    #[derive(Deserialize)]
    pub struct Assets {
        #[serde(rename = "renameAsset")]
        pub rename_asset: Option<RenameAsset>,
    }
    #[derive(Deserialize)]
    pub struct RenameAsset {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for AssetRename {
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

pub fn asset_rename(
    client: &Client,
    url: &str,
    id: Int,
    filename: String,
) -> Result<(), AssetError> {
    let variables = asset_rename::Variables { id, filename };
    let response =
        post_graphql::<asset_rename::AssetRename, _>(client, url, variables);
    if response.is_err() {
        return Err(AssetError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(assets) = data.assets {
            if let Some(rename_asset) = assets.rename_asset {
                if let Some(response_result) = rename_asset.response_result {
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

pub mod asset_delete {
    use super::*;

    pub struct AssetDelete;

    pub const OPERATION_NAME: &str = "AssetDelete";
    pub const QUERY : & str = "mutation AssetDelete($id: Int!) {\n  assets {\n    deleteAsset(id: $id) {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables {
        pub id: Int,
    }

    impl Variables {}

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub assets: Option<Assets>,
    }

    #[derive(Deserialize)]
    pub struct Assets {
        #[serde(rename = "deleteAsset")]
        pub delete_asset: Option<DeleteAsset>,
    }

    #[derive(Deserialize)]
    pub struct DeleteAsset {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for AssetDelete {
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

pub fn asset_delete(
    client: &Client,
    url: &str,
    id: Int,
) -> Result<(), AssetError> {
    let variables = asset_delete::Variables { id };
    let response =
        post_graphql::<asset_delete::AssetDelete, _>(client, url, variables);
    if response.is_err() {
        return Err(AssetError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(assets) = data.assets {
            if let Some(delete_asset) = assets.delete_asset {
                if let Some(response_result) = delete_asset.response_result {
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

pub mod asset_temp_upload_flush {
    use super::*;

    pub struct AssetTempUploadFlush;

    pub const OPERATION_NAME: &str = "AssetTempUploadFlush";
    pub const QUERY : & str = "mutation AssetTempUploadFlush {\n  assets {\n    flushTempUploads {\n      responseResult {\n        succeeded\n        errorCode\n        slug\n        message\n      }\n    }\n  }\n}\n" ;

    #[derive(Serialize)]
    pub struct Variables;

    #[derive(Deserialize)]
    pub struct ResponseData {
        pub assets: Option<Assets>,
    }

    #[derive(Deserialize)]
    pub struct Assets {
        #[serde(rename = "flushTempUploads")]
        pub flush_temp_uploads: Option<FlushTempUploads>,
    }

    #[derive(Deserialize)]
    pub struct FlushTempUploads {
        #[serde(rename = "responseResult")]
        pub response_result: Option<ResponseStatus>,
    }

    impl graphql_client::GraphQLQuery for AssetTempUploadFlush {
        type Variables = asset_temp_upload_flush::Variables;
        type ResponseData = asset_temp_upload_flush::ResponseData;
        fn build_query(
            variables: Self::Variables,
        ) -> ::graphql_client::QueryBody<Self::Variables> {
            graphql_client::QueryBody {
                variables,
                query: asset_temp_upload_flush::QUERY,
                operation_name: asset_temp_upload_flush::OPERATION_NAME,
            }
        }
    }
}

pub fn asset_temp_upload_flush(
    client: &Client,
    url: &str,
) -> Result<(), AssetError> {
    let variables = asset_temp_upload_flush::Variables;
    let response = post_graphql::<
        asset_temp_upload_flush::AssetTempUploadFlush,
        _,
    >(client, url, variables);
    if response.is_err() {
        return Err(AssetError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if let Some(data) = response_body.data {
        if let Some(assets) = data.assets {
            if let Some(flush_temp_uploads) = assets.flush_temp_uploads {
                if let Some(response_result) =
                    flush_temp_uploads.response_result
                {
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

pub fn asset_download(
    client: &Client,
    url: &str,
    path: String,
) -> Result<Vec<u8>, AssetError> {
    let response = client.get(format!("{}/{}", url, path).as_str()).send();
    if response.is_err() {
        return Err(AssetError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if response_body.status().is_success() {
        return Ok(response_body.bytes().unwrap().to_vec());
    }
    Err(AssetError::UnknownError)
}

pub fn asset_upload(
    client: &Client,
    url: &str,
    folder: Int,
    name: String,
    data: Vec<u8>,
) -> Result<(), AssetError> {
    // NOTE: we can also set the mime type like this, but apparently it
    //       also works without it.
    // let part = match reqwest::blocking::multipart::Part::bytes(data)
    //     .file_name(name)
    //     .mime_str("image/jpeg") {
    //         Ok(part) => part,
    //         Err(_) => return Err(AssetError::UnknownError),
    //     };
    let part = reqwest::blocking::multipart::Part::bytes(data).file_name(name);
    let form = reqwest::blocking::multipart::Form::new()
        .text("mediaUpload", format!("{{\"folderId\":{}}}", folder))
        .part("mediaUpload", part);
    let response = client
        .post(format!("{}/u", url).as_str())
        .multipart(form)
        .send();
    if response.is_err() {
        return Err(AssetError::UnknownErrorMessage {
            message: response.err().unwrap().to_string(),
        });
    }
    let response_body = response.unwrap();
    if response_body.status().is_success() {
        return Ok(());
    }
    Err(AssetError::UnknownError)
}
