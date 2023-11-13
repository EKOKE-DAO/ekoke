//! # Balances
//!
//! ICRC-1 token balances

mod account;
mod account_balance;

use std::cell::RefCell;

use candid::Principal;
use did::fly::{BalanceError, FlyError, FlyResult, PicoFly};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, StableCell};
use icrc::icrc1::account::{Account, DEFAULT_SUBACCOUNT};

use self::account::StorableAccount;
use self::account_balance::Balance as AccountBalance;
use crate::app::memory::{
    BALANCES_MEMORY_ID, CANISTER_WALLET_ACCOUNT_MEMORY_ID, MEMORY_MANAGER,
    REWARD_POOL_WALLET_ACCOUNT_MEMORY_ID,
};

thread_local! {
    /// Account balances
    static BALANCES: RefCell<StableBTreeMap<StorableAccount, AccountBalance, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableBTreeMap::new(MEMORY_MANAGER.with(|mm| mm.get(BALANCES_MEMORY_ID)))
    );

    /// Wallet which contains all the native tokens of the canister
    static CANISTER_WALLET_ACCOUNT: RefCell<StableCell<StorableAccount, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(CANISTER_WALLET_ACCOUNT_MEMORY_ID)),
        Account {
        owner: Principal::anonymous(),
        subaccount: None,
    }.into()).unwrap());

    /// Wallet which contains all the tokens reserved of the reward pool
    static REWARD_POOL_WALLET_ACCOUNT: RefCell<StableCell<StorableAccount, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(REWARD_POOL_WALLET_ACCOUNT_MEMORY_ID)),
        Account {
        owner: Principal::anonymous(),
        subaccount: None,
    }.into()).unwrap());
}

pub struct Balance;

impl Balance {
    /// Set init balances
    ///
    /// WARNING: this function DOESN'T check anything and it's meant to be used only on init.
    /// Panics if initializing more than total supply.
    pub fn init_balances(mut total_supply: PicoFly, initial_balances: Vec<(Account, PicoFly)>) {
        // make canister acount
        let canister_account = Account {
            owner: crate::utils::id(),
            subaccount: Some(*DEFAULT_SUBACCOUNT),
        };
        // set canister
        CANISTER_WALLET_ACCOUNT.with_borrow_mut(|wallet| {
            wallet
                .set(StorableAccount::from(canister_account))
                .expect("failed to set canister account");
        });

        // make pool wallet
        let reward_pool_account = Account {
            owner: crate::utils::id(),
            subaccount: Some(crate::utils::random_subaccount()),
        };
        // set wallet
        REWARD_POOL_WALLET_ACCOUNT.with_borrow_mut(|wallet| {
            wallet
                .set(StorableAccount::from(reward_pool_account))
                .expect("failed to set canister account");
        });

        BALANCES.with_borrow_mut(|balances| {
            // init accounts
            for (account, balance) in initial_balances {
                let storable_account = StorableAccount::from(account);
                balances.insert(storable_account, balance.into());
                total_supply -= balance;
            }
            // set remaining supply to canister account
            balances.insert(
                StorableAccount::from(canister_account),
                AccountBalance {
                    amount: total_supply,
                },
            );
        });
    }

    /// Get balance of account
    pub fn balance_of(account: Account) -> FlyResult<u64> {
        Self::with_balance(account, |balance| balance.amount)
    }

    /// Returns canister balance
    pub fn canister_balance() -> u64 {
        Self::balance_of(Self::canister_wallet_account()).unwrap()
    }

    /// Get reward pool account
    pub fn reward_pool_account() -> Account {
        REWARD_POOL_WALLET_ACCOUNT.with_borrow(|wallet| wallet.get().0.clone())
    }

    fn canister_wallet_account() -> Account {
        CANISTER_WALLET_ACCOUNT.with_borrow(|wallet| wallet.get().0.clone())
    }

    /// Move funds from canister wallet to the pool
    pub fn move_from_canister_to_reward_pool(amount: PicoFly) -> FlyResult<()> {
        Self::with_balance_mut(Self::canister_wallet_account(), |balance| {
            if balance.amount < amount {
                return Err(FlyError::Balance(BalanceError::InsufficientBalance));
            }
            balance.amount -= amount;
            Ok(())
        })?;
        Self::with_balance_mut(Self::reward_pool_account(), |balance| {
            balance.amount += amount;
            Ok(())
        })
    }

    fn with_balance<F, T>(account: Account, f: F) -> FlyResult<T>
    where
        F: FnOnce(&AccountBalance) -> T,
    {
        let storable_account = StorableAccount::from(account);
        BALANCES.with_borrow(|balances| match balances.get(&storable_account) {
            Some(balance) => Ok(f(&balance)),
            None => Err(FlyError::Balance(BalanceError::AccountNotFound)),
        })
    }

    fn with_balance_mut<F, T>(account: Account, f: F) -> FlyResult<T>
    where
        F: FnOnce(&mut AccountBalance) -> FlyResult<T>,
    {
        let storable_account = StorableAccount::from(account);
        BALANCES.with_borrow_mut(|balances| {
            let mut balance = match balances.get(&storable_account) {
                Some(balance) => balance,
                None => {
                    // If balance is not set, create it with 0 balance
                    balances.insert(storable_account.clone(), AccountBalance::from(0));
                    balances.get(&storable_account).unwrap()
                }
            };
            let res = f(&mut balance)?;

            balances.insert(storable_account, balance);

            Ok(res)
        })
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;
    use crate::app::test_utils::{alice_account, bob_account};
    use crate::utils::fly_to_picofly;

    #[test]
    fn test_should_init_balances() {
        let total_supply = fly_to_picofly(8_888_888);

        let initial_balances = vec![
            (alice_account(), fly_to_picofly(188_888)),
            (bob_account(), fly_to_picofly(100_000)),
        ];

        Balance::init_balances(total_supply, initial_balances);

        let canister_account = CANISTER_WALLET_ACCOUNT.with_borrow(|wallet| wallet.get().0.clone());
        assert_ne!(canister_account, Balance::reward_pool_account());
        assert_eq!(
            Balance::balance_of(canister_account).unwrap(),
            fly_to_picofly(8_888_888 - 188_888 - 100_000)
        );
        assert_eq!(
            Balance::canister_balance(),
            fly_to_picofly(8_888_888 - 188_888 - 100_000)
        );
        assert_eq!(
            Balance::balance_of(alice_account()).unwrap(),
            fly_to_picofly(188_888)
        );
        assert_eq!(
            Balance::balance_of(bob_account()).unwrap(),
            fly_to_picofly(100_000)
        );
    }

    #[test]
    fn test_should_move_from_canister_to_pool() {
        let total_supply = fly_to_picofly(8_888_888);
        let initial_balances = vec![];

        Balance::init_balances(total_supply, initial_balances);
        assert!(Balance::move_from_canister_to_reward_pool(fly_to_picofly(888)).is_ok());
        assert_eq!(Balance::canister_balance(), fly_to_picofly(8_888_888 - 888));
        assert_eq!(
            Balance::balance_of(Balance::reward_pool_account()).unwrap(),
            fly_to_picofly(888)
        );
    }
}
