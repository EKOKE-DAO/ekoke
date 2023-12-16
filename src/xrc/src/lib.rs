//! # XRC
//!
//! Exchange rate canister client

use candid::Principal;
use ic_cdk::api::call::RejectionCode;
pub use ic_xrc_types::{Asset, AssetClass, ExchangeRateError};
use ic_xrc_types::{GetExchangeRateRequest, GetExchangeRateResult};

/// Client for the XRC canister
pub struct XrcClient {
    principal: Principal,
}

impl Default for XrcClient {
    fn default() -> Self {
        Self {
            principal: Principal::from_text("uf6dk-hyaaa-aaaaq-qaaaq-cai").unwrap(),
        }
    }
}

impl XrcClient {
    pub fn new(principal: Principal) -> Self {
        Self { principal }
    }

    /// Get the exchange rate for the given asset pair
    pub async fn get_exchange_rate(
        &self,
        base_asset: Asset,
        quote_asset: Asset,
    ) -> Result<GetExchangeRateResult, (RejectionCode, String)> {
        let request = GetExchangeRateRequest {
            base_asset,
            quote_asset,
            timestamp: None,
        };
        let result: (GetExchangeRateResult,) =
            ic_cdk::call(self.principal, "get_exchange_rate", (request,)).await?;

        Ok(result.0)
    }
}
