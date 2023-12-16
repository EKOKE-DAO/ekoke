use candid::Nat;
#[cfg(target_arch = "wasm32")]
use candid::Principal;
use did::fly::FlyResult;
use icrc::icrc1::account::Account;
use icrc::icrc2;

/// CKBTC ledger canister client
pub struct CkBtc;

impl CkBtc {
    /// Get account balance
    #[allow(unused_variables)]
    pub async fn icrc1_balance_of(account: Account) -> FlyResult<Nat> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(88_378_u64.into())
        }
        #[cfg(target_arch = "wasm32")]
        {
            let result: (Nat,) = ic_cdk::call(
                Self::ckbtc_ledger_canister(),
                "icrc1_balance_of",
                (account,),
            )
            .await
            .map_err(|(code, err)| did::fly::FlyError::CanisterCall(code, err))?;

            Ok(result.0)
        }
    }

    /// Get CKBTC allowance for account
    #[allow(unused_variables)]
    pub async fn icrc2_allowance(
        spender: Account,
        account: Account,
    ) -> FlyResult<icrc2::allowance::Allowance> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(icrc2::allowance::Allowance {
                allowance: 5_000_000.into(),
                expires_at: None,
            })
        }
        #[cfg(target_arch = "wasm32")]
        {
            let args = icrc2::allowance::AllowanceArgs { spender, account };
            let allowance: (icrc2::allowance::Allowance,) =
                ic_cdk::call(Self::ckbtc_ledger_canister(), "icrc2_allowance", (args,))
                    .await
                    .map_err(|(code, err)| did::fly::FlyError::CanisterCall(code, err))?;

            Ok(allowance.0)
        }
    }

    /// Transfer CKBTC from `from` account to `to` account
    #[allow(unused_variables)]
    pub async fn icrc2_transfer_from(
        spender_subaccount: Option<[u8; 32]>,
        from: Account,
        to: Account,
        amount: Nat,
    ) -> FlyResult<Nat> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(amount)
        }
        #[cfg(target_arch = "wasm32")]
        {
            let args = icrc2::transfer_from::TransferFromArgs {
                spender_subaccount,
                from,
                to,
                amount,
                fee: None,
                memo: None,
                created_at_time: None,
            };
            let result: (Result<Nat, icrc2::transfer_from::TransferFromError>,) = ic_cdk::call(
                Self::ckbtc_ledger_canister(),
                "icrc2_transfer_from",
                (args,),
            )
            .await
            .map_err(|(code, err)| did::fly::FlyError::CanisterCall(code, err))?;

            Ok(result.0?)
        }
    }

    #[inline]
    #[cfg(target_arch = "wasm32")]
    fn ckbtc_ledger_canister() -> Principal {
        Principal::from_text(crate::constants::CKBTC_LEDGER_CANISTER).unwrap()
    }
}
