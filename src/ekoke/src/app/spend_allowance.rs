mod key;
mod spend;

use std::cell::RefCell;

use candid::{Nat, Principal};
use did::ekoke::{AllowanceError, EkokeError, EkokeResult};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use icrc::icrc1::account::{Account, Subaccount};
use icrc::icrc2::approve::ApproveArgs;
use key::AllowanceKey;
use spend::Spend;

use crate::app::memory::{MEMORY_MANAGER, SPEND_ALLOWANCE_MEMORY_ID};

thread_local! {
    /// Spend allowance
    static SPEND_ALLOWANCE: RefCell<StableBTreeMap<AllowanceKey, Spend, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableBTreeMap::new(MEMORY_MANAGER.with(|mm| mm.get(SPEND_ALLOWANCE_MEMORY_ID)))
    );

}

/// Takes care of verifying and storing spend allowance for ICRC2 token
pub struct SpendAllowance;

impl SpendAllowance {
    /// Approve a new spend from spender to caller.
    ///
    /// If the allowance already exists, then the amount is incremented.
    pub fn approve_spend(caller: Principal, approve: ApproveArgs) -> EkokeResult<Nat> {
        // check if caller is the spender
        if approve.spender.owner == caller {
            return Err(EkokeError::Allowance(AllowanceError::BadSpender));
        }
        // check if expiration is in the past
        if approve
            .expires_at
            .map(|exp| exp < crate::utils::time())
            .unwrap_or_default()
        {
            return Err(EkokeError::Allowance(AllowanceError::BadExpiration));
        }

        let allowance_key = AllowanceKey::new(
            Account {
                owner: caller,
                subaccount: approve.from_subaccount,
            },
            approve.spender,
        );
        let mut spend = Spend::from(approve);

        // if the allowance exists, then update current allowance
        let incr_allowance_closure = |existing_spend: &mut Spend| {
            // check expected allowance
            if spend
                .expected_allowance
                .as_ref()
                .map(|allowance| &existing_spend.amount != allowance)
                .unwrap_or_default()
            {
                return Err(EkokeError::Allowance(AllowanceError::AllowanceChanged));
            }

            // increment spend amount and overwrite current speed
            spend.amount += existing_spend.amount.clone();
            let new_amount = spend.amount.clone();
            *existing_spend = spend.clone();

            Ok(new_amount)
        };
        match Self::with_allowance_mut(&allowance_key, incr_allowance_closure) {
            Ok(new_amount) => return Ok(new_amount),
            Err(EkokeError::Allowance(AllowanceError::AllowanceNotFound)) => {}
            Err(err) => return Err(err),
        };

        let amount = spend.amount.clone();
        // check expected allowance to be None
        if spend.expected_allowance.is_some() {
            return Err(EkokeError::Allowance(AllowanceError::AllowanceChanged));
        }

        // if doesn't exist
        SPEND_ALLOWANCE.with_borrow_mut(|allowances| {
            allowances.insert(allowance_key, spend);
        });

        Ok(amount)
    }

    /// Spend allowance from spender to caller.
    pub fn spend_allowance(
        caller: Principal,
        from: Account,
        amount: Nat,
        spender_subaccount: Option<Subaccount>,
    ) -> EkokeResult<()> {
        let spender = Account {
            owner: caller,
            subaccount: spender_subaccount,
        };

        let allowance_key = AllowanceKey::new(from, spender);
        Self::with_allowance_mut(&allowance_key, |spend| {
            // check if expired
            if spend
                .expires_at
                .map(|exp| exp < crate::utils::time())
                .unwrap_or_default()
            {
                return Err(EkokeError::Allowance(AllowanceError::AllowanceExpired));
            }
            // check balance
            if spend.amount < amount {
                return Err(EkokeError::Allowance(AllowanceError::InsufficientFunds));
            }

            spend.amount -= amount;

            Ok(())
        })
    }

