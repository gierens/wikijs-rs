mod common;
use common::API;

use serial_test::serial;
use wikijs::page::{PageError, PageTreeMode};

#[test]
#[serial]
fn page_get_nonexistent() {
    let result = API.page_get(1000000);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err() == PageError::PageNotFound, true);
}

#[test]
#[serial]
fn page_get_by_path_nonexistent() {
    let result = API.page_get_by_path("qwerty".to_string(), "en".to_string());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err() == PageError::PageNotFound, true);
}

#[test]
#[serial]
fn page_list_empty() {
    let result = API.page_list();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
#[serial]
fn page_tag_list_empty() {
    let result = API.page_tag_list();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
#[serial]
fn page_tree_empty() {
    let result = API.page_tree(0, PageTreeMode::ALL, true, "en".to_string());
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
#[serial]
fn page_delete_nonexistent() {
    let result = API.page_delete(1000000);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err() == PageError::PageNotFound, true);
}

#[test]
#[serial]
fn page_render_nonexistent() {
    let result = API.page_render(1000000);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err() == PageError::PageNotFound, true);
}

#[test]
#[serial]
fn page_create_get_delete() {
    let result = API.page_create(
        "...".to_string(),
        "".to_string(),
        "markdown".to_string(),
        true,
        false,
        "en".to_string(),
        "test".to_string(),
        None,
        None,
        None,
        None,
        Vec::new(),
        "test".to_string(),
    );
    assert!(result.is_ok());
    let result2 = API.page_get_by_path("test".to_string(), "en".to_string());
    assert!(result2.is_ok());
    let id = result2.unwrap().id;
    let result3 = API.page_delete(id);
    assert!(result3.is_ok());
    let result4 = API.page_get(id);
    assert!(result4.is_err());
}

#[test]
#[serial]
fn page_update() {
    let result = API.page_create(
        "...".to_string(),
        "".to_string(),
        "markdown".to_string(),
        true,
        false,
        "en".to_string(),
        "test".to_string(),
        None,
        None,
        None,
        None,
        Vec::new(),
        "test".to_string(),
    );
    assert!(result.is_ok());
    let result2 = API.page_get_by_path("test".to_string(), "en".to_string());
    assert!(result2.is_ok());
    let id = result2.unwrap().id;
    let result3 = API.page_update(
        id,
        None,
        Some("test2".to_string()),
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
        Some("test2".to_string()),
    );
    assert!(result3.is_ok());
    let result4 = API.page_get(id);
    assert!(result4.is_ok());
    let page = result4.unwrap();
    assert_eq!(page.description, "test2");
    assert_eq!(page.title, "test2");
    let result5 = API.page_delete(id);
    assert!(result5.is_ok());
}
