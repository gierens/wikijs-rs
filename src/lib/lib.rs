// #![doc(
//     html_logo_url = "https://raw.githubusercontent.com/gierens/wikijs-rs/main/logo/logo.svg",
//     html_favicon_url = "https://raw.githubusercontent.com/gierens/wikijs-rs/main/logo/favicon.ico"
// )]
use reqwest::blocking::Client;
use reqwest::header::{HeaderValue, AUTHORIZATION};

pub mod analytics;
pub mod asset;
pub mod authentication;
pub mod comment;
pub mod common;
pub mod contribute;
pub mod group;
pub mod localization;
pub mod logging;
pub mod mail;
pub mod navigation;
pub mod page;
// pub mod search;
pub mod system;
pub mod theming;
pub mod user;

#[derive(Debug)]
pub enum Credentials {
    Key(String),
    UsernamePassword(String, String, String),
}

#[derive(Debug)]
pub struct Api {
    pub(crate) url: String,
    pub(crate) client: Client,
}

impl Api {
    pub fn new(url: String, credentials: Credentials) -> Self {
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
                )
                .unwrap();
                auth_response.jwt.unwrap()
            }
        };
        Self {
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
        }
    }

    // asset functions
    pub fn asset_list(
        &self,
        folder_id: i64,
        kind: asset::AssetKind,
    ) -> Result<Vec<asset::AssetListItem>, asset::AssetError> {
        asset::asset_list(
            &self.client,
            &format!("{}/graphql", self.url),
            folder_id,
            kind,
        )
    }

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

    pub fn asset_delete(&self, id: i64) -> Result<(), asset::AssetError> {
        asset::asset_delete(&self.client, &format!("{}/graphql", self.url), id)
    }

    pub fn asset_temp_upload_flush(&self) -> Result<(), asset::AssetError> {
        asset::asset_temp_upload_flush(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    pub fn asset_download(
        &self,
        path: String,
    ) -> Result<Vec<u8>, asset::AssetError> {
        asset::asset_download(&self.client, self.url.as_str(), path)
    }

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
    /// );
    /// println!("{:?}", api.page_get(1).unwrap());
    /// ```
    pub fn page_get(&self, id: i64) -> Result<page::Page, page::PageError> {
        page::page_get(&self.client, &format!("{}/graphql", self.url), id)
    }

    pub fn page_get_by_path(
        &self,
        path: String,
        locate: String,
    ) -> Result<page::Page, page::PageError> {
        page::page_get_by_path(
            &self.client,
            &format!("{}/graphql", self.url),
            path,
            locate,
        )
    }

    pub fn page_tag_list(&self) -> Result<Vec<page::PageTag>, page::PageError> {
        page::page_tag_list(&self.client, &format!("{}/graphql", self.url))
    }

    pub fn page_list(
        &self,
    ) -> Result<Vec<page::PageListItem>, page::PageError> {
        page::page_list(&self.client, &format!("{}/graphql", self.url))
    }

    pub fn page_tree(
        &self,
        parent: i64,
        mode: page::PageTreeMode,
        include_ancestors: bool,
        locale: String,
    ) -> Result<Vec<page::PageTreeItem>, page::PageError> {
        page::page_tree(
            &self.client,
            &format!("{}/graphql", self.url),
            parent,
            mode,
            include_ancestors,
            locale,
        )
    }

    pub fn page_delete(&self, id: i64) -> Result<(), page::PageError> {
        page::page_delete(&self.client, &format!("{}/graphql", self.url), id)
    }

    pub fn page_render(&self, id: i64) -> Result<(), page::PageError> {
        page::page_render(&self.client, &format!("{}/graphql", self.url), id)
    }

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

    pub fn page_link_get(
        &self,
        locale: String,
    ) -> Result<Vec<page::PageLinkItem>, page::PageError> {
        page::page_link_get(
            &self.client,
            &format!("{}/graphql", self.url),
            locale,
        )
    }

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

    pub fn page_tag_delete(&self, id: i64) -> Result<(), page::PageError> {
        page::page_tag_delete(
            &self.client,
            &format!("{}/graphql", self.url),
            id,
        )
    }

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

    pub fn page_cache_flush(&self) -> Result<(), page::PageError> {
        page::page_cache_flush(&self.client, &format!("{}/graphql", self.url))
    }

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

    pub fn page_tree_rebuild(&self) -> Result<(), page::PageError> {
        page::page_tree_rebuild(&self.client, &format!("{}/graphql", self.url))
    }

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

    pub fn api_key_list(
        &self,
    ) -> Result<Vec<authentication::ApiKey>, user::UserError> {
        authentication::api_key_list(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    pub fn api_state_get(&self) -> Result<bool, user::UserError> {
        authentication::api_state_get(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    pub fn authentication_strategy_list(
        &self,
    ) -> Result<Vec<authentication::AuthenticationStrategy>, user::UserError>
    {
        authentication::authentication_strategy_list(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

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

    pub fn api_key_revoke(&self, id: i64) -> Result<(), user::UserError> {
        authentication::api_key_revoke(
            &self.client,
            &format!("{}/graphql", self.url),
            id,
        )
    }

    pub fn api_state_set(&self, enabled: bool) -> Result<(), user::UserError> {
        authentication::api_state_set(
            &self.client,
            &format!("{}/graphql", self.url),
            enabled,
        )
    }

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

    pub fn authentication_certificate_regenerate(
        &self,
    ) -> Result<(), user::UserError> {
        authentication::authentication_certificate_regenerate(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    pub fn guest_user_reset(&self) -> Result<(), user::UserError> {
        authentication::guest_user_reset(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    // contribute functions
    pub fn contributor_list(
        &self,
    ) -> Result<Vec<contribute::Contributor>, contribute::ContributeError> {
        contribute::contributor_list(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    // analytics functions
    pub fn analytics_provider_list(
        &self,
    ) -> Result<Vec<analytics::AnalyticsProvider>, analytics::AnalyticsError>
    {
        analytics::analytics_provider_list(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

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

    pub fn comment_provider_list(
        &self,
    ) -> Result<Vec<comment::CommentProvider>, comment::CommentError> {
        comment::comment_provider_list(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    pub fn comment_get(
        &self,
        id: i64,
    ) -> Result<comment::Comment, comment::CommentError> {
        comment::comment_get(&self.client, &format!("{}/graphql", self.url), id)
    }

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

    pub fn comment_delete(&self, id: i64) -> Result<(), comment::CommentError> {
        comment::comment_delete(
            &self.client,
            &format!("{}/graphql", self.url),
            id,
        )
    }

    // user functions
    pub fn user_get(&self, id: i64) -> Result<user::User, user::UserError> {
        user::user_get(&self.client, &format!("{}/graphql", self.url), id)
    }

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

    pub fn user_activate(&self, id: i64) -> Result<(), user::UserError> {
        user::user_activate(&self.client, &format!("{}/graphql", self.url), id)
    }

    pub fn user_deactivate(&self, id: i64) -> Result<(), user::UserError> {
        user::user_deactivate(
            &self.client,
            &format!("{}/graphql", self.url),
            id,
        )
    }

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

    pub fn user_tfa_disable(&self, id: i64) -> Result<(), user::UserError> {
        user::user_tfa_disable(
            &self.client,
            &format!("{}/graphql", self.url),
            id,
        )
    }

    pub fn user_tfa_enable(&self, id: i64) -> Result<(), user::UserError> {
        user::user_tfa_enable(
            &self.client,
            &format!("{}/graphql", self.url),
            id,
        )
    }

    pub fn user_verify(&self, id: i64) -> Result<(), user::UserError> {
        user::user_verify(&self.client, &format!("{}/graphql", self.url), id)
    }

    pub fn user_search(
        &self,
        query: String,
    ) -> Result<Vec<user::UserMinimal>, user::UserError> {
        user::user_search(&self.client, &format!("{}/graphql", self.url), query)
    }

    pub fn user_profile_get(
        &self,
    ) -> Result<user::UserProfile, user::UserError> {
        user::user_profile_get(&self.client, &format!("{}/graphql", self.url))
    }

    pub fn user_last_login_list(
        &self,
    ) -> Result<Vec<user::UserLastLogin>, user::UserError> {
        user::user_last_login_list(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

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

    pub fn user_password_reset(&self, id: i64) -> Result<(), user::UserError> {
        user::user_password_reset(
            &self.client,
            &format!("{}/graphql", self.url),
            id,
        )
    }

    // group functions
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

    pub fn group_get(
        &self,
        id: i64,
    ) -> Result<group::Group, group::GroupError> {
        group::group_get(&self.client, &format!("{}/graphql", self.url), id)
    }

    pub fn group_create(&self, name: String) -> Result<(), group::GroupError> {
        group::group_create(
            &self.client,
            &format!("{}/graphql", self.url),
            name,
        )
    }

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

    pub fn group_delete(&self, id: i64) -> Result<(), group::GroupError> {
        group::group_delete(&self.client, &format!("{}/graphql", self.url), id)
    }

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
    pub fn locale_list(
        &self,
    ) -> Result<Vec<localization::Locale>, localization::LocaleError> {
        localization::locale_list(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    pub fn locale_config_get(
        &self,
    ) -> Result<localization::LocaleConfig, localization::LocaleError> {
        localization::locale_config_get(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

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
    pub fn mail_config_get(&self) -> Result<mail::MailConfig, mail::MailError> {
        mail::mail_config_get(&self.client, &format!("{}/graphql", self.url))
    }

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

    // system functions
    pub fn system_flag_list(
        &self,
    ) -> Result<Vec<system::SystemFlag>, system::SystemError> {
        system::system_flag_list(&self.client, &format!("{}/graphql", self.url))
    }

    pub fn system_info_get(
        &self,
    ) -> Result<system::SystemInfo, system::SystemError> {
        system::system_info_get(&self.client, &format!("{}/graphql", self.url))
    }

    pub fn system_extension_list(
        &self,
    ) -> Result<Vec<system::SystemExtension>, system::SystemError> {
        system::system_extension_list(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    pub fn system_export_status_get(
        &self,
    ) -> Result<system::SystemExportStatus, system::SystemError> {
        system::system_export_status_get(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

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

    pub fn telemetry_client_id_reset(&self) -> Result<(), system::SystemError> {
        system::telemetry_client_id_reset(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

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

    pub fn system_upgrade_perform(&self) -> Result<(), system::SystemError> {
        system::system_upgrade_perform(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

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

    pub fn https_certificate_renew(&self) -> Result<(), system::SystemError> {
        system::https_certificate_renew(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    // theming functions
    pub fn theme_list(
        &self,
    ) -> Result<Vec<theming::Theme>, theming::ThemeError> {
        theming::theme_list(&self.client, &format!("{}/graphql", self.url))
    }

    pub fn theme_config_get(
        &self,
    ) -> Result<theming::ThemingConfig, theming::ThemeError> {
        theming::theme_config_get(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

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
}
