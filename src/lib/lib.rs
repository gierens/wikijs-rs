#![doc(
    html_logo_url = "https://raw.githubusercontent.com/gierens/wikijs-rs/main/logo/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/gierens/wikijs-rs/main/logo/favicon.ico"
)]
//! API bindings, CLI client and FUSE filesystem for Wiki.js written in Rust.
//!
//! This documents the library only, for more information on using the
//! CLI and FUSE refer to their `-h/--help` commands.
//!
//! # Structure
//! The central struct is [`Api`](struct.Api.html) which is used to directly
//! access all functionality of the Wiki.js APIs. This very flat structure
//! should allow you easy discovery and autocompletion.
//!
//! The library has submodules for each of the Wiki.js' API endpoints. They
//! contain the internal implementation of the library functions as well as
//! all the structs and enums used to interact with the API.
//!
//! # Example
//! The following example shows you to login via an API key and retrieve a
//! [`page::Page`](page/struct.Page.html) struct:
//! ```no_run
//! use wikijs::{Api, Credentials};
//!
//! let api = Api::new(
//!     "http://localhost:3000".to_string(),
//!     Credentials::Key("my-api-key".to_string()),
//! ).unwrap();
//! // this returns a page::Page
//! let page = api.page_get(1).unwrap();
//! println!("{:?}", page);
//! ```
//!
//! # Error handling
//! All API functions return a `Result` with custom module specific error
//! types derived from Wiki.js' error codes. Note we are ignoring a potential
//! [`page::PageError`](page/enum.PageError.html) in the example above!
//!
//! # Testing
//! The integration tests require a clean Wiki.js instance running on localhost
//! with predefined admin login credentials. See the testing section of the
//! [README](https://github.com/gierens/wikijs-rs#testing) on Github or your
//! clone of the project for more details.

use reqwest::blocking::Client;
use reqwest::header::{HeaderValue, AUTHORIZATION};

/// Structs, enums, errors and internal API call implementations to interact
/// with the analytics settings.
pub mod analytics;
/// Structs, enums, errors and internal API call implementations to interact
/// with the assets and asset folders.
pub mod asset;
/// Structs, enums, errors and internal API call implementations to call
/// authentication functions, create API keys and so on.
pub mod authentication;
/// Structs, enums, errors and internal API call implementations to interact
/// with comments.
pub mod comment;
/// Common functions and traits used by multiple modules.
pub mod common;
/// Structs, enums, errors and internal API call implementations to list
/// contributors.
pub mod contribute;
/// Structs, enums, errors and internal API call implementations to interact
/// with user groups.
pub mod group;
/// Structs, enums, errors and internal API call implementations to interact
/// with localization settings.
pub mod localization;
/// Structs, enums, errors and internal API call implementations to interact
/// with logging settings.
pub mod logging;
/// Structs, enums, errors and internal API call implementations to interact
/// with mail settings.
pub mod mail;
/// Structs, enums, errors and internal API call implementations to interact
/// with navigation settings and modify the navigation tree.
pub mod navigation;
/// Structs, enums, errors and internal API call implementations to interact
/// with pages and their directory structure.
pub mod page;
/// Structs, enums, errors and internal API call implementations to interact
/// with rendering settings.
pub mod rendering;
/// Structs, enums, errors and internal API call implementations to interact
/// with search engine settings.
pub mod search;
/// Structs, enums, errors and internal API call implementations to interact
/// with site settings.
pub mod site;
/// Structs, enums, errors and internal API call implementations to interact
/// with storage settings.
pub mod storage;
/// Structs, enums, errors and internal API call implementations to interact
/// with system settings.
pub mod system;
/// Structs, enums, errors and internal API call implementations to interact
/// with theming settings.
pub mod theming;
/// Structs, enums, errors and internal API call implementations to interact
/// with users.
pub mod user;

/// Credentials to authenticate against the Wiki.js API.
#[derive(Debug)]
pub enum Credentials {
    /// API key
    Key(String),
    /// Username, password and authentication strategy ("local" for example)
    UsernamePassword(String, String, String),
}

/// Central struct to access all Wiki.js API endpoints.
#[derive(Debug)]
pub struct Api {
    pub(crate) url: String,
    pub(crate) client: Client,
}

