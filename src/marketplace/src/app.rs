mod configuration;
mod inspect;
mod memory;
mod roles;
mod test_utils;

use did::marketplace::MarketplaceInitData;

use self::configuration::Configuration;
pub use self::inspect::Inspect;
use self::roles::RolesManager;

pub struct Marketplace;

impl Marketplace {
    pub fn init(data: MarketplaceInitData) {
        Configuration::set_deferred_canister(data.deferred_canister);
        Configuration::set_fly_canister(data.fly_canister);
        RolesManager::set_admins(data.admins).unwrap();
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

    fn init_canister() {
        let data = MarketplaceInitData {
            deferred_canister: deferred_canister(),
            fly_canister: fly_canister(),
            admins: vec![caller()],
        };
        Marketplace::init(data);
    }
}
