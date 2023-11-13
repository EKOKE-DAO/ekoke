//! # Inspect
//!
//! Dilazionato inspect message handler

use candid::Principal;
use did::fly::Role;

use super::roles::RolesManager;

pub struct Inspect;

impl Inspect {
    /// Returns whether caller is custodian of the canister
    pub fn inspect_is_admin(caller: Principal) -> bool {
        RolesManager::is_admin(caller)
    }

    /// Returns whether caller is dilazionato canister
    pub fn inspect_is_dilazionato_canister(caller: Principal) -> bool {
        RolesManager::has_role(caller, Role::DilazionatoCanister)
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_inspect_is_custodian() {
        let caller = Principal::anonymous();
        assert_eq!(Inspect::inspect_is_admin(caller), false);

        let caller = Principal::from_text("aaaaa-aa").unwrap();
        assert_eq!(Inspect::inspect_is_admin(caller), false);

        let caller = Principal::from_text("aaaaa-aa").unwrap();
        assert!(RolesManager::set_admins(vec![caller]).is_ok());
        assert_eq!(Inspect::inspect_is_admin(caller), true);
    }

    #[test]
    fn test_should_inspect_is_dilazionato() {
        let caller = Principal::anonymous();
        assert_eq!(Inspect::inspect_is_dilazionato_canister(caller), false);

        let caller = Principal::from_text("aaaaa-aa").unwrap();
        RolesManager::give_role(caller, Role::DilazionatoCanister);
        assert_eq!(Inspect::inspect_is_dilazionato_canister(caller), true);
    }
}
