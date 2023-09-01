mod common;
use common::API;

use wikijs::page::PageError;

#[test]
fn nonexistent_page() {
    let result = API.page_get(1000000);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err() == PageError::PageNotFound, true);
}

#[test]
fn list_no_pages() {
    let result = API.page_list();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn list_no_page_tags() {
    let result = API.page_tag_list();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn get_empty_page_tree() {
    let result = API.page_tree(0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}
