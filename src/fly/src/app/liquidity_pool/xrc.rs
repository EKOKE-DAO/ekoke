use candid::Principal;
#[cfg(target_arch = "wasm32")]
use did::fly::FlyError;
use did::fly::FlyResult;
#[cfg(target_arch = "wasm32")]
use xrc::{Asset, AssetClass, XrcClient};

pub struct Xrc;

impl Xrc {
    /// Get the ICP to BTC rate
    /// Then you can convert ICP to BTC with:
    /// BTC = value * ExchangeRate
    #[allow(unused_variables)]
    pub async fn get_icp_to_btc_rate(xrc_principal: Principal) -> FlyResult<f64> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(0.0002162)
        }
        #[cfg(target_arch = "wasm32")]
        {
            let xrc = XrcClient::new(xrc_principal);

            // Base is the one on the left ICP/BTC
            let base_asset = Asset {
                symbol: "ICP".to_string(),
                class: AssetClass::Cryptocurrency,
            };
            let quote_asset = Asset {
                symbol: "BTC".to_string(),
                class: AssetClass::Cryptocurrency,
            };

            match xrc
                .get_exchange_rate(base_asset, quote_asset)
                .await
                .map_err(|(rc, m)| FlyError::CanisterCall(rc, m))
            {
                Err(fly_error) => Err(fly_error),
                Ok(Ok(exchange_rate)) => {
                    let rate = exchange_rate.rate as f64;
                    let decimals = exchange_rate.metadata.decimals;
                    let rate = rate / (10_u32.pow(decimals) as f64);

                    Ok(rate)
                }
                Ok(Err(_)) => Err(FlyError::XrcError),
            }
        }
    }
}
