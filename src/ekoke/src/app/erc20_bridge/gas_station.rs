use did::ekoke::EkokeResult;

#[cfg(target_family = "wasm")]
const ETHERSCAN_API_URL: &str = "https://api.etherscan.io/api?module=proxy&action=eth_gasPrice";
#[cfg(target_family = "wasm")]
const HEADER_SIZE_LIMIT: u64 = 2 * 1024;
#[cfg(target_family = "wasm")]
const HTTP_RESPONSE_SIZE_LIMIT: u64 = 2048;
#[cfg(target_family = "wasm")]
const BASE_SUBNET_SIZE: u128 = 13;
#[cfg(target_family = "wasm")]
const SUBNET_SIZE: u128 = 34;

pub struct GasStation;

impl GasStation {
    /// Returns the gas price in wei.
    #[cfg(not(target_family = "wasm"))]
    pub async fn fetch_gas_price() -> EkokeResult<u64> {
        Ok(32_000_000_000_u64)
    }

    /// Returns the gas price in wei.
    #[cfg(target_family = "wasm")]
    pub async fn fetch_gas_price() -> EkokeResult<u64> {
        use ic_cdk::api::management_canister::http_request::{
            http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, TransformContext,
            TransformFunc,
        };
        // get cycles to pay
        let effective_size_estimate = HTTP_RESPONSE_SIZE_LIMIT + HEADER_SIZE_LIMIT;
        let base_cycles = 400_000_000u128 + 100_000u128 * (2 * effective_size_estimate as u128);

        let cycles = base_cycles * SUBNET_SIZE / BASE_SUBNET_SIZE;

        let mut error = None;

        // iterate over endpoints until one succeeds
        for endpoint in Self::api_endpoints() {
            let http_argument = CanisterHttpRequestArgument {
                url: endpoint.to_string(),
                body: None,
                max_response_bytes: Some(HTTP_RESPONSE_SIZE_LIMIT),
                method: HttpMethod::POST,
                headers: vec![HttpHeader {
                    name: "Content-Type".to_string(),
                    value: "application/json".to_string(),
                }],
                transform: Some(TransformContext {
                    function: TransformFunc::new(
                        crate::utils::id(),
                        "http_transform_send_tx".to_string(),
                    ),
                    context: vec![],
                }),
            };
            let response = http_request(http_argument, cycles).await.map(|r| r.0);
            match Self::transform_result(response) {
                Ok(gas_price) => return Ok(gas_price),
                Err(e) => {
                    error = Some(e);
                }
            }
        }

        Err(error.unwrap())
    }

    /// Transform http result into ekoke result
    #[cfg(target_family = "wasm")]
    fn transform_result(
        result: Result<
            ic_cdk::api::management_canister::http_request::HttpResponse,
            (ic_cdk::api::call::RejectionCode, String),
        >,
    ) -> EkokeResult<u64> {
        let http_response =
            result.map_err(|(code, msg)| did::ekoke::EkokeError::CanisterCall(code, msg))?;
        // deserialize body
        let ethrpc_response: EthGasPriceResponse =
            serde_json::from_slice(http_response.body.as_slice())
                .map_err(|e| did::ekoke::EkokeError::EthRpcError(0, e.to_string()))?;

        // convert result (hex repr) to u64
        u64::from_str_radix(ethrpc_response.result.trim_start_matches("0x"), 16)
            .map_err(|_| did::ekoke::EkokeError::EthRpcError(0, ethrpc_response.result))
    }

    #[cfg(target_family = "wasm")]
    fn api_endpoints() -> Vec<&'static str> {
        vec![ETHERSCAN_API_URL]
    }
}

#[cfg(target_family = "wasm")]
#[derive(serde::Deserialize)]
struct EthGasPriceResponse {
    result: String,
}
