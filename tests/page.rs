mod common;
use common::API;

use wikijs::page::PageError;


#[test]
fn get_nonexistent_page() {
    let result = API.get_page(1000000);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err() == PageError::PageNotFound, true);
}

#[test]
fn list_no_pages() {
    let result = API.list_all_pages();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn list_no_page_tags() {
    let result = API.list_all_page_tags();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}
