mod common;
use common::API;

#[test]
fn list_no_contribute_contributors() {
    let result = API.list_contribute_contributors();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 299);
}
