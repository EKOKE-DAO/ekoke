mod ekoke_swapped;
mod response;

use did::ekoke::{EkokeResult, EthNetwork, PicoEkoke};
use did::H160;
use ethers_core::abi::AbiEncode;
use ethers_core::types::{Bytes, TransactionRequest};
use num_traits::cast::ToPrimitive;

use self::ekoke_swapped::EkokeSwapped;
#[cfg(target_family = "wasm")]
use self::response::EthRpcResponse;
use super::swap_fee::SwapFee;
use crate::app::configuration::Configuration;
use crate::constants::TRANSCRIBE_SWAP_TX_GAS;

#[cfg(target_family = "wasm")]
const GOERLI_PUBLICNODE_URL: &str = "https://ethereum-goerli.publicnode.com";
#[cfg(target_family = "wasm")]
const SEPOLIA_PUBLICNODE_URL: &str = "https://ethereum-sepolia.publicnode.com";
#[cfg(target_family = "wasm")]
const MAINNET_PUBLICNODE_URL: &str = "https://ethereum.publicnode.com";
#[cfg(target_family = "wasm")]
const MAINNET_CLOUDFLARE_URL: &str = "https://cloudflare-eth.com";
#[cfg(target_family = "wasm")]
const MAINNET_ANKR_URL: &str = "https://rpc.ankr.com/eth";
#[cfg(target_family = "wasm")]
const HEADER_SIZE_LIMIT: u64 = 2 * 1024;
#[cfg(target_family = "wasm")]
const HTTP_MAX_SIZE: u64 = 2_000_000;
#[cfg(target_family = "wasm")]
const HTTP_RESPONSE_SIZE_LIMIT: u64 = 2048;
#[cfg(target_family = "wasm")]
const BASE_SUBNET_SIZE: u128 = 13;
#[cfg(target_family = "wasm")]
const SUBNET_SIZE: u128 = 34;

/// Ethereum RPC client
pub struct EthRpcClient {
    #[allow(dead_code)]
    network: EthNetwork,
}

impl EthRpcClient {
    /// Creates a new Ethereum RPC client
    pub fn new(network: EthNetwork) -> Self {
        Self { network }
    }

