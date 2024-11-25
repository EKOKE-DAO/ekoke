use did::deferred::{Agency, ContractRegistration, ContractType, GenericValue, Seller};
use integration_tests::actor::agent;
use integration_tests::client::{DeferredDataClient, DeferredMinterClient};
use integration_tests::eth_rpc_client::{DeferredErc721Client, EthRpcClient};
use integration_tests::{DfxTestEnv, WalletName};

const ONE_ETH: u64 = 1_000_000_000_000_000_000;

#[tokio::test]
async fn test_should_create_and_close_contract() {
    let env = DfxTestEnv::init().await;

    // create agent
    DeferredMinterClient::new(&env)
        .admin_register_agency(
            agent(),
            Agency {
                owner: agent(),
                ..Default::default()
            },
        )
        .await;

    let minter_address = DeferredMinterClient::new(&env)
        .get_eth_address()
        .await
        .expect("Failed to get eth address");

    println!("Eth address: {minter_address}",);

    // transfer ETH to create the token on Ethereum
    EthRpcClient::new(&env)
        .send_eth(WalletName::Owner, minter_address, ONE_ETH)
        .await
        .expect("Failed to send eth");

    // create the contract
    let request = ContractRegistration {
        r#type: ContractType::Sell,
        sellers: vec![Seller {
            address: env.evm.get_eth_address(WalletName::Alice),
            quota: 100,
        }],
        buyers: vec![env.evm.get_eth_address(WalletName::Bob)],
        value: 500_000,
        token_value: 100,
        installments: 500_000 / 100,
        currency: "USD".to_string(),
        deposit: 10_000,
        expiration: "2050-01-01".to_string(),
        properties: vec![(
            "contract:address".to_string(),
            GenericValue::TextContent("Via Roma 10".to_string()),
        )],
        restricted_properties: vec![],
    };

    // send request
    let contract_id = DeferredMinterClient::new(&env)
        .create_contract(agent(), request)
        .await
        .expect("Failed to create contract");

    // check contract exists on ERC721
    let sell_contract = DeferredErc721Client::new(&env)
        .token_contract(0)
        .await
        .expect("Failed to get token contract");

    assert_eq!(
        sell_contract.buyers,
        vec![env.evm.get_eth_address(WalletName::Bob).0]
    );
    assert_eq!(
        sell_contract.sellers,
        vec![integration_tests::abi::Seller {
            seller: env.evm.get_eth_address(WalletName::Alice).0,
            token_from_id: 0.into(),
            token_to_id: ((500_000 / 100) - 1).into(),
        }]
    );

    // get contract on data
    let data_client = DeferredDataClient::new(&env);

    let contract = data_client
        .get_contract(&contract_id)
        .await
        .expect("Failed to get contract");

    assert_eq!(contract.closed, false);
    assert_eq!(contract.value, 500_000);
    assert_eq!(contract.id, contract_id);

    // close contract
    DeferredMinterClient::new(&env)
        .close_contract(agent(), contract_id.clone())
        .await
        .expect("Failed to close contract");

    assert_eq!(data_client.get_contract(&contract_id).await, None);
}
