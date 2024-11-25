mod evm_rpc_did;

use candid::Principal;
use did::deferred::{DeferredMinterError, DeferredMinterResult};
use did::H160;
use ethers_core::types::{Bytes, U256};
use evm_rpc_did::{
    BlockTag, CallArgs, CallResult, EthMainnetService, EthSepoliaService, GetTransactionCountArgs,
    GetTransactionCountResult, MultiCallResult, MultiGetTransactionCountResult, RpcConfig,
    RpcError, RpcService, SendRawTransactionResult, SendRawTransactionStatus, TransactionRequest,
};
use num_traits::cast::ToPrimitive;

use self::evm_rpc_did::{MultiSendRawTransactionResult, RpcApi, RpcServices};

const MAINNET_CHAIN_ID: u64 = 1;
const SEPOLIA_CHAIN_ID: u64 = 11155111;
const GET_NEXT_NONCE_SAMPLE_PAYLOAD: &str = r#"{"jsonrpc":"2.0","id":1,"method":"eth_getTransactionCount","params":["0xBf380C52C18d5ead99ea719b6FCfbbA551Df2F7F", "pending"]}"#;

pub struct EvmRpcClient {
    chain_id: u64,
    custom_rpc: Option<String>,
    principal: Principal,
}

impl EvmRpcClient {
    pub fn new(principal: Principal, chain_id: u64, custom_rpc: Option<String>) -> Self {
        Self {
            principal,
            chain_id,
            custom_rpc,
        }
    }

    /// Get next nonce for the given address
    pub async fn get_next_nonce(&self, address: H160) -> DeferredMinterResult<U256> {
        if cfg!(test) {
            return Ok(U256::zero());
        }

        let services = self.services();
        let rpc_config: Option<RpcConfig> = None;
        let args = GetTransactionCountArgs {
            address: address.to_hex_str(),
            block: BlockTag::Pending,
        };

        let cycles_cost = self.get_request_cost(GET_NEXT_NONCE_SAMPLE_PAYLOAD).await?;
        log::debug!("estimated cost for get next nonce: {cycles_cost}",);

        // send effective request
        let (result,) = ic_cdk::api::call::call::<_, (MultiGetTransactionCountResult,)>(
            self.principal,
            "eth_getTransactionCount",
            (services, rpc_config, args),
        )
        .await
        .map_err(|(code, msg)| DeferredMinterError::CanisterCall(code, msg))?;

        log::debug!("get next nonce result: {result:?}",);

        match result {
            MultiGetTransactionCountResult::Consistent(GetTransactionCountResult::Ok(nonce)) => {
                let nonce = nonce.0.to_u128().expect("Nonce is too large");
                Ok(U256::from(nonce))
            }
            MultiGetTransactionCountResult::Consistent(GetTransactionCountResult::Err(err)) => Err(
                DeferredMinterError::EvmRpc(format!("Failed to get nonce: {:?}", err)),
            ),
            MultiGetTransactionCountResult::Inconsistent(_) => Err(DeferredMinterError::EvmRpc(
                "Failed to get nonce with inconsistent result".to_string(),
            )),
        }
    }

    /// Call contract function
    pub async fn eth_call(&self, to: &H160, data: Bytes) -> DeferredMinterResult<String> {
        if cfg!(test) {
            return Ok(
                "0000000000000000000000000000000000000000000000000000000000003039".to_string(),
            );
        }

        let services = self.services();
        let rpc_config: Option<RpcConfig> = None;
        let data = data.to_string();

        let request_as_str = format!(
            r#"{{"jsonrpc":"2.0","id":1,"method":"eth_call","params":[{{"to":"{}","data":"{}"}},"latest"]}}"#,
            to.to_hex_str(),
            data
        );

        let cycles_cost = self.get_request_cost(&request_as_str).await?;
        log::debug!("estimated cost for eth call: {cycles_cost}",);

        let (result,) = ic_cdk::api::call::call_with_payment128::<_, (MultiCallResult,)>(
            self.principal,
            "eth_call",
            (
                services,
                rpc_config,
                CallArgs {
                    transaction: TransactionRequest {
                        to: Some(to.to_hex_str()),
                        input: Some(data),
                        ..Default::default()
                    },
                    block: Some(BlockTag::Latest),
                },
            ),
            cycles_cost,
        )
        .await
        .map_err(|(code, msg)| DeferredMinterError::CanisterCall(code, msg))?;

        log::debug!("eth call result: {result:?}",);

        match result {
            MultiCallResult::Consistent(call_result) => match call_result {
                CallResult::Ok(result) => Ok(result),
                CallResult::Err(err) => Err(DeferredMinterError::EvmRpc(format!(
                    "Failed to call contract: {:?}",
                    err
                ))),
            },
            MultiCallResult::Inconsistent(_) => Err(DeferredMinterError::EvmRpc(
                "Failed to call contract with inconsistent result".to_string(),
            )),
        }
    }

