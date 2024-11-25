mod use_case;

use integration_tests::TestEnv;

#[tokio::test]
async fn test_should_install_canisters() {
    TestEnv::init().await;
}
