//! Types associated to the "Ekoke" canister

mod error;
mod eth_network;
mod liquidity_pool;
mod role;

use candid::{CandidType, Deserialize, Nat, Principal};
use icrc::icrc1::account::Account;

pub use self::error::{
    AllowanceError, BalanceError, ConfigurationError, EcdsaError, EkokeError, PoolError,
    RegisterError,
};
pub use self::eth_network::EthNetwork;
pub use self::liquidity_pool::{LiquidityPoolAccounts, LiquidityPoolBalance};
pub use self::role::{Role, Roles};
use crate::H160;

pub type EkokeResult<T> = Result<T, EkokeError>;

/// 0.000000000001 $ekoke
pub type PicoEkoke = Nat;

/// These are the arguments which are taken by the ekoke canister on init
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct EkokeInitData {
    pub admins: Vec<Principal>,
    /// The canister ID of the CKBTC canister
    pub ckbtc_canister: Principal,
    /// The canister ID of the CKETH ledger canister
    pub cketh_ledger_canister: Principal,
    /// The canister ID of the CKETH minter canister
    pub cketh_minter_canister: Principal,
    /// The Ethereum address of the ERC20 bridge
    pub erc20_bridge_address: H160,
    /// Initial ERC20 swap fee
    pub erc20_gas_price: u64,
    /// The Ethereum network
    pub erc20_network: EthNetwork,
    /// Total supply of $picoekoke tokens
    pub total_supply: PicoEkoke,
    /// Initial balances (wallet subaccount -> picoekoke)
    pub initial_balances: Vec<(Account, PicoEkoke)>,
    /// The canister ID of the EKOKE archive canister
    pub archive_canister: Principal,
    /// Deferred canister
    pub deferred_canister: Principal,
    /// ICP ledger canister
    pub icp_ledger_canister: Principal,
    /// Marketplace canister
    pub marketplace_canister: Principal,
    /// Swap account
    pub swap_account: Account,
    /// Minting account, the account that can mint new tokens and burn them
    pub minting_account: Account,
    /// XRC canister
    pub xrc_canister: Principal,
}
