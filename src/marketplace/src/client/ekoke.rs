use candid::Principal;
#[cfg(target_arch = "wasm32")]
use did::ekoke::EkokeResult;
use did::ekoke::PicoEkoke;
#[cfg(target_arch = "wasm32")]
use did::marketplace::MarketplaceError;
use did::marketplace::MarketplaceResult;
use did::ID;
use icrc::icrc1::account::Account;

#[allow(dead_code)]
pub struct EkokeClient {
    ekoke_ledger_canister: Principal,
}

impl From<Principal> for EkokeClient {
    fn from(ekoke_ledger_canister: Principal) -> Self {
        Self {
            ekoke_ledger_canister,
        }
    }
}

impl EkokeClient {
    #[allow(unused_variables)]
    pub async fn send_reward(
        &self,
        contract_id: &ID,
        amount: PicoEkoke,
        account: Account,
    ) -> MarketplaceResult<()> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(())
        }
        #[cfg(target_arch = "wasm32")]
        {
            let result: (EkokeResult<()>,) = ic_cdk::api::call::call(
                self.ekoke_ledger_canister,
                "send_reward",
                (contract_id, amount, account),
            )
            .await
            .map_err(|(code, err)| MarketplaceError::CanisterCall(code, err))?;
            Ok(result.0?)
        }
    }
}
