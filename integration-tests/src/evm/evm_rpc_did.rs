#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::large_enum_variant)]
use candid::{self, CandidType, Deserialize, Principal};

pub type Regex = String;
#[derive(CandidType, Deserialize)]
pub enum LogFilter {
    ShowAll,
    HideAll,
    ShowPattern(Regex),
    HidePattern(Regex),
}

#[derive(CandidType, Deserialize)]
pub struct InstallArgs {
    pub logFilter: Option<LogFilter>,
    pub demo: Option<bool>,
    pub manageApiKeys: Option<Vec<Principal>>,
}

#[derive(CandidType, Deserialize)]
pub enum EthSepoliaService {
    Alchemy,
    BlockPi,
    PublicNode,
    Ankr,
    Sepolia,
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
#[derive(CandidType, Deserialize)]
pub struct HttpHeader {
    pub value: String,
    pub name: String,
}

#[derive(CandidType, Deserialize)]
pub struct RpcApi {
    pub url: String,
    pub headers: Option<Vec<HttpHeader>>,
}

#[derive(CandidType, Deserialize)]
pub enum EthMainnetService {
    Alchemy,
    Llama,
    BlockPi,
    Cloudflare,
    PublicNode,
    Ankr,
}

#[derive(CandidType, Deserialize)]
pub enum RpcServices {
    EthSepolia(Option<Vec<EthSepoliaService>>),
    BaseMainnet(Option<Vec<L2MainnetService>>),
    Custom {
        chainId: ChainId,
        services: Vec<RpcApi>,
    },
    OptimismMainnet(Option<Vec<L2MainnetService>>),
    ArbitrumOne(Option<Vec<L2MainnetService>>),
    EthMainnet(Option<Vec<EthMainnetService>>),
}

#[derive(CandidType, Deserialize)]
pub enum ConsensusStrategy {
    Equality,
    Threshold { min: u8, total: Option<u8> },
}

#[derive(CandidType, Deserialize)]
pub struct RpcConfig {
    pub responseConsensus: Option<ConsensusStrategy>,
    pub responseSizeEstimate: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct AccessListEntry {
    pub storageKeys: Vec<String>,
    pub address: String,
}

#[derive(CandidType, Deserialize)]
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

#[derive(CandidType, Deserialize)]
pub enum BlockTag {
    Earliest,
    Safe,
    Finalized,
    Latest,
    Number(candid::Nat),
    Pending,
}

#[derive(CandidType, Deserialize)]
pub struct CallArgs {
    pub transaction: TransactionRequest,
    pub block: Option<BlockTag>,
}

#[derive(CandidType, Deserialize)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
}

#[derive(CandidType, Deserialize)]
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

#[derive(CandidType, Deserialize)]
pub enum ValidationError {
    Custom(String),
    InvalidHex(String),
}

#[derive(CandidType, Deserialize)]
pub enum RejectionCode {
    NoError,
    CanisterError,
    SysTransient,
    DestinationInvalid,
    Unknown,
    SysFatal,
    CanisterReject,
}

#[derive(CandidType, Deserialize)]
pub enum HttpOutcallError {
    IcError {
        code: RejectionCode,
        message: String,
    },
    InvalidHttpJsonRpcResponse {
        status: u16,
        body: String,
        parsingError: Option<String>,
    },
}

#[derive(CandidType, Deserialize)]
pub enum RpcError {
    JsonRpcError(JsonRpcError),
    ProviderError(ProviderError),
    ValidationError(ValidationError),
    HttpOutcallError(HttpOutcallError),
}

#[derive(CandidType, Deserialize)]
pub enum CallResult {
    Ok(String),
    Err(RpcError),
}

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
pub enum MultiCallResult {
    Consistent(CallResult),
    Inconsistent(Vec<(RpcService, CallResult)>),
}

#[derive(CandidType, Deserialize)]
pub struct FeeHistoryArgs {
    pub blockCount: candid::Nat,
    pub newestBlock: BlockTag,
    pub rewardPercentiles: Option<serde_bytes::ByteBuf>,
}

#[derive(CandidType, Deserialize)]
pub struct FeeHistory {
    pub reward: Vec<Vec<candid::Nat>>,
    pub gasUsedRatio: Vec<f64>,
    pub oldestBlock: candid::Nat,
    pub baseFeePerGas: Vec<candid::Nat>,
}

#[derive(CandidType, Deserialize)]
pub enum FeeHistoryResult {
    Ok(FeeHistory),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiFeeHistoryResult {
    Consistent(FeeHistoryResult),
    Inconsistent(Vec<(RpcService, FeeHistoryResult)>),
}

#[derive(CandidType, Deserialize)]
pub struct Block {
    pub miner: String,
    pub totalDifficulty: Option<candid::Nat>,
    pub receiptsRoot: String,
    pub stateRoot: String,
    pub hash: String,
    pub difficulty: Option<candid::Nat>,
    pub size: candid::Nat,
    pub uncles: Vec<String>,
    pub baseFeePerGas: Option<candid::Nat>,
    pub extraData: String,
    pub transactionsRoot: Option<String>,
    pub sha3Uncles: String,
    pub nonce: candid::Nat,
    pub number: candid::Nat,
    pub timestamp: candid::Nat,
    pub transactions: Vec<String>,
    pub gasLimit: candid::Nat,
    pub logsBloom: String,
    pub parentHash: String,
    pub gasUsed: candid::Nat,
    pub mixHash: String,
}

#[derive(CandidType, Deserialize)]
pub enum GetBlockByNumberResult {
    Ok(Block),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiGetBlockByNumberResult {
    Consistent(GetBlockByNumberResult),
    Inconsistent(Vec<(RpcService, GetBlockByNumberResult)>),
}

pub type Topic = Vec<String>;
#[derive(CandidType, Deserialize)]
pub struct GetLogsArgs {
    pub fromBlock: Option<BlockTag>,
    pub toBlock: Option<BlockTag>,
    pub addresses: Vec<String>,
    pub topics: Option<Vec<Topic>>,
}

#[derive(CandidType, Deserialize)]
pub struct LogEntry {
    pub transactionHash: Option<String>,
    pub blockNumber: Option<candid::Nat>,
    pub data: String,
    pub blockHash: Option<String>,
    pub transactionIndex: Option<candid::Nat>,
    pub topics: Vec<String>,
    pub address: String,
    pub logIndex: Option<candid::Nat>,
    pub removed: bool,
}

#[derive(CandidType, Deserialize)]
pub enum GetLogsResult {
    Ok(Vec<LogEntry>),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiGetLogsResult {
    Consistent(GetLogsResult),
    Inconsistent(Vec<(RpcService, GetLogsResult)>),
}

#[derive(CandidType, Deserialize)]
pub struct GetTransactionCountArgs {
    pub address: String,
    pub block: BlockTag,
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

#[derive(CandidType, Deserialize)]
pub struct TransactionReceipt {
    pub to: Option<String>,
    pub status: Option<candid::Nat>,
    pub transactionHash: String,
    pub blockNumber: candid::Nat,
    pub from: String,
    pub logs: Vec<LogEntry>,
    pub blockHash: String,
    pub r#type: String,
    pub transactionIndex: candid::Nat,
    pub effectiveGasPrice: candid::Nat,
    pub logsBloom: String,
    pub contractAddress: Option<String>,
    pub gasUsed: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum GetTransactionReceiptResult {
    Ok(Option<TransactionReceipt>),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiGetTransactionReceiptResult {
    Consistent(GetTransactionReceiptResult),
    Inconsistent(Vec<(RpcService, GetTransactionReceiptResult)>),
}

#[derive(CandidType, Deserialize)]
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

#[derive(CandidType, Deserialize)]
pub enum MultiSendRawTransactionResult {
    Consistent(SendRawTransactionResult),
    Inconsistent(Vec<(RpcService, SendRawTransactionResult)>),
}

#[derive(CandidType, Deserialize)]
pub struct Metrics {
    pub cyclesWithdrawn: candid::Nat,
    pub responses: Vec<((String, String, String), u64)>,
    pub errNoPermission: u64,
    pub inconsistentResponses: Vec<((String, String), u64)>,
    pub cyclesCharged: Vec<((String, String), candid::Nat)>,
    pub requests: Vec<((String, String), u64)>,
    pub errHttpOutcall: Vec<((String, String), u64)>,
    pub errHostNotAllowed: Vec<(String, u64)>,
}

#[derive(CandidType, Deserialize)]
pub enum RpcAuth {
    BearerToken { url: String },
    UrlParameter { urlPattern: String },
}

#[derive(CandidType, Deserialize)]
pub enum RpcAccess {
    Authenticated {
        publicUrl: Option<String>,
        auth: RpcAuth,
    },
    Unauthenticated {
        publicUrl: String,
    },
}

#[derive(CandidType, Deserialize)]
pub struct Provider {
    pub access: RpcAccess,
    pub alias: Option<RpcService>,
    pub chainId: ChainId,
    pub providerId: ProviderId,
}

#[derive(CandidType, Deserialize)]
pub enum RequestResult {
    Ok(String),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum RequestCostResult {
    Ok(candid::Nat),
    Err(RpcError),
}
