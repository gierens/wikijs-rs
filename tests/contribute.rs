mod common;
use common::API;

#[test]
fn contributor_list() {
    // TODO this needs a more elaborate check
    let result = API.contributor_list();
    assert!(result.is_ok());
    assert!(result.unwrap().len() > 200);
}
