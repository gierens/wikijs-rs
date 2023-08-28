mod common;
use common::API;

use serial_test::serial;

#[test]
#[serial]
fn list_all_pages_empty() {
    assert_eq!(API.list_all_pages().unwrap().len(), 0);
}

#[test]
#[serial]
fn list_all_page_tags_empty() {
    assert_eq!(API.list_all_page_tags().unwrap().len(), 0);
}
