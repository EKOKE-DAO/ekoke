//! # Inspect
//!
//! Deferred inspect message handler

use candid::Principal;
use did::fly::Role;
use icrc::icrc1::account::Account;

use super::roles::RolesManager;

pub struct Inspect;

impl Inspect {
    /// Returns whether caller is custodian of the canister
    pub fn inspect_is_admin(caller: Principal) -> bool {
        RolesManager::is_admin(caller)
    }

    /// Returns whether caller is deferred canister
    pub fn inspect_is_deferred_canister(caller: Principal) -> bool {
        RolesManager::has_role(caller, Role::DeferredCanister)
    }

    /// Returns whether caller is owner of the wallet
    pub fn inspect_caller_owns_wallet(caller: Principal, account: Account) -> bool {
        caller == account.owner
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::app::test_utils;

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
    fn test_should_inspect_is_deferred() {
        let caller = Principal::anonymous();
        assert_eq!(Inspect::inspect_is_deferred_canister(caller), false);

        let caller = Principal::from_text("aaaaa-aa").unwrap();
        RolesManager::give_role(caller, Role::DeferredCanister);
        assert_eq!(Inspect::inspect_is_deferred_canister(caller), true);
    }

    #[test]
    fn test_should_inspect_owns_wallet() {
        assert!(Inspect::inspect_caller_owns_wallet(
            Principal::from_text("aaaaa-aa").unwrap(),
            Account {
                owner: Principal::from_text("aaaaa-aa").unwrap(),
                subaccount: Some([
                    0x21, 0xa9, 0x95, 0x49, 0xe7, 0x92, 0x90, 0x7c, 0x5e, 0x27, 0x5e, 0x54, 0x51,
                    0x06, 0x8d, 0x4d, 0xdf, 0x4d, 0x43, 0xee, 0x8d, 0xca, 0xb4, 0x87, 0x56, 0x23,
                    0x1a, 0x8f, 0xb7, 0x71, 0x31, 0x23,
                ])
            }
        ));

        assert!(!Inspect::inspect_caller_owns_wallet(
            Principal::from_text("aaaaa-aa").unwrap(),
            test_utils::alice_account()
        ));
    }
}
