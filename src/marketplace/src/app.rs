mod configuration;
mod exchange_rate;
mod inspect;
mod memory;
mod roles;
mod test_utils;

use candid::Principal;
use did::marketplace::{MarketplaceError, MarketplaceInitData, MarketplaceResult};
use dip721::TokenIdentifier;

use self::configuration::Configuration;
pub use self::inspect::Inspect;
use self::roles::RolesManager;
use crate::app::exchange_rate::ExchangeRate;
use crate::client::{DeferredClient, FlyClient, IcpLedgerClient};
use crate::utils::caller;

pub struct Marketplace;

impl Marketplace {
    pub fn init(data: MarketplaceInitData) {
        Configuration::set_deferred_canister(data.deferred_canister);
        Configuration::set_fly_canister(data.fly_canister);
        RolesManager::set_admins(data.admins).unwrap();
    }

    /// Sets the admins of the marketplace.
    pub fn admin_set_admins(admins: Vec<Principal>) -> MarketplaceResult<()> {
        if !Inspect::inspect_is_admin(caller()) {
            ic_cdk::trap("unauthorized");
        }
        RolesManager::set_admins(admins)
    }

    pub fn admin_set_deferred_canister(canister: Principal) {
        if !Inspect::inspect_is_admin(caller()) {
            ic_cdk::trap("unauthorized");
        }
        Configuration::set_deferred_canister(canister)
    }

    pub fn admin_set_fly_canister(canister: Principal) {
        if !Inspect::inspect_is_admin(caller()) {
            ic_cdk::trap("unauthorized");
        }
        Configuration::set_fly_canister(canister)
    }

    pub fn admin_set_interest_rate_for_buyer(interest_rate: f64) {
        if !Inspect::inspect_is_admin(caller()) {
            ic_cdk::trap("unauthorized");
        }
        if interest_rate <= 1.0 {
            ic_cdk::trap("interest rate must be greater than 1.0");
        }
        Configuration::set_interest_rate_for_buyer(interest_rate)
    }

    /// Given a token id, returns the price of the token in ICP.
    pub async fn get_token_price_icp(token_id: TokenIdentifier) -> MarketplaceResult<u64> {
        // get token info
        let deferred_client = DeferredClient::from(Configuration::get_deferred_canister());
        let token_info = match deferred_client.get_token(&token_id).await? {
            Some(token_info) => token_info,
            None => return Err(MarketplaceError::TokenNotFound),
        };
        // check if caller is a contract buyer
        let is_caller_contract_buyer = token_info.contract.buyers.contains(&caller());
        // get the price of the token in ICP
        let icp_rate = ExchangeRate::get_rate(&token_info.contract.currency).await?;
        let icp_price = icp_rate.convert(token_info.token.value);
        // multiply the price if needed by the interest rate
        Ok(if is_caller_contract_buyer {
            (icp_price as f64 * Configuration::get_interest_rate_for_buyer()).round() as u64
        } else {
            icp_price
        })
    }
}

#[cfg(test)]
mod test {

    use super::test_utils::{deferred_canister, fly_canister};
    use super::*;
    use crate::utils::caller;

    #[test]
    fn test_should_init_canister() {
        init_canister();
        assert_eq!(Configuration::get_deferred_canister(), deferred_canister());
        assert_eq!(Configuration::get_fly_canister(), fly_canister());
        assert_eq!(RolesManager::get_admins(), vec![caller()]);
    }

    #[test]
    fn test_should_change_fly_canister() {
        init_canister();
        let new_fly_canister = Principal::anonymous();
        Marketplace::admin_set_fly_canister(new_fly_canister);
        assert_eq!(Configuration::get_fly_canister(), new_fly_canister);
    }

    #[test]
    fn test_should_change_deferred_canister() {
        init_canister();
        let new_deferred_canister = Principal::anonymous();
        Marketplace::admin_set_deferred_canister(new_deferred_canister);
        assert_eq!(
            Configuration::get_deferred_canister(),
            new_deferred_canister
        );
    }

    #[test]
    fn test_should_change_interest_rate_for_buyer() {
        init_canister();
        let new_interest_rate = 1.2;
        Marketplace::admin_set_interest_rate_for_buyer(new_interest_rate);
        assert_eq!(
            Configuration::get_interest_rate_for_buyer(),
            new_interest_rate
        );
    }

    #[test]
    #[should_panic]
    fn test_should_not_allow_invalid_interest_rate_value() {
        init_canister();
        let new_interest_rate = 1.0;
        Marketplace::admin_set_interest_rate_for_buyer(new_interest_rate);
    }

    #[tokio::test]
    async fn test_should_get_icp_price() {
        init_canister();
        let icp_price = Marketplace::get_token_price_icp(TokenIdentifier::from(1))
            .await
            .unwrap();
        assert_eq!(icp_price, 1230012300);
    }

    fn init_canister() {
        let data = MarketplaceInitData {
            deferred_canister: deferred_canister(),
            fly_canister: fly_canister(),
            admins: vec![caller()],
        };
        Marketplace::init(data);
    }
}