    /// Get allowance for spender from owner.
    pub fn get_allowance(owner: Account, spender: Account) -> (Nat, Option<u64>) {
        let allowance_key = AllowanceKey::new(owner, spender);
        Self::with_allowance(&allowance_key, |spend| {
            (spend.amount.clone(), spend.expires_at)
        })
        .unwrap_or_default()
    }

    /// Remove the expired allowance from the map
    #[allow(unused)]
    pub fn remove_expired_allowance() {
        let now = crate::utils::time();
        SPEND_ALLOWANCE.with_borrow_mut(|allowances| {
            let mut expired_allowances = vec![];
            for (key, spend) in allowances.iter() {
                if spend.expires_at.map(|exp| exp < now).unwrap_or_default() || spend.amount == 0 {
                    expired_allowances.push(key.clone());
                }
            }

            for key in expired_allowances {
                allowances.remove(&key);
            }
        });
    }

    fn with_allowance<F, T>(allowance: &AllowanceKey, f: F) -> EkokeResult<T>
    where
        F: FnOnce(&Spend) -> T,
    {
        SPEND_ALLOWANCE.with_borrow(|allowances| match allowances.get(allowance) {
            Some(balance) => Ok(f(&balance)),
            None => Err(EkokeError::Allowance(AllowanceError::AllowanceNotFound)),
        })
    }

    fn with_allowance_mut<F, T>(allowance: &AllowanceKey, f: F) -> EkokeResult<T>
    where
        F: FnOnce(&mut Spend) -> EkokeResult<T>,
    {
        SPEND_ALLOWANCE.with_borrow_mut(|allowances| {
            let mut spend = allowances
                .get(allowance)
                .ok_or(EkokeError::Allowance(AllowanceError::AllowanceNotFound))?;
            let res = f(&mut spend)?;

            allowances.insert(allowance.clone(), spend);

            Ok(res)
        })
    }
}

#[cfg(test)]
mod test {

    use std::time::Duration;

    use candid::Principal;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::app::test_utils::{alice_account, bob_account, caller_account};
    use crate::utils::caller;

    #[test]
    fn test_should_insert_new_allowance() {
        let allowance = ApproveArgs {
            from_subaccount: None,
            spender: alice_account(),
            amount: 100.into(),
            expected_allowance: None,
            expires_at: None,
            fee: None,
            memo: None,
            created_at_time: None,
        };

        let allowance_key = AllowanceKey::new(caller_account(), allowance.spender);

        assert!(SpendAllowance::approve_spend(caller(), allowance.clone()).is_ok());

        let spend = SPEND_ALLOWANCE.with_borrow(|allowances| allowances.get(&allowance_key));

        assert!(spend.is_some());
        assert_eq!(spend.unwrap().amount, allowance.amount);
    }

    #[test]
    fn test_should_overwrite_allowance() {
        let exp_1 = crate::utils::time() * 2;
        let allowance = ApproveArgs {
            from_subaccount: None,
            spender: alice_account(),
            amount: 100.into(),
            expected_allowance: None,
            expires_at: Some(exp_1),
            fee: None,
            memo: None,
            created_at_time: None,
        };

        let allowance_key = AllowanceKey::new(caller_account(), allowance.spender);

        assert!(SpendAllowance::approve_spend(caller(), allowance.clone()).is_ok());

        // overwrite

        let exp_2 = crate::utils::time() * 3;
        let allowance = ApproveArgs {
            from_subaccount: None,
            spender: alice_account(),
            amount: 150.into(),
            expected_allowance: None,
            expires_at: Some(exp_2),
            fee: None,
            memo: None,
            created_at_time: None,
        };

        assert!(SpendAllowance::approve_spend(caller(), allowance.clone()).is_ok());
        let spend = SPEND_ALLOWANCE.with_borrow(|allowances| allowances.get(&allowance_key));

        assert!(spend.is_some());
        assert_eq!(spend.as_ref().unwrap().amount, 250_u64);
        assert_eq!(spend.as_ref().unwrap().expires_at.unwrap(), exp_2);
    }

