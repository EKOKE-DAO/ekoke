mod configuration;
mod exchange_rate;
mod inspect;
mod memory;
mod roles;
mod test_utils;

use candid::Principal;
use did::marketplace::{MarketplaceInitData, MarketplaceResult};

use self::configuration::Configuration;
pub use self::inspect::Inspect;
use self::roles::RolesManager;
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

    fn init_canister() {
        let data = MarketplaceInitData {
            deferred_canister: deferred_canister(),
            fly_canister: fly_canister(),
            admins: vec![caller()],
        };
        Marketplace::init(data);
    }
}