    /// Send transaction
    #[cfg(target_family = "wasm")]
    pub async fn send_tx(&self, signed_tx: Bytes) -> EkokeResult<String> {
        use ic_cdk::api::management_canister::http_request::{
            http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, TransformContext,
            TransformFunc,
        };

        let payload = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_sendRawTransaction",
            "params": [signed_tx],
            "id": 1
        });

        // get cycles to pay
        let effective_size_estimate = HTTP_RESPONSE_SIZE_LIMIT + HEADER_SIZE_LIMIT;
        let base_cycles = 400_000_000u128 + 100_000u128 * (2 * effective_size_estimate as u128);

        let cycles = base_cycles * SUBNET_SIZE / BASE_SUBNET_SIZE;

        let mut error = None;

        // iterate over endpoints until one succeeds
        for endpoint in self.rpc_endpoints() {
            let http_argument = CanisterHttpRequestArgument {
                url: endpoint.to_string(),
                body: Some(payload.to_string().as_bytes().to_vec()),
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
            match self.transform_result(response) {
                Ok(hash) => return Ok(hash),
                Err(e) => {
                    error = Some(e);
                }
            }
        }

        Err(error.unwrap())
    }

    /// Send transaction
    #[cfg(not(target_family = "wasm"))]
    #[allow(unused_variables)]
    pub async fn send_tx(&self, signed_tx: Bytes) -> EkokeResult<String> {
        Ok("0x613bbe6a28d9ecd3ff278a60b476f5daac096e28164efb749bea47dcc11a57bf".to_string())
    }

    /// Get next nonce
    #[cfg(target_family = "wasm")]
    pub async fn get_next_nonce(&self, wallet_address: H160) -> EkokeResult<u64> {
        use ic_cdk::api::management_canister::http_request::{
            http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, TransformContext,
            TransformFunc,
        };

        let payload = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_getTransactionCount",
            "params": [wallet_address.to_hex_str(), "latest"],
            "id": 1
        });

        // get cycles to pay
        let effective_size_estimate = HTTP_RESPONSE_SIZE_LIMIT + HEADER_SIZE_LIMIT;
        let base_cycles = 400_000_000u128 + 100_000u128 * (2 * effective_size_estimate as u128);

        let cycles = base_cycles * SUBNET_SIZE / BASE_SUBNET_SIZE;

        let mut error = None;

        // iterate over endpoints until one succeeds
        for endpoint in self.rpc_endpoints() {
            let http_argument = CanisterHttpRequestArgument {
                url: endpoint.to_string(),
                body: Some(payload.to_string().as_bytes().to_vec()),
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
            match self.transform_tx_count_result(response) {
                Ok(hex_value) => return Ok(hex_value),
                Err(e) => {
                    error = Some(e);
                }
            }
        }

        Err(error.unwrap())
    }

    /// Get ekoke swapped events.
    ///
    /// Returns the retrieved events and the last block fetched
    #[cfg(not(target_family = "wasm"))]
    #[allow(unused_variables)]
    pub async fn get_ekoke_swapped_events(
        &self,
        from_block: u64,
    ) -> EkokeResult<(u64, Vec<EkokeSwapped>)> {
        Ok((
            10,
            vec![EkokeSwapped {
                data: "0x000000000000000000000000000000000000000000000000002386f26fc10000"
                    .to_string(),
                block_number: "0x11d8aaa".to_string(),
                topics: vec![
                    "0x257e057bb61920d8d0ed2cb7b720ac7f9c513cd1110bc9fa543079154f45f435"
                        .to_string(),
                    "0x00000000000000000000000053d290220b4ae5cd91987517ef04e206c1078850"
                        .to_string(),
                    "0x1d7a3af512fb166ee6447759bd4e3a1c7daa4d98c0b7b8cb1fbb20b62b020000"
                        .to_string(),
                ],
            }],
        ))
    }

    /// Get ekoke swapped events.
    ///
    /// Returns the retrieved events and the last block fetched
    #[cfg(target_family = "wasm")]
    pub async fn get_ekoke_swapped_events(
        &self,
        from_block: u64,
    ) -> EkokeResult<(u64, Vec<EkokeSwapped>)> {
        use ic_cdk::api::management_canister::http_request::{
            http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, TransformContext,
            TransformFunc,
        };

        let contract_address = Configuration::get_erc20_bridge_address();

        let payload = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_getLogs",
            "params": [{
                "fromBlock": format!("0x{:x}", from_block),
                "toBlock": "latest",
                "address": contract_address.to_hex_str(),
                "topic": crate::constants::ERC20_EKOKE_SWAPPED_TOPIC,
            }],
            "id": 1
        });

        // get cycles to pay
        let effective_size_estimate = (1024 * 1024) + HEADER_SIZE_LIMIT;
        let base_cycles = 400_000_000u128 + 100_000u128 * (2 * effective_size_estimate as u128);

        let cycles = base_cycles * SUBNET_SIZE / BASE_SUBNET_SIZE;

        let mut error = None;

        // iterate over endpoints until one succeeds
        for endpoint in self.rpc_endpoints() {
            let http_argument = CanisterHttpRequestArgument {
                url: endpoint.to_string(),
                body: Some(payload.to_string().as_bytes().to_vec()),
                max_response_bytes: Some(1024 * 1024),
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
            match self.transform_eth_get_logs_result(response) {
                Ok(response) => return Ok(response),
                Err(e) => {
                    error = Some(e);
                }
            }
        }

        Err(error.unwrap())
    }

    /// Get next nonce
    #[cfg(not(target_family = "wasm"))]
    #[allow(unused_variables)]
    pub async fn get_next_nonce(&self, wallet_address: H160) -> EkokeResult<u64> {
        Ok(0)
    }

    /// Build the Eth transaction to call `transcribeSwap` on the ERC20 Ekoke contract
    pub async fn ekoke_transcribe_swap_tx(
        &self,
        from: H160,
        recipient: H160,
        amount: PicoEkoke,
    ) -> EkokeResult<TransactionRequest> {
        let ekoke_erc20: H160 = Configuration::get_erc20_bridge_address();
        let nonce = self.get_next_nonce(from.clone()).await?;

        let payload = crate::abi::EkokeCalls::TranscribeSwap(crate::abi::TranscribeSwapCall {
            recipient: recipient.0,
            amount: amount.0.to_u64().unwrap_or_default().into(),
        })
        .encode();

        let gas_price = SwapFee::get_gas_price();

        Ok(TransactionRequest {
            from: Some(from.0),
            to: Some(ekoke_erc20.0.into()),
            value: None,
            gas: Some(TRANSCRIBE_SWAP_TX_GAS.into()),
            gas_price: Some(gas_price.into()),
            data: Some(Bytes::from(payload)),
            nonce: Some(nonce.into()),
            chain_id: Some(Configuration::get_eth_network().chain_id().into()),
        })
    }

    /// Transform http result into ekoke result
    #[cfg(target_family = "wasm")]
    fn transform_result(
        &self,
        result: Result<
            ic_cdk::api::management_canister::http_request::HttpResponse,
            (ic_cdk::api::call::RejectionCode, String),
        >,
    ) -> EkokeResult<String> {
        let http_response =
            result.map_err(|(code, msg)| did::ekoke::EkokeError::CanisterCall(code, msg))?;
        // deserialize body
        let ethrpc_response: EthRpcResponse = serde_json::from_slice(http_response.body.as_slice())
            .map_err(|e| did::ekoke::EkokeError::EthRpcError(0, e.to_string()))?;

        ethrpc_response
            .into_result()
            .map_err(|e| did::ekoke::EkokeError::EthRpcError(e.code, e.message))
    }

    /// Transform http result into ekoke result
    #[cfg(target_family = "wasm")]
    fn transform_tx_count_result(
        &self,
        result: Result<
            ic_cdk::api::management_canister::http_request::HttpResponse,
            (ic_cdk::api::call::RejectionCode, String),
        >,
    ) -> EkokeResult<u64> {
        let http_response =
            result.map_err(|(code, msg)| did::ekoke::EkokeError::CanisterCall(code, msg))?;
        // deserialize body
        let ethrpc_response: EthRpcResponse = serde_json::from_slice(http_response.body.as_slice())
            .map_err(|e| did::ekoke::EkokeError::EthRpcError(0, e.to_string()))?;

        let value = ethrpc_response
            .into_result()
            .map_err(|e| did::ekoke::EkokeError::EthRpcError(e.code, e.message))?;

        let hex = u64::from_str_radix(value.trim_start_matches("0x"), 16)
            .map_err(|e| did::ekoke::EkokeError::EthRpcError(0, e.to_string()))?;

        Ok(hex)
    }

    /// Transform http result into ekoke result
    #[cfg(target_family = "wasm")]
    fn transform_eth_get_logs_result(
        &self,
        result: Result<
            ic_cdk::api::management_canister::http_request::HttpResponse,
            (ic_cdk::api::call::RejectionCode, String),
        >,
    ) -> EkokeResult<(u64, Vec<EkokeSwapped>)> {
        let http_response =
            result.map_err(|(code, msg)| did::ekoke::EkokeError::CanisterCall(code, msg))?;

        let ethrpc_response: self::ekoke_swapped::EkokeSwappedRpcResult =
            serde_json::from_slice(http_response.body.as_slice())
                .map_err(|e| did::ekoke::EkokeError::EthRpcError(0, e.to_string()))?;

        let last_block_number = ethrpc_response
            .result
            .iter()
            .map(|r| r.block_number().unwrap())
            .max()
            .unwrap_or(0);

        Ok((last_block_number, ethrpc_response.result))
    }

    /// Returns the RPC endpoints for the Ethereum network
    #[cfg(target_family = "wasm")]
    fn rpc_endpoints(&self) -> Vec<&'static str> {
        match self.network {
            EthNetwork::Goerli => vec![GOERLI_PUBLICNODE_URL],
            EthNetwork::Ethereum => vec![
                MAINNET_CLOUDFLARE_URL,
                MAINNET_PUBLICNODE_URL,
                MAINNET_ANKR_URL,
            ],
            EthNetwork::Sepolia => vec![SEPOLIA_PUBLICNODE_URL],
        }
    }
}