    #[test]
    fn test_should_not_authorize_same_spender() {
        let allowance = ApproveArgs {
            from_subaccount: None,
            spender: crate::app::test_utils::caller_account(),
            amount: 100.into(),
            expected_allowance: None,
            expires_at: None,
            fee: None,
            memo: None,
            created_at_time: None,
        };

        assert!(SpendAllowance::approve_spend(caller(), allowance.clone()).is_err());
    }

    #[test]
    fn test_should_not_authorize_expired_allowance() {
        let allowance = ApproveArgs {
            from_subaccount: None,
            spender: alice_account(),
            amount: 100.into(),
            expected_allowance: None,
            expires_at: Some(crate::utils::time() - 1),
            fee: None,
            memo: None,
            created_at_time: None,
        };

        assert!(SpendAllowance::approve_spend(caller(), allowance.clone()).is_err());
    }

    #[test]
    fn test_should_not_authorize_changed_allowance() {
        let allowance = ApproveArgs {
            from_subaccount: None,
            spender: alice_account(),
            amount: 100.into(),
            expected_allowance: None,
            expires_at: None,
            fee: None,
            memo: None,
            created_at_time: None,
        };

        assert!(SpendAllowance::approve_spend(caller(), allowance.clone()).is_ok());

        let allowance = ApproveArgs {
            from_subaccount: None,
            spender: alice_account(),
            amount: 100.into(),
            expected_allowance: Some(50.into()),
            expires_at: None,
            fee: None,
            memo: None,
            created_at_time: None,
        };

        assert!(SpendAllowance::approve_spend(caller(), allowance.clone()).is_err());

        let allowance = ApproveArgs {
            from_subaccount: None,
            spender: alice_account(),
            amount: 100.into(),
            expected_allowance: Some(100.into()),
            expires_at: None,
            fee: None,
            memo: None,
            created_at_time: None,
        };

        assert!(SpendAllowance::approve_spend(caller(), allowance.clone()).is_ok());
    }

    #[test]
    fn test_should_not_authorize_changed_allowance_on_new_allowance() {
        let allowance = ApproveArgs {
            from_subaccount: None,
            spender: alice_account(),
            amount: 100.into(),
            expected_allowance: Some(50.into()),
            expires_at: None,
            fee: None,
            memo: None,
            created_at_time: None,
        };

        assert!(SpendAllowance::approve_spend(caller(), allowance.clone()).is_err());
    }

    #[test]
    fn test_should_spend_allowance() {
        let allowance = ApproveArgs {
            from_subaccount: bob_account().subaccount,
            spender: caller_account(),
            amount: 100.into(),
            expected_allowance: None,
            expires_at: None,
            fee: None,
            memo: None,
            created_at_time: None,
        };

        let allowance_key = AllowanceKey::new(bob_account(), caller_account());
        let spend = Spend::from(allowance);

        SPEND_ALLOWANCE.with_borrow_mut(|allowances| {
            allowances.insert(allowance_key.clone(), spend);
        });

        assert!(SpendAllowance::spend_allowance(
            caller(),
            bob_account(),
            25.into(),
            caller_account().subaccount
        )
        .is_ok());
        let spend = SPEND_ALLOWANCE.with_borrow(|allowances| allowances.get(&allowance_key));

        assert!(spend.is_some());
        assert_eq!(spend.as_ref().unwrap().amount, 75_u64);
    }

