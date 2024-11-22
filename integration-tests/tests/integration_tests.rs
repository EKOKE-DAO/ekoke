use integration_tests::TestEnv;

#[test]
#[serial_test::serial]
fn test_should_install_canisters() {
    TestEnv::init();
}
