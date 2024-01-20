mod ic_cache;
mod xrc;

use std::cell::RefCell;
use std::collections::HashMap;

use candid::Principal;
use did::marketplace::MarketplaceResult;

use self::ic_cache::IcCache;
use crate::constants::NANOSECONDS_IN_A_DAY;
use crate::utils::time;

thread_local! {
    static EXCHANGE_RATE: RefCell<HashMap<String, IcCache<Rate>>> = RefCell::new(HashMap::default());
}

pub struct ExchangeRate;

#[derive(Debug, Copy, Clone)]
pub struct Rate {
    rate: u64,
    decimals: u32,
}

impl ExchangeRate {
    pub async fn get_rate(xrc_principal: Principal, currency: &str) -> MarketplaceResult<Rate> {
        let rate = EXCHANGE_RATE
            .with_borrow(|rates| rates.get(currency).and_then(|rate| rate.get().cloned()));

        // if rate is expired or not found, fetch it from the XRC
        let rate = if let Some(rate) = rate {
            rate
        } else {
            let rate = xrc::Xrc::get_rate(xrc_principal, currency).await?;
            // expires at the end of the day
            let expiration = time() - (time() % NANOSECONDS_IN_A_DAY) + NANOSECONDS_IN_A_DAY;

            EXCHANGE_RATE.with_borrow_mut(|rates| {
                rates.insert(currency.to_string(), IcCache::new(rate, expiration));
            });

            rate
        };

        Ok(rate)
    }
}

impl Rate {
    /// Converts a the provided amount to a token value using the rate and decimals.
    pub fn convert(&self, amount: u64) -> u64 {
        let amount = amount as f64;
        let rate = self.rate as f64;
        let decimals = self.decimals as f64;

        (((amount * 10_f64.powf(decimals)) / rate) * 10_f64.powf(decimals)).round() as u64
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[tokio::test]
    async fn test_should_get_rate() {
        let rate = ExchangeRate::get_rate(Principal::anonymous(), "EUR")
            .await
            .unwrap();
        assert_eq!(rate.rate, 813000000);
        assert_eq!(rate.decimals, 8);
    }
}
