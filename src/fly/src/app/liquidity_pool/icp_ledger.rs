#[cfg(target_arch = "wasm32")]
use candid::CandidType;
use candid::{Nat, Principal};
use did::fly::FlyResult;
#[cfg(target_arch = "wasm32")]
use icrc::icrc1;
#[cfg(target_arch = "wasm32")]
use serde::Deserialize;

use crate::Account;

/// ICP ledger canister client
pub struct IcpLedger;

impl IcpLedger {
    /// Get account identifier
    #[allow(unused_variables)]
    pub async fn account_identifier(id: Principal, subaccount: Option<Vec<u8>>) -> Vec<u8> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            vec![
                0x44, 0x49, 0x44, 0x4c, 0x01, 0x6d, 0x7b, 0x01, 0x00, 0x20, 0x21, 0xa9, 0x95, 0x49,
                0xe7, 0x92, 0x90, 0x7c, 0x5e, 0x27, 0x5e, 0x54, 0x51, 0x06, 0x8d, 0x4d, 0xdf, 0x4d,
                0x43, 0xee, 0x8d, 0xca, 0xb4, 0x87, 0x56, 0x23, 0x1a, 0x8f, 0xb7, 0x71, 0x31, 0x23,
            ]
        }
        #[cfg(target_arch = "wasm32")]
        {
            let result: (Vec<u8>,) = ic_cdk::call(
                Self::icp_ledger_canister(),
                "account_identifier",
                (AccountIdentifierRequest {
                    owner: id,
                    subaccount,
                },),
            )
            .await
            .unwrap();

            result.0
        }
    }

    /// Get account balance
    #[allow(unused_variables)]
    pub async fn account_balance(account: Vec<u8>) -> FlyResult<Nat> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(1_216_794_022.into())
        }
        #[cfg(target_arch = "wasm32")]
        {
            let result: (AccountBalanceResponse,) = ic_cdk::call(
                Self::icp_ledger_canister(),
                "account_balance",
                (AccountBalanceRequest { account },),
            )
            .await
            .map_err(|(code, err)| did::fly::FlyError::CanisterCall(code, err))?;

            Ok(result.0.e8s)
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
        Principal::from_text(crate::constants::ICP_LEDGER_CANISTER).unwrap()
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
