#[test]
fn cli() {
    trycmd::TestCases::new().case("tests/cli/*.toml");
}

// add more tests with preperation code here and put them in subfolder
// list creating page and then showing it ...
// some need to be made serial
