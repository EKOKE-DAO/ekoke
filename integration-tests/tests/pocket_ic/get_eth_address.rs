use did::H160;
use integration_tests::client::DeferredMinterClient;
use integration_tests::PocketIcTestEnv;
use pretty_assertions::assert_ne;

#[tokio::test]
async fn test_should_get_eth_address() {
    let env = PocketIcTestEnv::init().await;

    let client = DeferredMinterClient::new(&env);

    let address = client
        .get_eth_address()
        .await
        .expect("Failed to get eth address");

    println!("Eth address: {:?}", address);
    assert_ne!(address, H160::zero());
}
