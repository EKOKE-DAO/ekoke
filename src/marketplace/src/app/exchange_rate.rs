mod ic_cache;
mod xrc;

use std::cell::RefCell;
use std::collections::HashMap;

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
    pub async fn get_rate(currency: &str) -> MarketplaceResult<Rate> {
        let rate = EXCHANGE_RATE
            .with_borrow(|rates| rates.get(currency).and_then(|rate| rate.get().cloned()));

        // if rate is expired or not found, fetch it from the XRC
        let rate = if let Some(rate) = rate {
            rate
        } else {
            let rate = xrc::Xrc::get_rate(currency).await?;
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

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[tokio::test]
    async fn test_should_get_rate() {
        let rate = ExchangeRate::get_rate("EUR").await.unwrap();
        assert_eq!(rate.rate, 813000000);
        assert_eq!(rate.decimals, 8);
    }
}
