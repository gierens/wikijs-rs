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
// pub mod mail;
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

    // page functions
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

    pub fn user_activate(
        &self,
        id: i64,
    ) -> Result<(), user::UserError> {
        user::user_activate(&self.client, &format!("{}/graphql", self.url), id)
    }

    pub fn user_deactivate(
        &self,
        id: i64,
    ) -> Result<(), user::UserError> {
        user::user_deactivate(&self.client, &format!("{}/graphql", self.url), id)
    }

    pub fn user_delete(
        &self,
        id: i64,
        replace_id: i64,
    ) -> Result<(), user::UserError> {
        user::user_delete(&self.client, &format!("{}/graphql", self.url), id, replace_id)
    }

    pub fn user_tfa_disable(
        &self,
        id: i64,
    ) -> Result<(), user::UserError> {
        user::user_tfa_disable(&self.client, &format!("{}/graphql", self.url), id)
    }

    pub fn user_tfa_enable(
        &self,
        id: i64,
    ) -> Result<(), user::UserError> {
        user::user_tfa_enable(&self.client, &format!("{}/graphql", self.url), id)
    }

    pub fn user_verify(
        &self,
        id: i64,
    ) -> Result<(), user::UserError> {
        user::user_verify(&self.client, &format!("{}/graphql", self.url), id)
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

    // locale functions
    pub fn locale_list(
        &self,
    ) -> Result<Vec<localization::Locale>, localization::LocaleError> {
        localization::locale_list(
            &self.client,
            &format!("{}/graphql", self.url),
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

    // system functions
    pub fn system_flag_list(
        &self,
    ) -> Result<Vec<system::SystemFlag>, system::SystemError> {
        system::system_flag_list(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }

    // theming functions
    pub fn theme_list(
        &self,
    ) -> Result<Vec<theming::Theme>, theming::ThemeError> {
        theming::theme_list(
            &self.client,
            &format!("{}/graphql", self.url),
        )
    }
}
