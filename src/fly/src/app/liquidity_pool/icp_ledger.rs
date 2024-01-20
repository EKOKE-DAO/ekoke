use candid::Nat;
#[cfg(target_arch = "wasm32")]
use candid::{CandidType, Principal};
use did::fly::FlyResult;
#[cfg(target_arch = "wasm32")]
use icrc::icrc1;
#[cfg(target_arch = "wasm32")]
use serde::Deserialize;

use crate::Account;

/// ICP ledger canister client
pub struct IcpLedger;

impl IcpLedger {
    /// Get account balance
    #[allow(unused_variables)]
    pub async fn icrc1_balance_of(account: Account) -> FlyResult<Nat> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(1_216_794_022.into())
        }
        #[cfg(target_arch = "wasm32")]
        {
            let result: (Nat,) =
                ic_cdk::call(Self::icp_ledger_canister(), "icrc1_balance_of", (account,))
                    .await
                    .map_err(|(code, err)| did::fly::FlyError::CanisterCall(code, err))?;

            Ok(result.0)
        }
    }

    /// Transfer ICP from `from` account to `to` account
    #[allow(unused_variables)]
    pub async fn icrc1_transfer(to: Account, amount: Nat) -> FlyResult<Nat> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(amount)
        }
        #[cfg(target_arch = "wasm32")]
        {
            let args = icrc1::transfer::TransferArg {
                to,
                from_subaccount: None,
                fee: None,
                created_at_time: None,
                memo: None,
                amount,
            };
            let result: (Result<Nat, icrc1::transfer::TransferError>,) =
                ic_cdk::call(Self::icp_ledger_canister(), "icrc1_transfer", (args,))
                    .await
                    .map_err(|(code, err)| did::fly::FlyError::CanisterCall(code, err))?;

            Ok(result.0?)
        }
    }

    #[inline]
    #[cfg(target_arch = "wasm32")]
    fn icp_ledger_canister() -> Principal {
        crate::app::Configuration::get_icp_ledger_canister()
    }
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug, CandidType, Deserialize)]
struct AccountIdentifierRequest {
    owner: Principal,
    subaccount: Option<Vec<u8>>,
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug, CandidType, Deserialize)]
struct AccountBalanceRequest {
    account: Vec<u8>,
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug, CandidType, Deserialize)]
struct AccountBalanceResponse {
    e8s: Nat,
}
