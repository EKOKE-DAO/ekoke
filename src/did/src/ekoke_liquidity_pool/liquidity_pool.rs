use candid::{CandidType, Deserialize, Nat, Principal};
use ic_cdk::api::call::RejectionCode;
use icrc::icrc1::account::Account;
use icrc::icrc1::transfer::TransferError;
use serde::Serialize;
use thiserror::Error;

/// Withdraw refund error
#[derive(Clone, Debug, Error, CandidType, PartialEq, Eq, Deserialize)]
pub enum WithdrawError {
    #[error("Principal {0} has nothing to withdraw.")]
    NothingToWithdraw(Principal),
    #[error("icrc transfer error: {0}")]
    Transfer(TransferError),
    #[error("inter-canister call error: ({0:?}): {1}")]
    CanisterCall(RejectionCode, String),
}

impl From<TransferError> for WithdrawError {
    fn from(err: TransferError) -> Self {
        WithdrawError::Transfer(err)
    }
}

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
