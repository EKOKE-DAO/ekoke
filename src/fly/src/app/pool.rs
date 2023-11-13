//! # Pool
//!
//! A pool that holds all of the tokens for the dilazionato contracts

use std::cell::RefCell;

use did::fly::{FlyError, FlyResult, PoolError};
use did::{StorableNat, ID};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{BTreeMap, DefaultMemoryImpl};

use crate::app::memory::{MEMORY_MANAGER, POOL_MEMORY_ID};

thread_local! {
    /// Pool map is an association between a contract-id and the amount of $picoFly tokens reserved
    static POOL: RefCell<BTreeMap<StorableNat, u64, VirtualMemory<DefaultMemoryImpl>>>
        = RefCell::new(BTreeMap::new(MEMORY_MANAGER.with(|mm| mm.get(POOL_MEMORY_ID))));
}

/// Pool is a thread-local BTreeMap that holds all of the tokens reserved for reward for the dilazionato contracts
pub struct Pool;

impl Pool {
    /// Reserve a pool with $picoFly tokens for a contract.
    /// If the contract already has a pool, the reward will be incremented
    ///
    /// Returns the new balance
    pub fn reserve(contract_id: &ID, picofly: u64) -> FlyResult<u64> {
        if Self::has_pool(contract_id) {
            Self::with_pool_contract_mut(contract_id, |pool| {
                *pool += picofly;
                Ok(*pool)
            })
        } else {
            POOL.with_borrow_mut(|pool| {
                pool.insert(contract_id.clone().into(), picofly);
            });
            Ok(picofly)
        }
    }

    /// Returns pool balance
    pub fn balance_of(contract_id: &ID) -> FlyResult<u64> {
        Self::with_pool_contract(contract_id, |pool| Ok(pool))
    }

    /// Returns whether the provided contract has a pool reserved
    pub fn has_pool(contract_id: &ID) -> bool {
        Self::with_pool_contract(contract_id, |_| Ok(())).is_ok()
    }

    /// Withdraw $picoFly tokens from the pool
    ///
    /// Returns the new balance
    pub fn withdraw_tokens(contract_id: &ID, picofly: u64) -> FlyResult<u64> {
        Self::with_pool_contract_mut(contract_id, |tokens| {
            if *tokens < picofly {
                Err(FlyError::Pool(PoolError::NotEnoughTokens))
            } else {
                *tokens -= picofly;
                Ok(*tokens)
            }
        })
    }

    fn with_pool_contract<F, T>(contract_id: &ID, f: F) -> FlyResult<T>
    where
        F: FnOnce(u64) -> FlyResult<T>,
    {
        POOL.with_borrow_mut(|pool| {
            if let Some(contract_pool) = pool.get(&contract_id.clone().into()) {
                f(contract_pool)
            } else {
                Err(FlyError::Pool(PoolError::PoolNotFound(contract_id.clone())))
            }
        })
    }

    fn with_pool_contract_mut<F>(contract_id: &ID, f: F) -> FlyResult<u64>
    where
        F: FnOnce(&mut u64) -> FlyResult<u64>,
    {
        POOL.with_borrow_mut(|pool| {
            let key = contract_id.clone().into();
            if let Some(mut contract_pool) = pool.get(&key) {
                f(&mut contract_pool)?;
                pool.insert(key, contract_pool);

                Ok(contract_pool)
            } else {
                Err(FlyError::Pool(PoolError::PoolNotFound(contract_id.clone())))
            }
        })
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_reserve_new_pool() {
        assert_eq!(Pool::reserve(&1_u64.into(), 7_000).unwrap(), 7_000);
        assert_eq!(Pool::balance_of(&1_u64.into()).unwrap(), 7_000);
    }

    #[test]
    fn test_should_reserve_more_tokens() {
        assert_eq!(Pool::reserve(&1_u64.into(), 7_000).unwrap(), 7_000);
        assert_eq!(Pool::reserve(&1_u64.into(), 3_000).unwrap(), 10_000);
        assert_eq!(Pool::balance_of(&1_u64.into()).unwrap(), 10_000);
    }

    #[test]
    fn test_should_tell_whether_has_pool() {
        assert!(Pool::reserve(&1_u64.into(), 7_000).is_ok());
        assert!(Pool::has_pool(&1_u64.into()));
        assert!(!Pool::has_pool(&2_u64.into()));
    }

    #[test]
    fn test_should_withdraw_tokens_from_pool() {
        assert!(Pool::reserve(&1_u64.into(), 7_000).is_ok());
        assert_eq!(Pool::withdraw_tokens(&1_u64.into(), 3_000).unwrap(), 4_000);
        assert_eq!(Pool::balance_of(&1_u64.into()).unwrap(), 4_000);
    }

    #[test]
    fn test_should_not_withdraw_more_tokens_than_available() {
        assert!(Pool::reserve(&1_u64.into(), 7_000).is_ok());
        assert!(Pool::withdraw_tokens(&1_u64.into(), 8_000).is_err());
    }
}