    /// Send raw transaction to Ethereum network
    pub async fn eth_send_raw_transaction(&self, tx: Bytes) -> DeferredMinterResult<()> {
        if cfg!(test) {
            return Ok(());
        }

        let services = self.services();
        let rpc_config: Option<RpcConfig> = None;
        let tx = tx.to_string();

        let request_as_str = format!(
            r#"{{"jsonrpc":"2.0","id":1,"method":"eth_sendRawTransaction","params":["{}"]}}"#,
            tx
        );

        let cycles_cost = self.get_request_cost(&request_as_str).await?;
        log::debug!("estimated cost for send raw transaction: {cycles_cost}",);

        let result =
            ic_cdk::api::call::call_with_payment128::<_, (MultiSendRawTransactionResult,)>(
                self.principal,
                "eth_sendRawTransaction",
                (services, rpc_config, tx),
                cycles_cost,
            )
            .await
            .map_err(|(code, msg)| DeferredMinterError::CanisterCall(code, msg))?
            .0;

        log::debug!("send raw transaction result: {result:?}",);

        match result {
            MultiSendRawTransactionResult::Consistent(SendRawTransactionResult::Ok(
                SendRawTransactionStatus::Ok(_),
            )) => Ok(()),
            MultiSendRawTransactionResult::Consistent(SendRawTransactionResult::Ok(status)) => Err(
                DeferredMinterError::EvmRpc(format!("Transaction failed with status: {status:?}",)),
            ),
            MultiSendRawTransactionResult::Consistent(SendRawTransactionResult::Err(err)) => Err(
                DeferredMinterError::EvmRpc(format!("Transaction failed with error: {:?}", err)),
            ),
            MultiSendRawTransactionResult::Inconsistent(_) => Err(DeferredMinterError::EvmRpc(
                "Transaction failed with inconsistent result".to_string(),
            )),
        }
    }

    /// Estimate request cost
    async fn get_request_cost(&self, request: &str) -> DeferredMinterResult<u128> {
        let trimmed_request = &request[..std::cmp::min(request.len(), 256)];

        log::info!("getting request cost for {trimmed_request}",);
        let services = self.service();
        // estimate cycles
        let (cycles_result,) = ic_cdk::api::call::call::<_, (Result<u128, RpcError>,)>(
            self.principal,
            "requestCost",
            (services, request.to_string(), 1024u64),
        )
        .await
        .map_err(|(code, msg)| DeferredMinterError::CanisterCall(code, msg))?;

        match cycles_result {
            Ok(cycles) => Ok(cycles),
            Err(err) => Err(DeferredMinterError::EvmRpc(format!(
                "Failed to estimate cycles: {:?}",
                err
            ))),
        }
    }

    #[inline]
    fn service(&self) -> RpcService {
        if let Some(url) = self.custom_rpc.as_deref() {
            return RpcService::Custom(RpcApi {
                url: url.to_string(),
                headers: None,
            });
        }

        match self.chain_id {
            MAINNET_CHAIN_ID => RpcService::EthMainnet(EthMainnetService::Cloudflare),
            SEPOLIA_CHAIN_ID => RpcService::EthSepolia(EthSepoliaService::Sepolia),
            _ => ic_cdk::trap("Unsupported chain id"),
        }
    }

    #[inline]
    fn services(&self) -> RpcServices {
        if let Some(url) = self.custom_rpc.as_deref() {
            return RpcServices::Custom {
                chainId: self.chain_id,
                services: vec![RpcApi {
                    url: url.to_string(),
                    headers: None,
                }],
            };
        }

        match self.chain_id {
            MAINNET_CHAIN_ID => RpcServices::EthMainnet(None),
            SEPOLIA_CHAIN_ID => RpcServices::EthSepolia(None),
            _ => ic_cdk::trap("Unsupported chain id"),
        }
    }
}
