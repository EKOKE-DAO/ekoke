//! # Pool
//!
//! A pool that holds all of the tokens for the dilazionato contracts

use std::cell::RefCell;

use did::fly::{FlyError, FlyResult, PoolError};
use did::{StorableNat, ID};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{BTreeMap, DefaultMemoryImpl};
use icrc::icrc1::account::Account;

use super::balance::{Balance, StorableAccount};
use crate::app::memory::{MEMORY_MANAGER, POOL_MEMORY_ID};
use crate::utils;

thread_local! {
    /// Pool map is an association between a contract-id and the account which holds the pool for that contract.
    /// There is an account for each contract.
    static POOL: RefCell<BTreeMap<StorableNat, StorableAccount, VirtualMemory<DefaultMemoryImpl>>>
        = RefCell::new(BTreeMap::new(MEMORY_MANAGER.with(|mm| mm.get(POOL_MEMORY_ID))));
}

/// Pool is a thread-local BTreeMap that holds all of the tokens reserved for reward for the dilazionato contracts
pub struct Pool;

impl Pool {
    /// Reserve a pool with $picoFly tokens for a contract.
    /// If the contract already has a pool, the reward will be incremented
    ///
    /// Returns the new balance
    pub fn reserve(contract_id: &ID, from_account: Account, picofly: u64) -> FlyResult<u64> {
        let account = Self::with_pool_contract_mut(contract_id, |account| {
            Balance::transfer_wno_fees(from_account, *account, picofly)?;

            Ok(*account)
        })?;

        Balance::balance_of(account)
    }

    /// Returns pool balance for a contract
    pub fn balance_of(contract_id: &ID) -> FlyResult<u64> {
        Self::with_pool_contract(contract_id, |account| Balance::balance_of(*account))
    }

    /// Returns whether the provided contract has a pool reserved
    pub fn has_pool(contract_id: &ID) -> bool {
        Self::with_pool_contract(contract_id, |_| Ok(())).is_ok()
    }

    /// Withdraw $picoFly tokens from the pool and give them to `to` wallet
    ///
    /// Returns the new balance
    pub fn withdraw_tokens(contract_id: &ID, to: Account, picofly: u64) -> FlyResult<u64> {
        Self::with_pool_contract_mut(contract_id, |account| {
            Balance::transfer_wno_fees(*account, to, picofly)?;
            Balance::balance_of(*account)
        })
    }

    fn with_pool_contract<F, T>(contract_id: &ID, f: F) -> FlyResult<T>
    where
        F: FnOnce(&Account) -> FlyResult<T>,
    {
        POOL.with_borrow_mut(|pool| {
            if let Some(account) = pool.get(&contract_id.clone().into()) {
                f(&account.0)
            } else {
                Err(FlyError::Pool(PoolError::PoolNotFound(contract_id.clone())))
            }
        })
    }

    fn with_pool_contract_mut<F, T>(contract_id: &ID, f: F) -> FlyResult<T>
    where
        F: FnOnce(&mut Account) -> FlyResult<T>,
    {
        POOL.with_borrow_mut(|pool| {
            let key = contract_id.clone().into();
            if let Some(mut contract_pool) = pool.get(&key) {
                let res = f(&mut contract_pool.0)?;
                pool.insert(key, contract_pool);

                Ok(res)
            } else {
                // generate account
                let mut new_account = Account {
                    owner: utils::id(),
                    subaccount: Some(utils::random_subaccount()),
                };
                let res = f(&mut new_account)?;
                pool.insert(key, new_account.into());

                Ok(res)
            }
        })
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::app::test_utils;

    #[test]
    fn test_should_reserve_new_pool() {
        Balance::init_balances(utils::fly_to_picofly(8_000_000), vec![]);

        assert_eq!(
            Pool::reserve(&1_u64.into(), Balance::canister_wallet_account(), 7_000).unwrap(),
            7_000
        );
        assert_eq!(Pool::balance_of(&1_u64.into()).unwrap(), 7_000);
    }

    #[test]
    fn test_should_reserve_more_tokens() {
        Balance::init_balances(utils::fly_to_picofly(8_000_000), vec![]);

        assert_eq!(
            Pool::reserve(&1_u64.into(), Balance::canister_wallet_account(), 7_000).unwrap(),
            7_000
        );
        assert_eq!(
            Pool::reserve(&1_u64.into(), Balance::canister_wallet_account(), 3_000).unwrap(),
            10_000
        );
        assert_eq!(Pool::balance_of(&1_u64.into()).unwrap(), 10_000);
    }

    #[test]
    fn test_should_tell_whether_has_pool() {
        Balance::init_balances(utils::fly_to_picofly(8_000_000), vec![]);

        assert!(Pool::reserve(&1_u64.into(), Balance::canister_wallet_account(), 7_000).is_ok());
        assert!(Pool::has_pool(&1_u64.into()));
        assert!(!Pool::has_pool(&2_u64.into()));
    }

    #[test]
    fn test_should_withdraw_tokens_from_pool() {
        Balance::init_balances(utils::fly_to_picofly(8_000_000), vec![]);
        let to = test_utils::bob_account();

        assert!(Pool::reserve(&1_u64.into(), Balance::canister_wallet_account(), 7_000).is_ok());
        assert_eq!(
            Pool::withdraw_tokens(&1_u64.into(), to, 3_000).unwrap(),
            4_000
        );
        assert_eq!(Pool::balance_of(&1_u64.into()).unwrap(), 4_000);
        assert_eq!(Balance::balance_of(to).unwrap(), 3_000);
    }

    #[test]
    fn test_should_not_withdraw_more_tokens_than_available() {
        Balance::init_balances(utils::fly_to_picofly(8_000_000), vec![]);
        let to = test_utils::bob_account();

        assert!(Pool::reserve(&1_u64.into(), Balance::canister_wallet_account(), 7_000).is_ok());
        assert!(Pool::withdraw_tokens(&1_u64.into(), to, 8_000).is_err());
        assert!(Balance::balance_of(to).is_err());
    }
}