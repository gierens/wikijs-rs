mod common;
use common::API;

#[test]
fn contributors() {
    let result = API.contributor_list();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 299);
}
