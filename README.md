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
- [ ] `asset_get` ... doesn't exist in the API
- [x] `asset_folder_list`
- [x] `asset_folder_create`
- [x] `asset_rename`
- [x] `asset_delete`
- [x] `asset_flush_temp_uploads`
- [x] `asset_download` ... this needs the REST API
- [x] `asset_upload` ... this needs the REST API

### authentication
- [x] `api_key_list`
- [x] `api_state_get`
- [x] `authentication_strategy_list`
- [x] `authentication_active_strategy_list`
- [x] `api_key_create`
- [x] `login`
- [x] `login_tfa`
- [x] `login_change_password`
- [x] `forgot_password`
- [x] `register`
- [x] `api_key_revoke`
- [x] `api_state_set`
- [x] `authentication_strategy_update`
- [x] `certificate_regenerate`
- [x] `guest_user_reset`

### comment
- [x] `comment_provider_list`
- [x] `comment_list`
- [x] `comment_get`
- [x] `comment_provider_update`
- [x] `comment_create`
- [x] `comment_update`
- [x] `comment_delete`

### contribute
- [x] `contributor_list`

### group
- [x] `group_list`
- [x] `group_get`
- [x] `group_create`
- [x] `group_update`
- [x] `group_delete`
- [x] `group_assign_user`
- [x] `group_unassign_user`

### localization
- [x] `locale_list`
- [x] `locale_config_get`
- [x] `translation_list`
- [x] `locale_download`
- [x] `locale_update`

### logging
- [x] `logger_list`
- [x] `logger_update`

### mail
- [x] `mail_config_get`
- [x] `mail_send_test`
- [x] `mail_config_update`

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
- [x] `system_info_get`
- [x] `system_extension_list`
- [x] `system_status_export_get` ... maybe this should be renamed
- [x] `system_flags_update`
- [x] `telemetry_reset_client_id`
- [x] `telemetry_set`
- [x] `system_upgrade`
- [x] `user_import_from_v1`
- [x] `https_redirection_set`
- [x] `https_certificate_renew`

### theming
- [x] `theme_list`
- [x] `theme_config_get`
- [x] `theme_config_set`

### user
- [x] `user_list`
- [x] `user_search`
- [x] `user_get`
- [x] `user_profile_get`
- [x] `user_last_login_list`
- [x] `user_create`
- [x] `user_update`
- [x] `user_delete`
- [x] `user_verify`
- [x] `user_activate`
- [x] `user_deactivate`
- [x] `user_tfa_enable`
- [x] `user_tfa_disable`
- [x] `user_profile_update`
- [x] `user_password_change`
- [x] `user_password_reset`
