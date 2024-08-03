use candid::{CandidType, Deserialize, Nat};
use icrc::icrc1::account::Account;
use serde::Serialize;

/// The accounts that hold the liquidity pools for the CKBTC and ICP tokens.
#[derive(Clone, Debug, PartialEq, Eq, CandidType, Deserialize, Serialize)]
pub struct LiquidityPoolAccounts {
    /// The account that holds the pool for the ICP tokens.
    pub icp: Account,
}

/// The balance of the liquidity pool
#[derive(Clone, Debug, PartialEq, Eq, CandidType, Deserialize, Serialize)]
pub struct LiquidityPoolBalance {
    /// ICP tokens hold in the liquidity pool
    pub icp: Nat,
}