/// The main implementation of the API struct.
impl Api {
    /// Create a new API struct.
    ///
    /// # Arguments
    /// * `url` - The base URL of the Wiki.js instance.
    /// * `credentials` - The credentials to authenticate against the API.
    ///
    /// # Returns
    /// A new API struct.
    pub fn new(
        url: String,
        credentials: Credentials,
    ) -> Result<Self, user::UserError> {
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
                )?;
                auth_response.jwt.unwrap()
            }
        };
        Ok(Self {
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
        })
    }

    // asset functions

    /// List all assets in a folder.
    ///
    /// # Arguments
    /// * `folder_id` - The id of the folder to list assets from.
    /// * `kind` - The kind of assets to list.
    pub fn asset_list(
        &self,
        folder_id: i64,
        kind: asset::AssetKind,
    ) -> Result<Vec<asset::AssetItem>, asset::AssetError> {
        asset::asset_list(
            &self.client,
            &format!("{}/graphql", self.url),
            folder_id,
            kind,
        )
    }

    /// List all asset folders.
    ///
    /// # Arguments
    /// * `parent_id` - The id of the parent folder to list asset folders from.
    ///   Use 0 to list all root folders.
    pub fn asset_folder_list(
        &self,
        parent_id: i64,
    ) -> Result<Vec<asset::AssetFolder>, asset::AssetError> {
        asset::asset_folder_list(
            &self.client,
            &format!("{}/graphql", self.url),
            parent_id,
        )
    }

    /// Create a new asset folder.
    ///
    /// # Arguments
    /// * `parent_folder_id` - The id of the parent folder to create the new
    ///   folder in. Use 0 to create a root folder.
    /// * `slug` - The slug of the new folder.
    /// * `name` - The name of the new folder.
    pub fn asset_folder_create(
        &self,
        parent_folder_id: i64,
        slug: String,
        name: Option<String>,
    ) -> Result<(), asset::AssetError> {
        asset::asset_folder_create(
            &self.client,
            &format!("{}/graphql", self.url),
            parent_folder_id,
            slug,
            name,
        )
    }

    /// Rename an asset.
    ///
    /// # Arguments
    /// * `id` - The id of the asset to rename.
    /// * `filename` - The new name of the asset.
    pub fn asset_rename(
        &self,
        id: i64,
        filename: String,
    ) -> Result<(), asset::AssetError> {
        asset::asset_rename(
            &self.client,
            &format!("{}/graphql", self.url),
            id,
            filename,
        )
    }

    /// Delete an asset.
    ///
    /// # Arguments
    /// * `id` - The id of the asset to delete.
    pub fn asset_delete(&self, id: i64) -> Result<(), asset::AssetError> {
        asset::asset_delete(&self.client, &format!("{}/graphql", self.url), id)
    }

    /// Flush the temporary upload folder.
    pub fn asset_temp_upload_flush(&self) -> Result<(), asset::AssetError> {
        asset::asset_temp_upload_flush(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    /// Download an asset.
    ///
    /// # Arguments
    /// * `path` - The path of the asset to download.
    ///
    /// # Returns
    /// A Result containing either the asset's bytes or an asset error.
    pub fn asset_download(
        &self,
        path: String,
    ) -> Result<Vec<u8>, asset::AssetError> {
        asset::asset_download(&self.client, self.url.as_str(), path)
    }

    /// Upload an asset.
    ///
    /// # Arguments
    /// * `folder` - The id of the folder to upload the asset to.
    /// * `name` - The name of the asset.
    /// * `data` - The bytes of the asset.
    pub fn asset_upload(
        &self,
        folder: i64,
        name: String,
        data: Vec<u8>,
    ) -> Result<(), asset::AssetError> {
        asset::asset_upload(&self.client, self.url.as_str(), folder, name, data)
    }

    // page functions

    /// Get a page by its id.
    ///
    /// # Arguments
    /// * `id` - The id of the page to get.
    ///
    /// # Returns
    /// A Result containing either the page or a page error.
    ///
    /// # Example
    /// ```no_run
    /// use wikijs::{Api, Credentials};
    ///
    /// let api = Api::new(
    ///     "http://localhost:3000".to_string(),
    ///     Credentials::Key("my-api-key".to_string()),
    /// ).unwrap();
    /// println!("{:?}", api.page_get(1).unwrap());
    /// ```
    pub fn page_get(&self, id: i64) -> Result<page::Page, page::PageError> {
        page::page_get(&self.client, &format!("{}/graphql", self.url), id)
    }

    /// Get datetime of last update of a page.
    ///
    /// # Arguments
    /// * `id` - The id of the page to get the last update datetime of.
    ///
    /// # Returns
    /// A Result containing either the datetime string or a page error.
    #[allow(unused)]
    pub fn page_get_updated_at(
        &self,
        id: i64,
    ) -> Result<String, page::PageError> {
        page::page_get_updated_at(
            &self.client,
            &format!("{}/graphql", self.url),
            id,
        )
    }

    /// Get a page's minimal information.
    ///
    /// # Arguments
    /// * `id` - The id of the page to get the minimal information of.
    ///
    /// # Returns
    /// A Result containing either the minimal page information or a page error.
    #[allow(unused)]
    pub fn page_get_minimal(
        &self,
        id: i64,
    ) -> Result<page::PageMinimal, page::PageError> {
        page::page_get_minimal(
            &self.client,
            &format!("{}/graphql", self.url),
            id,
        )
    }

    /// Get a page by its path.
    ///
    /// # Arguments
    /// * `path` - The path of the page to get.
    /// * `locale` - The locale of the page to get.
    pub fn page_get_by_path(
        &self,
        path: String,
        locale: String,
    ) -> Result<page::Page, page::PageError> {
        page::page_get_by_path(
            &self.client,
            &format!("{}/graphql", self.url),
            path,
            locale,
        )
    }

    /// List all page tags.
    pub fn page_tag_list(&self) -> Result<Vec<page::PageTag>, page::PageError> {
        page::page_tag_list(&self.client, &format!("{}/graphql", self.url))
    }

    /// List all pages.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of pages to return.
    /// * `order_by` - The field to order the pages by.
    /// * `order_by_direction` - The direction to order the pages by.
    /// * `tags` - A list of tags to filter the pages by.
    /// * `locale` - The locale of the pages to list.
    /// * `creator_id` - The id of the creator of the pages to list.
    /// * `author_id` - The id of the author of the pages to list.
    #[allow(clippy::too_many_arguments)]
    pub fn page_list(
        &self,
        limit: Option<i64>,
        order_by: Option<page::PageOrderBy>,
        order_by_direction: Option<page::PageOrderByDirection>,
        tags: Option<Vec<String>>,
        locale: Option<String>,
        creator_id: Option<i64>,
        author_id: Option<i64>,
    ) -> Result<Vec<page::PageListItem>, page::PageError> {
        page::page_list(
            &self.client,
            &format!("{}/graphql", self.url),
            limit,
            order_by,
            order_by_direction,
            tags,
            locale,
            creator_id,
            author_id,
        )
    }

    /// Get a page's content by its id.
    ///
    /// # Arguments
    /// * `parent` - The id of the parent tree item. Use 0 for the root.
    /// * `mode` - The mode of what items to include.
    /// * `include_ancestors` - Whether to include the ancestors of the page.
    /// * `locale` - The locale of the page to get.
    pub fn page_tree_get(
        &self,
        parent: i64,
        mode: page::PageTreeMode,
        include_ancestors: bool,
        locale: String,
    ) -> Result<Vec<page::PageTreeItem>, page::PageError> {
        page::page_tree_get(
            &self.client,
            &format!("{}/graphql", self.url),
            parent,
            mode,
            include_ancestors,
            locale,
        )
    }

    /// Delete a page.
    ///
    /// # Arguments
    /// * `id` - The id of the page to delete.
    pub fn page_delete(&self, id: i64) -> Result<(), page::PageError> {
        page::page_delete(&self.client, &format!("{}/graphql", self.url), id)
    }

    /// Render a page.
    ///
    /// # Arguments
    /// * `id` - The id of the page to render.
    pub fn page_render(&self, id: i64) -> Result<(), page::PageError> {
        page::page_render(&self.client, &format!("{}/graphql", self.url), id)
    }

    /// Create a new page.
    ///
    /// # Arguments
    /// * `content` - The content of the page.
    /// * `description` - The description of the page.
    /// * `editor` - The editor of the page.
    /// * `is_published` - Whether the page is published.
    /// * `is_private` - Whether the page is private.
    /// * `locale` - The locale of the page.
    /// * `path` - The path of the page.
    /// * `publish_end_date` - The end date of the page's publication.
    /// * `publish_start_date` - The start date of the page's publication.
    /// * `script_css` - The CSS script of the page.
    /// * `script_js` - The JS script of the page.
    /// * `tags` - The tags of the page.
    /// * `title` - The title of the page.
    #[allow(clippy::too_many_arguments)]
    pub fn page_create(
        &self,
        content: String,
        description: String,
        editor: String,
        is_published: bool,
        is_private: bool,
        locale: String,
        path: String,
        publish_end_date: Option<common::Date>,
        publish_start_date: Option<common::Date>,
        script_css: Option<String>,
        script_js: Option<String>,
        tags: Vec<Option<String>>,
        title: String,
    ) -> Result<(), page::PageError> {
        page::page_create(
            &self.client,
            &format!("{}/graphql", self.url),
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
        )
    }

    /// Update a page.
    ///
    /// # Arguments
    /// * `id` - The id of the page to update.
    /// * `content` - The new content of the page.
    /// * `description` - The new description of the page.
    /// * `editor` - The new editor of the page.
    /// * `is_published` - Whether the page is published.
    /// * `is_private` - Whether the page is private.
    /// * `locale` - The new locale of the page.
    /// * `path` - The new path of the page.
    /// * `publish_end_date` - The new end date of the page's publication.
    /// * `publish_start_date` - The new start date of the page's publication.
    /// * `script_css` - The new CSS script of the page.
    /// * `script_js` - The new JS script of the page.
    /// * `tags` - The new tags of the page.
    /// * `title` - The new title of the page.
    #[allow(clippy::too_many_arguments)]
    pub fn page_update(
        &self,
        id: i64,
        content: Option<String>,
        description: Option<String>,
        editor: Option<String>,
        is_private: Option<bool>,
        is_published: Option<bool>,
        locale: Option<String>,
        path: Option<String>,
        publish_end_date: Option<common::Date>,
        publish_start_date: Option<common::Date>,
        script_css: Option<String>,
        script_js: Option<String>,
        tags: Option<Vec<Option<String>>>,
        title: Option<String>,
    ) -> Result<(), page::PageError> {
        page::page_update(
            &self.client,
            &format!("{}/graphql", self.url),
            id,
            content,
            description,
            editor,
            is_private,
            is_published,
            locale,
            path,
            publish_end_date,
            publish_start_date,
            script_css,
            script_js,
            tags,
            title,
        )
    }

    /// Update a page's content.
    ///
    /// # Arguments
    /// * `id` - The id of the page to update.
    /// * `content` - The new content of the page.
    pub fn page_update_content(
        &self,
        id: i64,
        content: String,
    ) -> Result<(), page::PageError> {
        self.page_update(
            id,
            Some(content),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
    }

    /// Get a page's history.
    ///
    /// # Arguments
    /// * `id` - The id of the page to get the history of.
    /// * `offset_page` - The page offset.
    /// * `offset_size` - The offset size.
    pub fn page_history_get(
        &self,
        id: i64,
        offset_page: Option<i64>,
        offset_size: Option<i64>,
    ) -> Result<page::PageHistoryResult, page::PageError> {
        page::page_history_get(
            &self.client,
            &format!("{}/graphql", self.url),
            id,
            offset_page,
            offset_size,
        )
    }

    /// Get specific version of a page.
    ///
    /// # Arguments
    /// * `id` - The id of the page to get the version of.
    /// * `version` - The version of the page to get.
    pub fn page_version_get(
        &self,
        id: i64,
        version: i64,
    ) -> Result<page::PageVersion, page::PageError> {
        page::page_version_get(
            &self.client,
            &format!("{}/graphql", self.url),
            id,
            version,
        )
    }

    /// Search for pages.
    ///
    /// # Arguments
    /// * `query` - The query to search for.
    /// * `path` - The path to search in.
    /// * `locale` - The locale to search in.
    pub fn page_search(
        &self,
        query: String,
        path: Option<String>,
        locale: Option<String>,
    ) -> Result<page::PageSearchResponse, page::PageError> {
        page::page_search(
            &self.client,
            &format!("{}/graphql", self.url),
            query,
            path,
            locale,
        )
    }

    /// List all page links.
    ///
    /// # Arguments
    /// * `locale` - The locale to list page links for.
    pub fn page_link_list(
        &self,
        locale: String,
    ) -> Result<Vec<page::PageLinkItem>, page::PageError> {
        page::page_link_list(
            &self.client,
            &format!("{}/graphql", self.url),
            locale,
        )
    }

    /// Check for page conflicts.
    ///
    /// # Arguments
    /// * `id` - The id of the page.
    /// * `checkout_date` - The checkout date of the page.
    pub fn page_conflict_check(
        &self,
        id: i64,
        checkout_date: String,
    ) -> Result<bool, page::PageError> {
        page::page_conflict_check(
            &self.client,
            &format!("{}/graphql", self.url),
            id,
            checkout_date,
        )
    }

    /// Get the latest page conflict.
    ///
    /// # Arguments
    /// * `id` - The id of the page.
    pub fn page_conflict_latest(
        &self,
        id: i64,
    ) -> Result<page::PageConflictLatest, page::PageError> {
        page::page_conflict_latest(
            &self.client,
            &format!("{}/graphql", self.url),
            id,
        )
    }

    /// Convert a page to a different editor format.
    ///
    /// # Arguments
    /// * `id` - The id of the page.
    /// * `editor` - The editor to convert the page to.
    pub fn page_convert(
        &self,
        id: i64,
        editor: String,
    ) -> Result<(), page::PageError> {
        page::page_convert(
            &self.client,
            &format!("{}/graphql", self.url),
            id,
            editor,
        )
    }

    /// Move a page.
    ///
    /// # Arguments
    /// * `id` - The id of the page.
    /// * `destination_path` - The destination path of the page.
    /// * `destination_locale` - The destination locale of the page.
    pub fn page_move(
        &self,
        id: i64,
        destination_path: String,
        destination_locale: String,
    ) -> Result<(), page::PageError> {
        page::page_move(
            &self.client,
            &format!("{}/graphql", self.url),
            id,
            destination_path,
            destination_locale,
        )
    }

    /// Delete a page tag.
    ///
    /// # Arguments
    /// * `id` - The id of the page tag.
    pub fn page_tag_delete(&self, id: i64) -> Result<(), page::PageError> {
        page::page_tag_delete(
            &self.client,
            &format!("{}/graphql", self.url),
            id,
        )
    }

    /// Update a page tag.
    ///
    /// # Arguments
    /// * `id` - The id of the page tag.
    /// * `tag` - The new name of the page tag.
    /// * `title` - The new title of the page tag.
    pub fn page_tag_update(
        &self,
        id: i64,
        tag: String,
        title: String,
    ) -> Result<(), page::PageError> {
        page::page_tag_update(
            &self.client,
            &format!("{}/graphql", self.url),
            id,
            tag,
            title,
        )
    }

    /// Flush the page cache.
    pub fn page_cache_flush(&self) -> Result<(), page::PageError> {
        page::page_cache_flush(&self.client, &format!("{}/graphql", self.url))
    }

    /// Migrate pages from one locale to another.
    ///
    /// # Arguments
    /// * `source_locale` - The source locale to migrate from.
    /// * `target_locale` - The target locale to migrate to.
    pub fn page_migrate_to_locale(
        &self,
        source_locale: String,
        target_locale: String,
    ) -> Result<(), page::PageError> {
        page::page_migrate_to_locale(
            &self.client,
            &format!("{}/graphql", self.url),
            source_locale,
            target_locale,
        )
    }

    /// Rebuild the page tree.
    pub fn page_tree_rebuild(&self) -> Result<(), page::PageError> {
        page::page_tree_rebuild(&self.client, &format!("{}/graphql", self.url))
    }

    /// Restore a page version.
    ///
    /// # Arguments
    /// * `page_id` - The id of the page.
    /// * `version_id` - The id of the version to restore.
    pub fn page_restore(
        &self,
        page_id: i64,
        version_id: i64,
    ) -> Result<(), page::PageError> {
        page::page_restore(
            &self.client,
            &format!("{}/graphql", self.url),
            page_id,
            version_id,
        )
    }

    /// Purge the page history.
    ///
    /// # Arguments
    /// * `older_than` - The date to purge history entries older than.
    pub fn page_history_purge(
        &self,
        older_than: String,
    ) -> Result<(), page::PageError> {
        page::page_history_purge(
            &self.client,
            &format!("{}/graphql", self.url),
            older_than,
        )
    }

    // authentication functions

    /// Login via username and password.
    ///
    /// # Arguments
    /// * `username` - The username to login with.
    /// * `password` - The password to login with.
    /// * `strategy` - The authentication strategy to use, for example "local".
    ///   Use [`authentication_strategy_list`](#method.authentication_strategy_list)
    pub fn login(
        &self,
        username: String,
        password: String,
        strategy: String,
    ) -> Result<authentication::AuthenticationLoginResponse, user::UserError>
    {
        authentication::login(
            &self.client,
            &format!("{}/graphql", self.url),
            username,
            password,
            strategy,
        )
    }

    /// List API keys.
    pub fn api_key_list(
        &self,
    ) -> Result<Vec<authentication::ApiKey>, user::UserError> {
        authentication::api_key_list(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    /// Get the current API state.
    pub fn api_state_get(&self) -> Result<bool, user::UserError> {
        authentication::api_state_get(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    /// List all authentication strategies.
    pub fn authentication_strategy_list(
        &self,
    ) -> Result<Vec<authentication::AuthenticationStrategy>, user::UserError>
    {
        authentication::authentication_strategy_list(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    /// List authentication strategies.
    ///
    /// # Arguments
    /// * `enabled_only` - Only list enabled authentication strategies.
    pub fn authentication_active_strategy_list(
        &self,
        enabled_only: Option<bool>,
    ) -> Result<
        Vec<authentication::AuthenticationActiveStrategy>,
        user::UserError,
    > {
        authentication::authentication_active_strategy_list(
            &self.client,
            &format!("{}/graphql", self.url),
            enabled_only,
        )
    }

    /// Create a new API key.
    ///
    /// # Arguments
    /// * `name` - The name of the API key.
    /// * `expiration` - The expiration date of the API key.
    /// * `full_access` - Whether the API key has full access.
    /// * `group` - The group of the API key.
    pub fn api_key_create(
        &self,
        name: String,
        expiration: Option<String>,
        full_access: bool,
        group: Option<i64>,
    ) -> Result<String, user::UserError> {
        authentication::api_key_create(
            &self.client,
            &format!("{}/graphql", self.url),
            name,
            expiration,
            full_access,
            group,
        )
    }

    /// Login via TFA.
    ///
    /// # Arguments
    /// * `continuation_token` - The continuation token of the TFA login.
    /// * `security_code` - The security code of the TFA login.
    /// * `setup` - Whether this is a setup login.
    pub fn login_tfa(
        &self,
        continuation_token: String,
        security_code: String,
        setup: Option<bool>,
    ) -> Result<authentication::AuthenticationLoginResponse, user::UserError>
    {
        authentication::login_tfa(
            &self.client,
            &format!("{}/graphql", self.url),
            continuation_token,
            security_code,
            setup,
        )
    }

    /// Change the password of a user.
    ///
    /// # Arguments
    /// * `continuation_token` - The continuation token of the password change.
    /// * `new_password` - The new password of the user.
    pub fn login_password_change(
        &self,
        continuation_token: String,
        new_password: String,
    ) -> Result<authentication::AuthenticationLoginResponse, user::UserError>
    {
        authentication::login_password_change(
            &self.client,
            &format!("{}/graphql", self.url),
            continuation_token,
            new_password,
        )
    }

    /// Issue a password forgotten mail.
    ///
    /// # Arguments
    /// * `email` - The email of the user to reset the password for.
    pub fn password_forgot(
        &self,
        email: String,
    ) -> Result<(), user::UserError> {
        authentication::password_forgot(
            &self.client,
            &format!("{}/graphql", self.url),
            email,
        )
    }

    /// Register a new user.
    ///
    /// # Arguments
    /// * `email` - The email of the new user.
    /// * `password` - The password of the new user.
    /// * `name` - The name of the new user.
    pub fn register(
        &self,
        email: String,
        password: String,
        name: String,
    ) -> Result<authentication::AuthenticationRegisterResponse, user::UserError>
    {
        authentication::register(
            &self.client,
            &format!("{}/graphql", self.url),
            email,
            password,
            name,
        )
    }

    /// Revoke an API key.
    ///
    /// # Arguments
    /// * `id` - The id of the API key to revoke.
    pub fn api_key_revoke(&self, id: i64) -> Result<(), user::UserError> {
        authentication::api_key_revoke(
            &self.client,
            &format!("{}/graphql", self.url),
            id,
        )
    }

    /// Set the API state.
    ///
    /// # Arguments
    /// * `enabled` - Whether the API should be enabled.
    pub fn api_state_set(&self, enabled: bool) -> Result<(), user::UserError> {
        authentication::api_state_set(
            &self.client,
            &format!("{}/graphql", self.url),
            enabled,
        )
    }

    /// Update the authentication strategies.
    ///
    /// # Arguments
    /// * `strategies` - The new authentication strategies.
    pub fn authentication_strategy_update(
        &self,
        strategies: Vec<authentication::AuthenticationStrategyInput>,
    ) -> Result<(), user::UserError> {
        authentication::authentication_strategy_update(
            &self.client,
            &format!("{}/graphql", self.url),
            strategies,
        )
    }

    /// Regenerate the authentication certificates.
    pub fn authentication_certificate_regenerate(
        &self,
    ) -> Result<(), user::UserError> {
        authentication::authentication_certificate_regenerate(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    /// Reset the guest user.
    pub fn guest_user_reset(&self) -> Result<(), user::UserError> {
        authentication::guest_user_reset(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    // contribute functions

    /// List all contributors.
    pub fn contributor_list(
        &self,
    ) -> Result<Vec<contribute::Contributor>, contribute::ContributeError> {
        contribute::contributor_list(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    // analytics functions

    /// List all analytics providers.
    pub fn analytics_provider_list(
        &self,
    ) -> Result<Vec<analytics::AnalyticsProvider>, analytics::AnalyticsError>
    {
        analytics::analytics_provider_list(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    /// Update the analytics providers.
    ///
    /// # Arguments
    /// * `providers` - The new analytics providers.
    pub fn analytics_provider_update(
        &self,
        providers: Vec<analytics::AnalyticsProviderInput>,
    ) -> Result<(), analytics::AnalyticsError> {
        analytics::analytics_provider_update(
            &self.client,
            &format!("{}/graphql", self.url),
            providers,
        )
    }

    // comment functions

    /// List all comments of a page
    ///
    /// # Arguments
    /// * `locale` - The locale of the page to list comments for.
    /// * `path` - The path of the page to list comments for.
    pub fn comment_list(
        &self,
        locale: String,
        path: String,
    ) -> Result<Vec<comment::Comment>, comment::CommentError> {
        comment::comment_list(
            &self.client,
            &format!("{}/graphql", self.url),
            locale,
            path,
        )
    }

    /// List all comment providers.
    pub fn comment_provider_list(
        &self,
    ) -> Result<Vec<comment::CommentProvider>, comment::CommentError> {
        comment::comment_provider_list(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    /// Get a specific comment.
    ///
    /// # Arguments
    /// * `id` - The id of the comment to get.
    pub fn comment_get(
        &self,
        id: i64,
    ) -> Result<comment::Comment, comment::CommentError> {
        comment::comment_get(&self.client, &format!("{}/graphql", self.url), id)
    }

    /// Update the comment providers.
    ///
    /// # Arguments
    /// * `providers` - The new comment providers.
    pub fn comment_provider_update(
        &self,
        providers: Vec<comment::CommentProviderInput>,
    ) -> Result<(), comment::CommentError> {
        comment::comment_provider_update(
            &self.client,
            &format!("{}/graphql", self.url),
            providers,
        )
    }

    /// Create a new comment.
    ///
    /// # Arguments
    /// * `page_id` - The id of the page to create the comment for.
    /// * `reply_to` - The id of the comment to reply to.
    /// * `content` - The content of the comment.
    /// * `guest_name` - The name of the guest if the comment is created by a guest.
    /// * `guest_email` - The email of the guest if the comment is created by a guest.
    pub fn comment_create(
        &self,
        page_id: i64,
        reply_to: Option<i64>,
        content: String,
        guest_name: Option<String>,
        guest_email: Option<String>,
    ) -> Result<(), comment::CommentError> {
        comment::comment_create(
            &self.client,
            &format!("{}/graphql", self.url),
            page_id,
            reply_to,
            content,
            guest_name,
            guest_email,
        )
    }

    /// Update a comment.
    ///
    /// # Arguments
    /// * `id` - The id of the comment to update.
    /// * `content` - The new content of the comment.
    pub fn comment_update(
        &self,
        id: i64,
        content: String,
    ) -> Result<(), comment::CommentError> {
        comment::comment_update(
            &self.client,
            &format!("{}/graphql", self.url),
            id,
            content,
        )
    }

    /// Delete a comment.
    ///
    /// # Arguments
    /// * `id` - The id of the comment to delete.
    pub fn comment_delete(&self, id: i64) -> Result<(), comment::CommentError> {
        comment::comment_delete(
            &self.client,
            &format!("{}/graphql", self.url),
            id,
        )
    }

    // user functions

    /// Get a user.
    ///
    /// # Arguments
    /// * `id` - The id of the user to get.
    pub fn user_get(&self, id: i64) -> Result<user::User, user::UserError> {
        user::user_get(&self.client, &format!("{}/graphql", self.url), id)
    }

    /// List users.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply.
    /// * `order_by` - The order by to apply.
    pub fn user_list(
        &self,
        filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<Vec<user::UserMinimal>, user::UserError> {
        user::user_list(
            &self.client,
            &format!("{}/graphql", self.url),
            filter,
            order_by,
        )
    }

    /// Activate a user.
    ///
    /// # Arguments
    /// * `id` - The id of the user to activate.
    pub fn user_activate(&self, id: i64) -> Result<(), user::UserError> {
        user::user_activate(&self.client, &format!("{}/graphql", self.url), id)
    }

    /// Deactivate a user.
    ///
    /// # Arguments
    /// * `id` - The id of the user to deactivate.
    pub fn user_deactivate(&self, id: i64) -> Result<(), user::UserError> {
        user::user_deactivate(
            &self.client,
            &format!("{}/graphql", self.url),
            id,
        )
    }

    /// Delete a user.
    ///
    /// # Arguments
    /// * `id` - The id of the user to delete.
    pub fn user_delete(
        &self,
        id: i64,
        replace_id: i64,
    ) -> Result<(), user::UserError> {
        user::user_delete(
            &self.client,
            &format!("{}/graphql", self.url),
            id,
            replace_id,
        )
    }

    /// Disable TFA for a user.
    ///
    /// # Arguments
    /// * `id` - The id of the user.
    pub fn user_tfa_disable(&self, id: i64) -> Result<(), user::UserError> {
        user::user_tfa_disable(
            &self.client,
            &format!("{}/graphql", self.url),
            id,
        )
    }

    /// Enable TFA for a user.
    ///
    /// # Arguments
    /// * `id` - The id of the user.
    pub fn user_tfa_enable(&self, id: i64) -> Result<(), user::UserError> {
        user::user_tfa_enable(
            &self.client,
            &format!("{}/graphql", self.url),
            id,
        )
    }

    /// Verify a user.
    ///
    /// # Arguments
    /// * `id` - The id of the user.
    pub fn user_verify(&self, id: i64) -> Result<(), user::UserError> {
        user::user_verify(&self.client, &format!("{}/graphql", self.url), id)
    }

    /// Search for users.
    ///
    /// # Arguments
    /// * `query` - The query to search for.
    pub fn user_search(
        &self,
        query: String,
    ) -> Result<Vec<user::UserMinimal>, user::UserError> {
        user::user_search(&self.client, &format!("{}/graphql", self.url), query)
    }

    /// Get the current user's profile.
    pub fn user_profile_get(
        &self,
    ) -> Result<user::UserProfile, user::UserError> {
        user::user_profile_get(&self.client, &format!("{}/graphql", self.url))
    }

    /// List the last logins.
    pub fn user_last_login_list(
        &self,
    ) -> Result<Vec<user::UserLastLogin>, user::UserError> {
        user::user_last_login_list(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    /// Create a new user.
    ///
    /// # Arguments
    /// * `email` - The email address.
    /// * `name` - The username.
    /// * `password_raw` - The raw password.
    /// * `provider_key` - The provider key.
    /// * `groups` - The groups of the user.
    /// * `must_change_password` - Whether the user must change the password.
    /// * `send_welcome_email` - Whether to send a welcome email.
    #[allow(clippy::too_many_arguments)]
    pub fn user_create(
        &self,
        email: String,
        name: String,
        password_raw: Option<String>,
        provider_key: String,
        groups: Vec<Option<i64>>,
        must_change_password: Option<bool>,
        send_welcome_email: Option<bool>,
    ) -> Result<(), user::UserError> {
        user::user_create(
            &self.client,
            &format!("{}/graphql", self.url),
            email,
            name,
            password_raw,
            provider_key,
            groups,
            must_change_password,
            send_welcome_email,
        )
    }

    /// Update a user.
    ///
    /// # Arguments
    /// * `id` - The id of the user to update.
    /// * `email` - The email address.
    /// * `name` - The username.
    /// * `new_password` - The new password.
    /// * `groups` - The groups of the user.
    /// * `location` - The location of the user.
    /// * `job_title` - The job title of the user.
    /// * `timezone` - The timezone of the user.
    /// * `date_format` - The date format of the user.
    /// * `appearance` - The appearance of the user.
    #[allow(clippy::too_many_arguments)]
    pub fn user_update(
        &self,
        id: i64,
        email: Option<String>,
        name: Option<String>,
        new_password: Option<String>,
        groups: Option<Vec<Option<i64>>>,
        location: Option<String>,
        job_title: Option<String>,
        timezone: Option<String>,
        date_format: Option<String>,
        appearance: Option<String>,
    ) -> Result<(), user::UserError> {
        user::user_update(
            &self.client,
            &format!("{}/graphql", self.url),
            id,
            email,
            name,
            new_password,
            groups,
            location,
            job_title,
            timezone,
            date_format,
            appearance,
        )
    }

    /// Update the current user's profile.
    ///
    /// # Arguments
    /// * `name` - The username.
    /// * `location` - The location of the user.
    /// * `job_title` - The job title of the user.
    /// * `timezone` - The timezone of the user.
    /// * `date_format` - The date format of the user.
    /// * `appearance` - The appearance of the user.
    #[allow(clippy::too_many_arguments)]
    pub fn user_profile_update(
        &self,
        name: String,
        location: String,
        job_title: String,
        timezone: String,
        date_format: String,
        appearance: String,
    ) -> Result<Option<String>, user::UserError> {
        user::user_profile_update(
            &self.client,
            &format!("{}/graphql", self.url),
            name,
            location,
            job_title,
            timezone,
            date_format,
            appearance,
        )
    }

    /// Change the password of the current user.
    ///
    /// # Arguments
    /// * `current` - The current password.
    /// * `new` - The new password.
    pub fn user_password_change(
        &self,
        current: String,
        new: String,
    ) -> Result<Option<String>, user::UserError> {
        user::user_password_change(
            &self.client,
            &format!("{}/graphql", self.url),
            current,
            new,
        )
    }

    /// Reset the password of a user.
    ///
    /// # Arguments
    /// * `id` - The id of the user to reset the password for.
    pub fn user_password_reset(&self, id: i64) -> Result<(), user::UserError> {
        user::user_password_reset(
            &self.client,
            &format!("{}/graphql", self.url),
            id,
        )
    }

    // group functions

    /// List groups.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply.
    /// * `order_by` - The order by to apply.
    pub fn group_list(
        &self,
        filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<Vec<group::GroupMinimal>, group::GroupError> {
        group::group_list(
            &self.client,
            &format!("{}/graphql", self.url),
            filter,
            order_by,
        )
    }

    /// Get a group.
    ///
    /// # Arguments
    /// * `id` - The id of the group to get.
    pub fn group_get(
        &self,
        id: i64,
    ) -> Result<group::Group, group::GroupError> {
        group::group_get(&self.client, &format!("{}/graphql", self.url), id)
    }

    /// Create a new group.
    ///
    /// # Arguments
    /// * `name` - The name of the group.
    pub fn group_create(&self, name: String) -> Result<(), group::GroupError> {
        group::group_create(
            &self.client,
            &format!("{}/graphql", self.url),
            name,
        )
    }

    /// Update a group.
    ///
    /// # Arguments
    /// * `id` - The id of the group to update.
    /// * `name` - The new name of the group.
    /// * `redirect_on_login` - The new redirect on login of the group.
    /// * `permissions` - The new permissions of the group.
    /// * `page_rules` - The new page rules of the group.
    pub fn group_update(
        &self,
        id: i64,
        name: String,
        redirect_on_login: String,
        permissions: Vec<String>,
        page_rules: Vec<group::PageRuleInput>,
    ) -> Result<(), group::GroupError> {
        group::group_update(
            &self.client,
            &format!("{}/graphql", self.url),
            id,
            name,
            redirect_on_login,
            permissions,
            page_rules,
        )
    }

    /// Delete a group.
    ///
    /// # Arguments
    /// * `id` - The id of the group to delete.
    pub fn group_delete(&self, id: i64) -> Result<(), group::GroupError> {
        group::group_delete(&self.client, &format!("{}/graphql", self.url), id)
    }

    /// Assign a user to a group.
    ///
    /// # Arguments
    /// * `group_id` - The id of the group to assign the user to.
    /// * `user_id` - The id of the user to assign to the group.
    pub fn group_user_assign(
        &self,
        group_id: i64,
        user_id: i64,
    ) -> Result<(), group::GroupError> {
        group::group_user_assign(
            &self.client,
            &format!("{}/graphql", self.url),
            group_id,
            user_id,
        )
    }

    /// Unassign a user from a group.
    ///
    /// # Arguments
    /// * `group_id` - The id of the group to unassign the user from.
    /// * `user_id` - The id of the user to unassign from the group.
    pub fn group_user_unassign(
        &self,
        group_id: i64,
        user_id: i64,
    ) -> Result<(), group::GroupError> {
        group::group_user_unassign(
            &self.client,
            &format!("{}/graphql", self.url),
            group_id,
            user_id,
        )
    }

    // locale functions

    /// List all locales.
    pub fn locale_list(
        &self,
    ) -> Result<Vec<localization::Locale>, localization::LocaleError> {
        localization::locale_list(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    /// Get a locale configuration.
    pub fn locale_config_get(
        &self,
    ) -> Result<localization::LocaleConfig, localization::LocaleError> {
        localization::locale_config_get(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    /// List all translations.
    ///
    /// # Arguments
    /// * `locale` - The locale to list translations for.
    /// * `namespace` - The namespace to list translations for.
    pub fn translation_list(
        &self,
        locale: String,
        namespace: String,
    ) -> Result<Vec<localization::Translation>, localization::LocaleError> {
        localization::translation_list(
            &self.client,
            &format!("{}/graphql", self.url),
            locale,
            namespace,
        )
    }

    /// Download a locale.
    ///
    /// # Arguments
    /// * `locale` - The locale to download.
    pub fn locale_download(
        &self,
        locale: String,
    ) -> Result<(), localization::LocaleError> {
        localization::locale_download(
            &self.client,
            &format!("{}/graphql", self.url),
            locale,
        )
    }

    /// Upload a locale.
    ///
    /// # Arguments
    /// * `locale` - The locale to upload.
    /// * `auto_update` - Whether to auto update the locale.
    /// * `namespacing` - Whether to namespace the locale.
    /// * `namespaces` - The namespaces to use.
    pub fn locale_update(
        &self,
        locale: String,
        auto_update: bool,
        namespacing: bool,
        namespaces: Vec<String>,
    ) -> Result<(), localization::LocaleError> {
        localization::locale_update(
            &self.client,
            &format!("{}/graphql", self.url),
            locale,
            auto_update,
            namespacing,
            namespaces,
        )
    }

    // logger functions

    /// List all loggers.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply.
    /// * `order_by` - The order by to apply.
    pub fn logger_list(
        &self,
        filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<Vec<logging::Logger>, logging::LoggingError> {
        logging::logger_list(
            &self.client,
            &format!("{}/graphql", self.url),
            filter,
            order_by,
        )
    }

    /// Update loggers.
    ///
    /// # Arguments
    /// * `loggers` - The new loggers.
    pub fn logger_update(
        &self,
        loggers: Vec<logging::LoggerInput>,
    ) -> Result<(), logging::LoggingError> {
        logging::logger_update(
            &self.client,
            &format!("{}/graphql", self.url),
            loggers,
        )
    }

    // mail functions

    /// Get the mail configuration.
    pub fn mail_config_get(&self) -> Result<mail::MailConfig, mail::MailError> {
        mail::mail_config_get(&self.client, &format!("{}/graphql", self.url))
    }

    /// Send a test mail.
    ///
    /// # Arguments
    /// * `recipient_email` - The email address of the recipient.
    pub fn mail_send_test(
        &self,
        recipient_email: String,
    ) -> Result<(), mail::MailError> {
        mail::mail_send_test(
            &self.client,
            &format!("{}/graphql", self.url),
            recipient_email,
        )
    }

    /// Update the mail configuration.
    ///
    /// # Arguments
    /// * `sender_name` - The name of the sender.
    /// * `sender_email` - The email address of the sender.
    /// * `host` - The host of the mail server.
    /// * `port` - The port of the mail server.
    /// * `name` - The name of the mail server.
    /// * `secure` - Whether the connection should be secure.
    /// * `verify_ssl` - Whether the SSL certificate should be verified.
    /// * `user` - The user of the mail server.
    /// * `pass` - The password of the mail server.
    /// * `use_dkim` - Whether DKIM should be used.
    /// * `dkim_domain_name` - The domain name of DKIM.
    /// * `dkim_key_selector` - The key selector of DKIM.
    /// * `dkim_private_key` - The private key of DKIM.
    #[allow(clippy::too_many_arguments)]
    pub fn mail_config_update(
        &self,
        sender_name: String,
        sender_email: String,
        host: String,
        port: i64,
        name: String,
        secure: bool,
        verify_ssl: bool,
        user: String,
        pass: String,
        use_dkim: bool,
        dkim_domain_name: String,
        dkim_key_selector: String,
        dkim_private_key: String,
    ) -> Result<(), mail::MailError> {
        mail::mail_config_update(
            &self.client,
            &format!("{}/graphql", self.url),
            sender_name,
            sender_email,
            host,
            port,
            name,
            secure,
            verify_ssl,
            user,
            pass,
            use_dkim,
            dkim_domain_name,
            dkim_key_selector,
            dkim_private_key,
        )
    }

    // navigation functions

    /// Get the navigation configuration.
    pub fn navigation_config_get(
        &self,
    ) -> Result<navigation::NavigationConfig, navigation::NavigationError> {
        navigation::navigation_config_get(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    /// Get the navigation tree.
    pub fn navigation_tree_get(
        &self,
    ) -> Result<Vec<navigation::NavigationTree>, navigation::NavigationError>
    {
        navigation::navigation_tree_get(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    /// Update the navigation configuration.
    ///
    /// # Arguments
    /// * `mode` - The new navigation mode.
    pub fn navigation_config_update(
        &self,
        mode: navigation::NavigationMode,
    ) -> Result<(), navigation::NavigationError> {
        navigation::navigation_config_update(
            &self.client,
            &format!("{}/graphql", self.url),
            mode,
        )
    }

    /// Update the navigation tree.
    ///
    /// # Arguments
    /// * `tree` - The new navigation tree.
    pub fn navigation_tree_update(
        &self,
        tree: Vec<navigation::NavigationTreeInput>,
    ) -> Result<(), navigation::NavigationError> {
        navigation::navigation_tree_update(
            &self.client,
            &format!("{}/graphql", self.url),
            tree,
        )
    }

    // system functions

    /// List all system flags.
    pub fn system_flag_list(
        &self,
    ) -> Result<Vec<system::SystemFlag>, system::SystemError> {
        system::system_flag_list(&self.client, &format!("{}/graphql", self.url))
    }

    /// Get the system info.
    pub fn system_info_get(
        &self,
    ) -> Result<system::SystemInfo, system::SystemError> {
        system::system_info_get(&self.client, &format!("{}/graphql", self.url))
    }

    /// List all system extensions.
    pub fn system_extension_list(
        &self,
    ) -> Result<Vec<system::SystemExtension>, system::SystemError> {
        system::system_extension_list(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    /// Get the system's export status.
    pub fn system_export_status_get(
        &self,
    ) -> Result<system::SystemExportStatus, system::SystemError> {
        system::system_export_status_get(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    /// Update the system flags.
    ///
    /// # Arguments
    /// * `flags` - The new system flags.
    pub fn system_flags_update(
        &self,
        flags: Vec<system::SystemFlagInput>,
    ) -> Result<(), system::SystemError> {
        system::system_flags_update(
            &self.client,
            &format!("{}/graphql", self.url),
            flags,
        )
    }

    /// Reset the telemetry client id.
    pub fn telemetry_client_id_reset(&self) -> Result<(), system::SystemError> {
        system::telemetry_client_id_reset(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    /// Set the telemetry enabled flag.
    ///
    /// # Arguments
    /// * `enabled` - Whether telemetry is enabled.
    pub fn telemetry_set(
        &self,
        enabled: bool,
    ) -> Result<(), system::SystemError> {
        system::telemetry_set(
            &self.client,
            &format!("{}/graphql", self.url),
            enabled,
        )
    }

    /// Perform a system upgrade.
    pub fn system_upgrade_perform(&self) -> Result<(), system::SystemError> {
        system::system_upgrade_perform(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    /// Import users from a v1 database.
    ///
    /// # Arguments
    /// * `mongo_db_conn_string` - The MongoDB connection string.
    /// * `group_code` - The group code to use.
    pub fn system_user_import_from_v1(
        &self,
        mongo_db_conn_string: String,
        group_code: system::SystemImportUsersGroupMode,
    ) -> Result<(), system::SystemError> {
        system::system_user_import_from_v1(
            &self.client,
            &format!("{}/graphql", self.url),
            mongo_db_conn_string,
            group_code,
        )
    }

    /// Set the https redirection flag.
    ///
    /// # Arguments
    /// * `enabled` - Whether https redirection is enabled.
    pub fn https_redirection_set(
        &self,
        enabled: bool,
    ) -> Result<(), system::SystemError> {
        system::https_redirection_set(
            &self.client,
            &format!("{}/graphql", self.url),
            enabled,
        )
    }

    /// Renew the https certificate.
    pub fn https_certificate_renew(&self) -> Result<(), system::SystemError> {
        system::https_certificate_renew(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    // theming functions

    /// List all themes.
    pub fn theme_list(
        &self,
    ) -> Result<Vec<theming::Theme>, theming::ThemeError> {
        theming::theme_list(&self.client, &format!("{}/graphql", self.url))
    }

    /// Get the theme configuration.
    pub fn theme_config_get(
        &self,
    ) -> Result<theming::ThemingConfig, theming::ThemeError> {
        theming::theme_config_get(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    /// Update the theme configuration.
    ///
    /// # Arguments
    /// * `theme` - The new theme.
    /// * `iconset` - The new iconset.
    /// * `dark_mode` - Whether dark mode is enabled.
    /// * `toc_position` - The new toc position.
    /// * `inject_css` - The new css to inject.
    /// * `inject_head` - The new head to inject.
    /// * `inject_body` - The new body to inject.
    #[allow(clippy::too_many_arguments)]
    pub fn theme_config_update(
        &self,
        theme: String,
        iconset: String,
        dark_mode: bool,
        toc_position: Option<String>,
        inject_css: Option<String>,
        inject_head: Option<String>,
        inject_body: Option<String>,
    ) -> Result<(), theming::ThemeError> {
        theming::theme_config_update(
            &self.client,
            &format!("{}/graphql", self.url),
            theme,
            iconset,
            dark_mode,
            toc_position,
            inject_css,
            inject_head,
            inject_body,
        )
    }

    // rendering functions

    /// List all renderers.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply.
    /// * `order_by` - The order by to apply.
    pub fn renderer_list(
        &self,
        filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<Vec<rendering::Renderer>, rendering::RenderingError> {
        rendering::renderer_list(
            &self.client,
            &format!("{}/graphql", self.url),
            filter,
            order_by,
        )
    }

    /// Update renderers.
    ///
    /// # Arguments
    /// * `renderers` - The new renderers.
    pub fn renderer_update(
        &self,
        renderers: Vec<rendering::RendererInput>,
    ) -> Result<(), rendering::RenderingError> {
        rendering::renderer_update(
            &self.client,
            &format!("{}/graphql", self.url),
            renderers,
        )
    }

    // search functions

    /// List search engines.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply.
    /// * `order_by` - The order by to apply.
    pub fn search_engine_list(
        &self,
        filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<Vec<search::SearchEngine>, search::SearchError> {
        search::search_engine_list(
            &self.client,
            &format!("{}/graphql", self.url),
            filter,
            order_by,
        )
    }

    /// Rebuild the search engine index.
    pub fn search_engine_index_rebuild(
        &self,
    ) -> Result<(), search::SearchError> {
        search::search_engine_index_rebuild(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    /// Update search engines.
    ///
    /// # Arguments
    /// * `engines` - The new search engines.
    pub fn search_engine_update(
        &self,
        engines: Vec<search::SearchEngineInput>,
    ) -> Result<(), search::SearchError> {
        search::search_engine_update(
            &self.client,
            &format!("{}/graphql", self.url),
            engines,
        )
    }

    // site functions

    /// Get the site configuration.
    pub fn site_config_get(&self) -> Result<site::SiteConfig, site::SiteError> {
        site::site_config_get(&self.client, &format!("{}/graphql", self.url))
    }

    /// Update the site configuration.
    ///
    /// # Arguments
    /// * `config` - The new site configuration.
    pub fn site_config_update(
        &self,
        config: site::SiteConfig,
    ) -> Result<(), site::SiteError> {
        site::site_config_update(
            &self.client,
            &format!("{}/graphql", self.url),
            config,
        )
    }

    // storage functions

    /// Execute a storage action.
    ///
    /// # Arguments
    /// * `target_key` - The target key.
    /// * `handler` - The handler.
    pub fn storage_action_execute(
        &self,
        target_key: String,
        handler: String,
    ) -> Result<(), storage::StorageError> {
        storage::storage_action_execute(
            &self.client,
            &format!("{}/graphql", self.url),
            target_key,
            handler,
        )
    }

    /// List all storage target's status.
    pub fn storage_status_list(
        &self,
    ) -> Result<Vec<storage::StorageStatus>, storage::StorageError> {
        storage::storage_status_list(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    /// List all storage targets.
    pub fn storage_target_list(
        &self,
    ) -> Result<Vec<storage::StorageTarget>, storage::StorageError> {
        storage::storage_target_list(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    /// Update storage targets.
    ///
    /// # Arguments
    /// * `targets` - The new storage targets.
    pub fn storage_target_update(
        &self,
        targets: Vec<storage::StorageTargetInput>,
    ) -> Result<(), storage::StorageError> {
        storage::storage_target_update(
            &self.client,
            &format!("{}/graphql", self.url),
            targets,
        )
    }
}
