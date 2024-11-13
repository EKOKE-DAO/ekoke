use std::path::Path;

use candid::{CandidType, Nat, Principal};
use icrc::{icrc::generic_metadata_value::MetadataValue, icrc1::account::Account};
use serde::Deserialize;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct Icrc2TemplateInitArgs {
    pub accounts: Vec<(Account, Nat)>,
    pub decimals: u8,
    pub fee: u64,
    pub logo: String,
    pub minting_account: Account,
    pub name: String,
    pub symbol: String,
    pub total_supply: Nat,
}

#[derive(Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct IcrcLedgerInitArgs {
    pub minting_account: Account,
    pub fee_collector_account: Option<Account>,
    pub initial_balances: Vec<(Account, Nat)>,
    pub transfer_fee: Nat,
    pub decimals: Option<u8>,
    pub token_name: String,
    pub token_symbol: String,
    pub metadata: Vec<(String, MetadataValue)>,
    pub archive_options: ArchiveOptions,
    pub max_memo_length: Option<u16>,
    pub feature_flags: Option<FeatureFlags>,
    pub maximum_number_of_accounts: Option<u64>,
    pub accounts_overflow_trim_quantity: Option<u64>,
}

#[derive(Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub struct ArchiveOptions {
    /// The number of blocks which, when exceeded, will trigger an archiving
    /// operation
    pub trigger_threshold: usize,
    /// The number of blocks to archive when trigger threshold is exceeded
    pub num_blocks_to_archive: usize,
    pub node_max_memory_size_bytes: Option<u64>,
    pub max_message_size_bytes: Option<u64>,
    pub controller_id: Principal,
    // cycles to use for the call to create a new archive canister
    #[serde(default)]
    pub cycles_for_archive_creation: Option<u64>,
    // Max transactions returned by the [get_transactions] endpoint
    #[serde(default)]
    pub max_transactions_per_response: Option<u64>,
}

#[derive(CandidType, Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct FeatureFlags {
    pub icrc2: bool,
}

#[derive(Deserialize, CandidType, Clone, Debug, PartialEq, Eq)]
pub enum IcrcLedgerArgs {
    Init(IcrcLedgerInitArgs),
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct IcrcIndexInitArgs {
    pub ledger_id: Principal,
}

pub enum Canister {
    Deferred,
    EkokeErc20Swap,
    EkokeIcrcIndex,
    EkokeIcrcLedger,
    EkokeLiquidityPool,
    EkokeRewardPool,
    Icrc2Template,
    Marketplace,
    Xrc,
}

impl Canister {
    pub fn as_path(&self) -> &'static Path {
        match self {
            Canister::Deferred => Path::new("../.dfx/local/canisters/deferred/deferred.wasm"),
            Canister::EkokeErc20Swap => {
                Path::new("../.dfx/local/canisters/ekoke-erc20-swap/ekoke-erc20-swap.wasm")
            }
            Canister::EkokeIcrcIndex => Path::new("../assets/wasm/ekoke-icrc-index.wasm.gz"),
            Canister::EkokeIcrcLedger => Path::new("../assets/wasm/ekoke-icrc-ledger.wasm.gz"),
            Canister::EkokeLiquidityPool => {
                Path::new("../.dfx/local/canisters/ekoke-liquidity-pool/ekoke-liquidity-pool.wasm")
            }
            Canister::EkokeRewardPool => {
                Path::new("../.dfx/local/canisters/ekoke-reward-pool/ekoke-reward-pool.wasm")
            }
            Canister::Marketplace => {
                Path::new("../.dfx/local/canisters/marketplace/marketplace.wasm")
            }
            Canister::Xrc => Path::new("../assets/wasm/xrc-dummy-canister.wasm"),
            Canister::Icrc2Template => Path::new("../assets/wasm/icrc2-template-canister.wasm"),
        }
    }
}
