//! # Pool
//!
//! A pool that holds all of the tokens for the deferred contracts

use std::cell::RefCell;

use candid::Nat;
use did::ekoke::{Ekoke, EkokeError, EkokeResult, PoolError};
use did::{StorableNat, ID};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{BTreeMap, DefaultMemoryImpl};
use icrc::icrc1::account::Account;
use num_traits::ToPrimitive;

use super::ledger_client::LedgerClient;
use crate::app::memory::{MEMORY_MANAGER, POOL_MEMORY_ID};
use crate::constants::ICRC1_FEE;

thread_local! {
    /// Pool map is an association between a contract-id and the pool balance for that contract
    static POOL: RefCell<BTreeMap<StorableNat, u128, VirtualMemory<DefaultMemoryImpl>>>
        = RefCell::new(BTreeMap::new(MEMORY_MANAGER.with(|mm| mm.get(POOL_MEMORY_ID))));
}

/// Pool is a thread-local BTreeMap that holds all of the tokens reserved for reward for the deferred contracts
pub struct Pool;

impl Pool {
    /// Reserve a pool with $ekoke tokens for a contract.
    /// If the contract already has a pool, the reward will be incremented
    ///
    /// Returns the new balance
    pub fn reserve(contract_id: &ID, ekoke: Ekoke) -> Ekoke {
        // get current reserve
        let current_reserve = Self::balance_of(contract_id)
            .ok()
            .unwrap_or(Nat::from(0u64));

        let new_reserve = (current_reserve + ekoke).0.to_u128().unwrap_or_default();
        // reserve tokens
        POOL.with_borrow_mut(|pool| {
            pool.insert(contract_id.clone().into(), new_reserve);
        });

        new_reserve.into()
    }

    /// Returns pool balance for a contract
    pub fn balance_of(contract_id: &ID) -> EkokeResult<Ekoke> {
        POOL.with_borrow(|pool| {
            pool.get(&contract_id.clone().into())
                .map(Ekoke::from)
                .ok_or(EkokeError::Pool(PoolError::PoolNotFound(
                    contract_id.clone(),
                )))
        })
    }

    /// Returns whether the provided contract has a pool reserved
    pub fn has_pool(contract_id: &ID) -> bool {
        Self::with_pool_contract(contract_id, |_| Ok(())).is_ok()
    }

    /// Withdraw $ekoke tokens from the pool and transfer them to `to` wallet
    pub async fn withdraw_tokens(contract_id: &ID, to: Account, ekoke: Ekoke) -> EkokeResult<()> {
        // Withdraw tokens from the liquidity pool for the contract
        let pool_liquidity = Self::balance_of(contract_id)?;
        // check if ekoke is bigger than liquidity
        if ekoke > pool_liquidity {
            return Err(EkokeError::Pool(PoolError::NotEnoughTokens));
        }
        // transfer funds
        if ekoke < ICRC1_FEE {
            return Err(EkokeError::Pool(PoolError::NotEnoughTokens));
        }
        let amount_to_send = ekoke.clone() - ICRC1_FEE;
        LedgerClient::transfer(to, amount_to_send).await?;
        // subtract funds from the liquidity pool
        let pool_liquidity = pool_liquidity.0.to_u128().unwrap_or_default();
        let ekoke = ekoke.0.to_u128().unwrap_or_default();
        let new_pool_liquidity = pool_liquidity.checked_sub(ekoke).unwrap_or_default();
        POOL.with_borrow_mut(|pool| {
            pool.insert(contract_id.clone().into(), new_pool_liquidity);
        });

        Ok(())
    }

    /// Returns the available liquidity in the pool
    pub async fn available_liquidity() -> EkokeResult<Ekoke> {
        let wallet_balance = LedgerClient::canister_balance()
            .await?
            .0
            .to_u128()
            .unwrap_or_default();
        // subtract from wallet the reserved amount in the pool
        let reserved = POOL.with_borrow(|pool| {
            pool.iter()
                .map(|(_, balance)| balance)
                .sum::<u128>()
                .to_u128()
                .unwrap_or_default()
        });

        Ok((wallet_balance.checked_sub(reserved).unwrap_or_default()).into())
    }

    fn with_pool_contract<F, T>(contract_id: &ID, f: F) -> EkokeResult<T>
    where
        F: FnOnce(u128) -> EkokeResult<T>,
    {
        POOL.with_borrow_mut(|pool| {
            if let Some(balance) = pool.get(&contract_id.clone().into()) {
                f(balance)
            } else {
                Err(EkokeError::Pool(PoolError::PoolNotFound(
                    contract_id.clone(),
                )))
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
        assert_eq!(Pool::reserve(&1_u64.into(), 7_000_u64.into()), 7_000_u64);
        assert_eq!(Pool::balance_of(&1_u64.into()).unwrap(), 7_000_u64);
    }

    #[test]
    fn test_should_reserve_more_tokens() {
        assert_eq!(Pool::reserve(&1_u64.into(), 7_000_u64.into()), 7_000_u64);
        assert_eq!(Pool::reserve(&1_u64.into(), 3_000_u64.into()), 10_000_u64);
        assert_eq!(Pool::balance_of(&1_u64.into()).unwrap(), 10_000_u64);
    }

    #[test]
    fn test_should_tell_whether_has_pool() {
        Pool::reserve(&1_u64.into(), 7_000_u64.into());
        assert!(Pool::has_pool(&1_u64.into()));
        assert!(!Pool::has_pool(&2_u64.into()));
    }

    #[tokio::test]
    async fn test_should_withdraw_tokens_from_pool() {
        let to = test_utils::bob_account();

        Pool::reserve(&1_u64.into(), 70_000_u64.into());
        assert!(Pool::withdraw_tokens(&1_u64.into(), to, 30_000_u64.into())
            .await
            .is_ok());
        assert_eq!(Pool::balance_of(&1_u64.into()).unwrap(), 40_000_u64);
    }

    #[tokio::test]
    async fn test_should_not_withdraw_more_tokens_than_available() {
        let to = test_utils::bob_account();

        Pool::reserve(&1_u64.into(), 70_000_u64.into());
        assert!(Pool::withdraw_tokens(&1_u64.into(), to, 80_000_u64.into())
            .await
            .is_err());
    }
}
