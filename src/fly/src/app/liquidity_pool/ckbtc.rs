use candid::Nat;
#[cfg(target_arch = "wasm32")]
use candid::Principal;
use did::fly::FlyResult;
use icrc::icrc1::account::Account;

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

    #[inline]
    #[cfg(target_arch = "wasm32")]
    fn ckbtc_ledger_canister() -> Principal {
        Principal::from_text(crate::constants::CKBTC_LEDGER_CANISTER).unwrap()
    }
}
