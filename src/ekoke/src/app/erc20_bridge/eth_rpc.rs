mod response;

use did::ekoke::{EkokeResult, EthNetwork, PicoEkoke};
use did::H160;
use ethers_core::abi::AbiEncode;
use ethers_core::types::{Bytes, TransactionRequest};
use num_traits::cast::ToPrimitive;

#[cfg(target_family = "wasm")]
use self::response::EthRpcResponse;
use super::swap_fee::SwapFee;
use crate::app::configuration::Configuration;

#[cfg(target_family = "wasm")]
const GOERLI_PUBLICNODE_URL: &str = "https://ethereum-goerli.publicnode.com";
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

const SWAP_TX_GAS: u64 = 108210;

/// Ethereum RPC client
pub struct EthRpcClient {
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
            recipient: recipient.0.into(),
            amount: amount.0.to_u64().unwrap_or_default().into(),
        })
        .encode();

        let gas_price = SwapFee::get_swap_fee() / SWAP_TX_GAS;

        Ok(TransactionRequest {
            from: Some(from.0),
            to: Some(ekoke_erc20.0.into()),
            value: None,
            gas: Some(SWAP_TX_GAS.into()),
            gas_price: Some(gas_price.into()),
            data: Some(Bytes::from(payload)),
            nonce: Some(nonce.into()),
            chain_id: Some(Configuration::get_eth_chain_id().into()),
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
        }
    }
}
