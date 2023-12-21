use candid::Principal;
#[cfg(target_arch = "wasm32")]
use did::fly::FlyResult;
use did::fly::{LiquidityPoolAccounts, PicoFly};
#[cfg(target_arch = "wasm32")]
use did::marketplace::MarketplaceError;
use did::marketplace::MarketplaceResult;
use did::ID;
use icrc::icrc1::account::Account;

#[allow(dead_code)]
pub struct FlyClient {
    fly_canister: Principal,
}

impl From<Principal> for FlyClient {
    fn from(fly_canister: Principal) -> Self {
        Self { fly_canister }
    }
}

impl FlyClient {
    /// Returns the liquidity pool accounts.
    #[allow(unused_variables)]
    pub async fn liquidity_pool_accounts(&self) -> MarketplaceResult<LiquidityPoolAccounts> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(LiquidityPoolAccounts {
                icp: Account {
                    owner: Principal::anonymous(),
                    subaccount: None,
                },
                ckbtc: Account {
                    owner: Principal::anonymous(),
                    subaccount: None,
                },
            })
        }
        #[cfg(target_arch = "wasm32")]
        {
            let result: (FlyResult<LiquidityPoolAccounts>,) =
                ic_cdk::api::call::call(self.fly_canister, "liquidity_pool_accounts", ())
                    .await
                    .map_err(|(code, err)| MarketplaceError::CanisterCall(code, err))?;
            Ok(result.0?)
        }
    }

    #[allow(unused_variables)]
    pub async fn send_reward(
        &self,
        contract_id: &ID,
        amount: PicoFly,
        account: Account,
    ) -> MarketplaceResult<()> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(())
        }
        #[cfg(target_arch = "wasm32")]
        {
            let result: (FlyResult<()>,) = ic_cdk::api::call::call(
                self.fly_canister,
                "send_reward",
                (contract_id, amount, account),
            )
            .await
            .map_err(|(code, err)| MarketplaceError::CanisterCall(code, err))?;
            Ok(result.0?)
        }
    }
}
