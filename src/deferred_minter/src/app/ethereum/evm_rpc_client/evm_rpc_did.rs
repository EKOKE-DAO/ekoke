#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::enum_variant_names)]

use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(CandidType, Serialize, Deserialize)]
pub enum EthMainnetService {
    Alchemy,
    Llama,
    BlockPi,
    Cloudflare,
    PublicNode,
    Ankr,
}

#[derive(CandidType, Serialize, Deserialize)]
pub enum EthSepoliaService {
    Alchemy,
    BlockPi,
    PublicNode,
    Ankr,
    Sepolia,
}

#[derive(CandidType, Deserialize, Serialize)]
pub enum ConsensusStrategy {
    Equality,
    Threshold { min: u8, total: Option<u8> },
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct RpcConfig {
    #[allow(non_snake_case)]
    pub responseConsensus: Option<ConsensusStrategy>,
    #[allow(non_snake_case)]
    pub responseSizeEstimate: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct RpcApi {
    pub url: String,
    pub headers: Option<Vec<HttpHeader>>,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct HttpHeader {
    pub value: String,
    pub name: String,
}

#[derive(CandidType, Serialize)]
pub enum RpcServices {
    EthSepolia(Option<Vec<EthSepoliaService>>),
    Custom {
        #[allow(non_snake_case)]
        chainId: u64,
        services: Vec<RpcApi>,
    },
    EthMainnet(Option<Vec<EthMainnetService>>),
}

#[derive(CandidType, Deserialize, Debug)]
pub enum SendRawTransactionStatus {
    Ok(Option<String>),
    NonceTooLow,
    NonceTooHigh,
    InsufficientFunds,
}

#[derive(CandidType, Deserialize)]
pub enum SendRawTransactionResult {
    Ok(SendRawTransactionStatus),
    Err(RpcError),
}

#[derive(CandidType, Deserialize, Debug)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum ProviderError {
    TooFewCycles {
        expected: candid::Nat,
        received: candid::Nat,
    },
    InvalidRpcConfig(String),
    MissingRequiredProvider,
    ProviderNotFound,
    NoPermission,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum ValidationError {
    Custom(String),
    InvalidHex(String),
}

#[derive(CandidType, Deserialize, Debug)]
pub enum RejectionCode {
    NoError,
    CanisterError,
    SysTransient,
    DestinationInvalid,
    Unknown,
    SysFatal,
    CanisterReject,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum RpcError {
    JsonRpcError(JsonRpcError),
    ProviderError(ProviderError),
    ValidationError(ValidationError),
    HttpOutcallError(HttpOutcallError),
}

#[derive(CandidType, Deserialize, Debug)]
pub enum HttpOutcallError {
    IcError {
        code: RejectionCode,
        message: String,
    },
    InvalidHttpJsonRpcResponse {
        status: u16,
        body: String,
        #[allow(non_snake_case)]
        parsingError: Option<String>,
    },
}

#[derive(CandidType, Deserialize)]
pub enum L2MainnetService {
    Alchemy,
    Llama,
    BlockPi,
    PublicNode,
    Ankr,
}

pub type ChainId = u64;
pub type ProviderId = u64;

#[derive(CandidType, Deserialize)]
pub enum RpcService {
    EthSepolia(EthSepoliaService),
    BaseMainnet(L2MainnetService),
    Custom(RpcApi),
    OptimismMainnet(L2MainnetService),
    ArbitrumOne(L2MainnetService),
    EthMainnet(EthMainnetService),
    Provider(ProviderId),
}

#[derive(CandidType, Deserialize)]
pub enum MultiSendRawTransactionResult {
    Consistent(SendRawTransactionResult),
    Inconsistent(Vec<(RpcService, SendRawTransactionResult)>),
}

#[derive(CandidType, Serialize)]
pub struct GetTransactionCountArgs {
    pub address: String,
    pub block: BlockTag,
}

#[derive(CandidType, Serialize)]
pub enum BlockTag {
    Earliest,
    Safe,
    Finalized,
    Latest,
    Number(candid::Nat),
    Pending,
}

#[derive(CandidType, Deserialize)]
pub enum GetTransactionCountResult {
    Ok(candid::Nat),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiGetTransactionCountResult {
    Consistent(GetTransactionCountResult),
    Inconsistent(Vec<(RpcService, GetTransactionCountResult)>),
}

#[derive(CandidType, Serialize)]
pub struct CallArgs {
    pub transaction: TransactionRequest,
    pub block: Option<BlockTag>,
}

#[allow(non_snake_case)]
#[derive(Default, CandidType, Serialize)]
pub struct TransactionRequest {
    pub to: Option<String>,
    pub gas: Option<candid::Nat>,
    pub maxFeePerGas: Option<candid::Nat>,
    pub gasPrice: Option<candid::Nat>,
    pub value: Option<candid::Nat>,
    pub maxFeePerBlobGas: Option<candid::Nat>,
    pub from: Option<String>,
    pub r#type: Option<String>,
    pub accessList: Option<Vec<AccessListEntry>>,
    pub nonce: Option<candid::Nat>,
    pub maxPriorityFeePerGas: Option<candid::Nat>,
    pub blobs: Option<Vec<String>>,
    pub input: Option<String>,
    pub chainId: Option<candid::Nat>,
    pub blobVersionedHashes: Option<Vec<String>>,
}

#[derive(CandidType, Serialize)]
pub struct AccessListEntry {
    #[allow(non_snake_case)]
    pub storageKeys: Vec<String>,
    pub address: String,
}

#[derive(CandidType, Deserialize)]
pub enum CallResult {
    Ok(String),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiCallResult {
    Consistent(CallResult),
    Inconsistent(Vec<(RpcService, CallResult)>),
}
