mod common;
use common::API;

use wikijs::page::PageError;

#[test]
fn page_get_nonexistent() {
    let result = API.page_get(1000000);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err() == PageError::PageNotFound, true);
}

#[test]
fn page_get_by_path_nonexistent() {
    let result = API.page_get_by_path("test".to_string(), "en".to_string());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err() == PageError::PageNotFound, true);
}

#[test]
fn page_list_empty() {
    let result = API.page_list();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn page_tag_list_empty() {
    let result = API.page_tag_list();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn page_tree_empty() {
    let result = API.page_tree(0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn page_delete_nonexistent() {
    let result = API.page_delete(1000000);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err() == PageError::PageNotFound, true);
}

#[test]
fn page_render_nonexistent() {
    let result = API.page_render(1000000);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err() == PageError::PageNotFound, true);
}
