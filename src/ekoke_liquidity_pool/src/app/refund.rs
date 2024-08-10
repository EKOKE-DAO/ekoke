use std::cell::RefCell;

use candid::{Nat, Principal};
use did::{StorableNat, StorablePrincipal};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};

use crate::app::memory::{MEMORY_MANAGER, REFUND_MAP_MEMORY_ID};

thread_local! {
    /// Refund map
    static REFUND_MAP: RefCell<StableBTreeMap<StorablePrincipal, StorableNat, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableBTreeMap::new(MEMORY_MANAGER.with(|mm| mm.get(REFUND_MAP_MEMORY_ID)))
    );
}

pub struct Refund;

impl Refund {
    /// Insert or update refund
    pub fn insert_or_update(principal: Principal, amount: Nat) {
        let storable_principal = StorablePrincipal(principal);
        let current_amount = REFUND_MAP.with_borrow(|refunds| {
            refunds
                .get(&storable_principal)
                .map(|nat| nat.0)
                .unwrap_or_else(|| Nat::from(0u64))
        });

        let new_amount = current_amount + amount;

        REFUND_MAP.with_borrow_mut(|refunds| {
            refunds.insert(storable_principal, StorableNat(new_amount));
        })
    }

    /// Remove refund
    pub fn remove(principal: Principal) {
        REFUND_MAP.with_borrow_mut(|refunds| {
            let storable_principal = StorablePrincipal(principal);
            refunds.remove(&storable_principal);
        })
    }

    /// Get refund
    pub fn get(principal: Principal) -> Option<Nat> {
        REFUND_MAP.with_borrow(|refunds| {
            let storable_principal = StorablePrincipal(principal);
            refunds.get(&storable_principal).map(|nat| nat.0)
        })
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_insert_refund() {
        Refund::insert_or_update(Principal::anonymous(), Nat::from(100u64));
        Refund::insert_or_update(Principal::anonymous(), Nat::from(200u64));

        assert_eq!(Refund::get(Principal::anonymous()), Some(Nat::from(300u64)));
    }

    #[test]
    fn test_should_remove_refund() {
        Refund::insert_or_update(Principal::anonymous(), Nat::from(100u64));
        Refund::remove(Principal::anonymous());

        assert_eq!(Refund::get(Principal::anonymous()), None);
    }
}
