use candid::Principal;
#[cfg(target_arch = "wasm32")]
use did::marketplace::MarketplaceError;
use did::marketplace::MarketplaceResult;
#[cfg(target_arch = "wasm32")]
use xrc::{Asset, AssetClass, XrcClient};

use super::Rate;

pub struct Xrc;

impl Xrc {
    /// Get the ICP to provided currency rate
    /// Then you can convert currency to ICP with:
    /// ICP = value * ExchangeRate
    #[allow(unused_variables)]
    pub async fn get_rate(xrc_principal: Principal, currency: &str) -> MarketplaceResult<Rate> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(Rate {
                rate: 813000000,
                decimals: 8,
            })
        }
        #[cfg(target_arch = "wasm32")]
        {
            let xrc = XrcClient::new(xrc_principal);

            // Base is the one on the left {currency}/ICP
            let base_asset = Asset {
                symbol: currency.to_string(),
                class: AssetClass::FiatCurrency,
            };
            let quote_asset = Asset {
                symbol: "ICP".to_string(),
                class: AssetClass::Cryptocurrency,
            };

            match xrc
                .get_exchange_rate(base_asset, quote_asset)
                .await
                .map_err(|(rc, m)| MarketplaceError::CanisterCall(rc, m))
            {
                Err(err) => Err(err),
                Ok(Ok(exchange_rate)) => Ok(Rate {
                    rate: exchange_rate.rate,
                    decimals: exchange_rate.metadata.decimals,
                }),
                Ok(Err(e)) => Err(e.into()),
            }
        }
    }
}
