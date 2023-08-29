mod common;
use common::API;

use wikijs::page::PageError;


#[test]
fn get_nonexistent_page() {
    let result = API.get_page(1000000);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err() == PageError::PageNotFound, true);
}
