mod get_eth_address;

use integration_tests::PocketIcTestEnv;

#[tokio::test]
async fn test_should_install_canisters() {
    PocketIcTestEnv::init().await;
}