    #[test]
    fn test_should_not_spend_allowance_if_expired() {
        let allowance = ApproveArgs {
            from_subaccount: bob_account().subaccount,
            spender: caller_account(),
            amount: 100.into(),
            expected_allowance: None,
            expires_at: Some(crate::utils::time() + 100_000),
            fee: None,
            memo: None,
            created_at_time: None,
        };

        let allowance_key = AllowanceKey::new(bob_account(), caller_account());
        let spend = Spend::from(allowance);

        SPEND_ALLOWANCE.with_borrow_mut(|allowances| {
            allowances.insert(allowance_key.clone(), spend);
        });

        std::thread::sleep(Duration::from_millis(100));

        assert!(SpendAllowance::spend_allowance(
            caller(),
            bob_account(),
            25.into(),
            caller_account().subaccount
        )
        .is_err());
    }

    #[test]
    fn test_should_not_spend_allowance_if_insufficient_funds() {
        let allowance = ApproveArgs {
            from_subaccount: bob_account().subaccount,
            spender: caller_account(),
            amount: 100.into(),
            expected_allowance: None,
            expires_at: None,
            fee: None,
            memo: None,
            created_at_time: None,
        };

        let allowance_key = AllowanceKey::new(bob_account(), caller_account());
        let spend = Spend::from(allowance);

        SPEND_ALLOWANCE.with_borrow_mut(|allowances| {
            allowances.insert(allowance_key, spend);
        });

        assert!(SpendAllowance::spend_allowance(
            caller(),
            bob_account(),
            125.into(),
            caller_account().subaccount
        )
        .is_err());
    }

    #[test]
    fn test_should_get_allowance() {
        let allowance = ApproveArgs {
            from_subaccount: None,
            spender: alice_account(),
            amount: 100.into(),
            expected_allowance: None,
            expires_at: None,
            fee: None,
            memo: None,
            created_at_time: None,
        };

        assert!(SpendAllowance::approve_spend(caller(), allowance.clone()).is_ok());

        assert_eq!(
            SpendAllowance::get_allowance(caller_account(), alice_account()),
            (100.into(), None)
        );

        let exp = crate::utils::time() * 2;

        let allowance = ApproveArgs {
            from_subaccount: None,
            spender: bob_account(),
            amount: 100.into(),
            expected_allowance: None,
            expires_at: Some(exp),
            fee: None,
            memo: None,
            created_at_time: None,
        };

        assert!(SpendAllowance::approve_spend(caller(), allowance.clone()).is_ok());

        assert_eq!(
            SpendAllowance::get_allowance(caller_account(), bob_account()),
            (100.into(), Some(exp))
        );

        // unexisting account
        assert_eq!(
            SpendAllowance::get_allowance(alice_account(), bob_account()),
            (0.into(), None)
        );
    }

    #[test]
    fn test_should_remove_expired_allowances() {
        let allowance = ApproveArgs {
            from_subaccount: None,
            spender: alice_account(),
            amount: 100.into(),
            expected_allowance: None,
            expires_at: Some(crate::utils::time() * 2),
            fee: None,
            memo: None,
            created_at_time: None,
        };

        assert!(SpendAllowance::approve_spend(caller(), allowance.clone()).is_ok());

        let allowance = ApproveArgs {
            from_subaccount: None,
            spender: bob_account(),
            amount: 100.into(),
            expected_allowance: None,
            expires_at: Some(crate::utils::time() + 100_000),
            fee: None,
            memo: None,
            created_at_time: None,
        };

        assert!(SpendAllowance::approve_spend(caller(), allowance.clone()).is_ok());

        let allowance = ApproveArgs {
            from_subaccount: None,
            spender: Account {
                owner: Principal::management_canister(),
                subaccount: None,
            },
            amount: 0.into(),
            expected_allowance: None,
            expires_at: None,
            fee: None,
            memo: None,
            created_at_time: None,
        };

        assert!(SpendAllowance::approve_spend(caller(), allowance.clone()).is_ok());

        std::thread::sleep(Duration::from_millis(100));

        SpendAllowance::remove_expired_allowance();

        assert_eq!(
            SPEND_ALLOWANCE.with_borrow(|allowances| allowances.len()),
            1
        );
    }
}
