use candid::Principal;
use did::ekoke_liquidity_pool::LiquidityPoolAccounts;
#[cfg(target_arch = "wasm32")]
use did::marketplace::MarketplaceError;
use did::marketplace::MarketplaceResult;

#[allow(dead_code)]
pub struct EkokeLiquidityPoolClient {
    ekoke_liquidity_pool_canister: Principal,
}

impl From<Principal> for EkokeLiquidityPoolClient {
    fn from(ekoke_liquidity_pool_canister: Principal) -> Self {
        Self {
            ekoke_liquidity_pool_canister,
        }
    }
}

impl EkokeLiquidityPoolClient {
    /// Returns the liquidity pool accounts.
    #[allow(unused_variables)]
    pub async fn liquidity_pool_accounts(&self) -> MarketplaceResult<LiquidityPoolAccounts> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(LiquidityPoolAccounts {
                icp: icrc::icrc1::account::Account {
                    owner: Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap(),
                    subaccount: None,
                },
                ckbtc: icrc::icrc1::account::Account {
                    owner: Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap(),
                    subaccount: None,
                },
            })
        }
        #[cfg(target_arch = "wasm32")]
        {
            let result: (LiquidityPoolAccounts,) = ic_cdk::api::call::call(
                self.ekoke_liquidity_pool_canister,
                "liquidity_pool_accounts",
                (),
            )
            .await
            .map_err(|(code, err)| MarketplaceError::CanisterCall(code, err))?;
            Ok(result.0)
        }
    }
}
