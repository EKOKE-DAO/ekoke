//! Types associated to the "Ekoke" canister

mod liquidity_pool;

use candid::{CandidType, Deserialize, Principal};
use icrc::icrc1::account::Account;

pub use self::liquidity_pool::{LiquidityPoolAccounts, LiquidityPoolBalance};

/// These are the arguments which are taken by the ekoke liquidity pool canister on init
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct EkokeLiquidityPoolInitData {
    pub admins: Vec<Principal>,
    /// The canister ID of the CKBTC canister
    pub ckbtc_canister: Principal,
    /// ICP ledger canister id
    pub icp_ledger_canister: Principal,
    /// Swap account
    pub swap_account: Account,
    /// XRC canister
    pub xrc_canister: Principal,
}
