# wikijs-rs
API bindings, CLI client and FUSE filesystem for Wiki.js written in Rust.

## WARNING
This is a pre-release just to secure the crate name on crates.io and not
in a usable state yet, that's also why the repo is still private. The first
real version will be released in a few days, then I'll also publish the repo.

## Status of the API
The following checklists show what queries and mutations are already
implemented and what is still to be done.

### analytics
- [x] `provider_list`
- [x] `update_providers`

### asset
- [x] `asset_list`
- [x] `asset_folder_list`
- [x] `asset_folder_create`
- [x] `asset_rename`
- [x] `asset_delete`
- [x] `asset_flush_temp_uploads`
- [ ] `asset_get` ... this needs the REST API
- [ ] `asset_create` ... this needs the REST API

### authentication
- [ ] `api_key_list`
- [ ] `api_state_get`
- [ ] `authentication_strategy_list`
- [ ] `authentication_active_strategy_list`
- [ ] `api_key_create`
- [x] `login`
- [ ] `login_tfa`
- [ ] `login_change_password`
- [ ] `forgot_password`
- [ ] `register`
- [ ] `api_key_revoke`
- [ ] `api_state_set`
- [ ] `authentication_strategy_update`
- [ ] `certificate_regenerate`
- [ ] `guest_user_reset`

### comment
- [ ] `comment_provider_list`
- [x] `comment_list`
- [ ] `comment_get`
- [ ] `comment_provider_update`
- [ ] `comment_create`
- [ ] `comment_update`
- [ ] `comment_delete`

### contribute
- [x] `contributor_list`

### group
- [x] `group_list`
- [ ] `group_get`
- [ ] `group_create`
- [ ] `group_update`
- [ ] `group_delete`
- [ ] `group_assign_user`
- [ ] `group_unassign_user`

### localization
- [x] `locale_list`
- [ ] `locale_config_get`
- [ ] `translation_list`
- [ ] `locale_download`
- [ ] `locale_update`

### logging
- [x] `logger_list`
- [ ] `logger_update`

### mail
- [ ] `mail_config_get`
- [ ] `mail_send_test`
- [ ] `mail_config_update`

### navigation
- [ ] `navigation_tree_get`
- [ ] `navigation_config_get`
- [ ] `navigation_tree_update`
- [ ] `navigation_config_update`

### page
- [x] `page_history_get`
- [x] `page_version_get`
- [x] `page_search`
- [ ] `page_list` ... implementation is only there in parts so far
- [x] `page_get`
- [x] `page_get_by_path`
- [x] `page_tag_list`
- [ ] `page_tree` ... should be renamed to `page_tree_get`
- [ ] `page_link_list` ... is done but name `page_link_get`
- [x] `page_check_conflicts`
- [x] `page_conflict_latest`
- [x] `page_create`
- [x] `page_update`
- [x] `page_convert`
- [x] `page_move`
- [x] `page_delete`
- [x] `page_tag_delete`
- [x] `page_tag_update`
- [x] `page_flush_cache`
- [x] `page_migrate_to_locale`
- [x] `page_rebuild_tree`
- [x] `page_render`
- [x] `page_restore`
- [x] `page_purge_history`

### rendering
- [ ] `renderer_list`
- [ ] `renderer_update`

### search
- [ ] `search_engine_list`
- [ ] `search_engine_update`
- [ ] `search_engine_rebuild_index`

### site
- [ ] `site_config_get`
- [ ] `site_config_update`

### storage
- [ ] `storage_target_list`
- [ ] `storage_status_list`
- [ ] `storage_target_update`
- [ ] `storage_execute_action`

### system
- [x] `system_flags_list`
- [ ] `system_info_get`
- [ ] `system_extension_list`
- [ ] `system_status_export`
- [ ] `system_flags_update`
- [ ] `telemetry_reset_client_id`
- [ ] `telemetry_set`
- [ ] `system_upgrade`
- [ ] `user_import_from_v1`
- [ ] `http_redirection_set`
- [ ] `https_certificate_renew`
- [ ] `system_export`

### theming
- [x] `theme_list`
- [ ] `theme_config_get`
- [ ] `theme_config_set`

### user
- [x] `user_list`
- [x] `user_search`
- [x] `user_get`
- [x] `user_profile_get`
- [ ] `user_last_login_list`
- [ ] `user_create`
- [ ] `user_update`
- [x] `user_delete`
- [x] `user_verify`
- [x] `user_activate`
- [x] `user_deactivate`
- [x] `user_tfa_enable`
- [x] `user_tfa_disable`
- [ ] `user_profile_update`
- [ ] `user_password_change`
- [ ] `user_password_reset`
