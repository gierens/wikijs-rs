mod common;
use common::API;

#[test]
fn asset_list_empty() {
    // TODO this needs a more elaborate check
    let result = API.asset_list(0, wikijs::asset::AssetKind::ALL);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}
