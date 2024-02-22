use integration_tests::client::HttpClient;
use integration_tests::TestEnv;
use serde_json::Value;

#[test]
#[serial_test::serial]
fn test_http_should_get_icrc1_metadata() {
    let env = TestEnv::init();

    let http_client = HttpClient::new(env.ekoke_ledger_id, &env);

    let icrc1_name: Value = http_client.http_request("icrc1Name", serde_json::json!({}));
    assert_eq!(icrc1_name["name"], "ekoke");

    let icrc1_symbol: Value = http_client.http_request("icrc1Symbol", serde_json::json!({}));
    assert_eq!(icrc1_symbol["symbol"], "EKOKE");

    let icrc1_decimals: Value = http_client.http_request("icrc1Decimals", serde_json::json!({}));
    assert_eq!(icrc1_decimals["decimals"], 12);

    let icrc1_total_supply: Value =
        http_client.http_request("icrc1TotalSupply", serde_json::json!({}));
    assert_eq!(icrc1_total_supply["totalSupply"], 8880101010000000000u64);

    let icrc1_fee: Value = http_client.http_request("icrc1Fee", serde_json::json!({}));
    assert_eq!(icrc1_fee["fee"], 10_000u64);
}
