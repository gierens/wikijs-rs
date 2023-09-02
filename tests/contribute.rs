mod common;
use common::API;

use serial_test::serial;

#[test]
#[serial]
fn contributor_list() {
    // TODO this needs a more elaborate check
    let result = API.contributor_list();
    assert!(result.is_ok());
    assert!(result.unwrap().len() > 200);
}
