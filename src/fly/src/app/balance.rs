//! # Balances
//!
//! ICRC-1 token balances

mod account;
mod account_balance;

use std::cell::RefCell;

use candid::{Nat, Principal};
use did::fly::{BalanceError, FlyError, FlyResult, PicoFly};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, StableCell};
use icrc::icrc1::account::{Account, DEFAULT_SUBACCOUNT};

pub use self::account::StorableAccount;
use self::account_balance::Balance as AccountBalance;
use super::configuration::Configuration;
use crate::app::memory::{BALANCES_MEMORY_ID, CANISTER_WALLET_ACCOUNT_MEMORY_ID, MEMORY_MANAGER};

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

        BALANCES.with_borrow_mut(|balances| {
            // init accounts
            for (account, balance) in initial_balances {
                let storable_account = StorableAccount::from(account);
                balances.insert(storable_account, balance.clone().into());
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

    pub fn total_supply() -> PicoFly {
        let minting_account = Configuration::get_minting_account();
        BALANCES.with_borrow(|balances| {
            let mut supply = Nat::from(0);
            for (account, balance) in balances.iter() {
                if minting_account != account.0 {
                    supply += balance.amount;
                }
            }

            supply
        })
    }

    /// Get balance of account
    pub fn balance_of(account: Account) -> FlyResult<PicoFly> {
        Self::with_balance(account, |balance| balance.amount.clone())
    }

    /// Returns canister balance
    pub fn canister_balance() -> PicoFly {
        Self::balance_of(Self::canister_wallet_account()).unwrap()
    }

    pub fn canister_wallet_account() -> Account {
        CANISTER_WALLET_ACCOUNT.with_borrow(|wallet| wallet.get().0)
    }

    /// Transfer $picoFly tokens from `from` account to `to` account.
    /// The fee is transferred to the Minting Account, making it burned
    pub fn transfer(from: Account, to: Account, value: PicoFly, fee: PicoFly) -> FlyResult<()> {
        Self::transfer_wno_fees(from, to, value)?;

        if fee > 0_u64 {
            Self::transfer_wno_fees(from, Configuration::get_minting_account(), fee)
        } else {
            Ok(())
        }
    }

    /// Transfer $picoFly tokens from canister to `to` account.
    ///
    /// This function is meant to be used only by the dilazionato canister and does not apply fees or burns.
    pub fn transfer_wno_fees(from: Account, to: Account, value: PicoFly) -> FlyResult<()> {
        Self::with_balance_mut(from, |balance| {
            if balance.amount < value {
                return Err(FlyError::Balance(BalanceError::InsufficientBalance));
            }
            balance.amount -= value.clone();
            Ok(())
        })?;
        Self::with_balance_mut(to, |balance| {
            balance.amount += value;
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
                    balances.insert(storable_account.clone(), AccountBalance::from(Nat::from(0)));
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

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::app::test_utils::{alice_account, bob_account};
    use crate::utils::{self, fly_to_picofly};

    #[test]
    fn test_should_init_balances() {
        let total_supply = fly_to_picofly(8_888_888);

        let initial_balances = vec![
            (alice_account(), fly_to_picofly(188_888)),
            (bob_account(), fly_to_picofly(100_000)),
        ];

        Balance::init_balances(total_supply, initial_balances);

        let canister_account = CANISTER_WALLET_ACCOUNT.with_borrow(|wallet| wallet.get().0.clone());
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
    fn test_should_transfer_from_canister() {
        let total_supply = fly_to_picofly(8_888_888);
        let initial_balances = vec![];

        let recipient_account = Account {
            owner: utils::id(),
            subaccount: Some(utils::random_subaccount()),
        };

        Balance::init_balances(total_supply, initial_balances);
        assert!(Balance::transfer_wno_fees(
            Balance::canister_wallet_account(),
            recipient_account.clone(),
            fly_to_picofly(888)
        )
        .is_ok());
        assert_eq!(Balance::canister_balance(), fly_to_picofly(8_888_888 - 888));
        assert_eq!(
            Balance::balance_of(recipient_account).unwrap(),
            fly_to_picofly(888)
        );
    }

    #[test]
    fn test_should_transfer_between_accounts() {
        let total_supply = fly_to_picofly(8_888_888);
        let initial_balances = vec![
            (alice_account(), fly_to_picofly(120)),
            (bob_account(), fly_to_picofly(50)),
        ];
        Balance::init_balances(total_supply, initial_balances);

        // transfer
        assert!(Balance::transfer(
            alice_account(),
            bob_account(),
            fly_to_picofly(50),
            fly_to_picofly(1)
        )
        .is_ok());
        // verify balances
        assert_eq!(
            Balance::balance_of(alice_account()).unwrap(),
            fly_to_picofly(120 - 50 - 1)
        );
        assert_eq!(
            Balance::balance_of(bob_account()).unwrap(),
            fly_to_picofly(100)
        );
        // fee should be burned
        assert_eq!(Balance::total_supply(), fly_to_picofly(8_888_888 - 1));
    }

    #[test]
    fn test_should_fail_transfer_if_has_no_balance_to_pay_fee() {
        let total_supply = fly_to_picofly(8_888_888);
        let initial_balances = vec![
            (alice_account(), fly_to_picofly(50)),
            (bob_account(), fly_to_picofly(50)),
        ];
        Balance::init_balances(total_supply, initial_balances);

        // transfer
        assert!(Balance::transfer(
            alice_account(),
            bob_account(),
            fly_to_picofly(50),
            fly_to_picofly(1)
        )
        .is_err());
    }

    #[test]
    fn test_should_not_pay_fee_if_fee_is_zero() {
        let total_supply = fly_to_picofly(8_888_888);
        let initial_balances = vec![
            (alice_account(), fly_to_picofly(50)),
            (bob_account(), fly_to_picofly(50)),
        ];
        Balance::init_balances(total_supply, initial_balances);

        // transfer
        assert!(Balance::transfer(
            alice_account(),
            bob_account(),
            fly_to_picofly(50),
            fly_to_picofly(0)
        )
        .is_ok());
        // verify balances
        assert_eq!(
            Balance::balance_of(alice_account()).unwrap(),
            fly_to_picofly(0)
        );
        assert_eq!(
            Balance::balance_of(bob_account()).unwrap(),
            fly_to_picofly(100)
        );
        // fee should be burned
        assert_eq!(Balance::total_supply(), fly_to_picofly(8_888_888));
    }

    #[test]
    fn test_should_not_allow_transfer_if_not_enough_balance() {
        let total_supply = fly_to_picofly(8_888_888);
        let initial_balances = vec![
            (alice_account(), fly_to_picofly(50)),
            (bob_account(), fly_to_picofly(50)),
        ];
        Balance::init_balances(total_supply, initial_balances);

        // transfer
        assert!(Balance::transfer(
            alice_account(),
            bob_account(),
            fly_to_picofly(100),
            fly_to_picofly(1)
        )
        .is_err());
    }

    #[test]
    fn test_should_get_total_supply() {
        let total_supply = fly_to_picofly(8_888_888);
        let initial_balances = vec![(bob_account(), fly_to_picofly(100_000))];
        Balance::init_balances(total_supply, initial_balances);
        assert_eq!(Balance::total_supply(), fly_to_picofly(8_888_888));

        // burn
        assert!(Balance::transfer_wno_fees(
            bob_account(),
            Configuration::get_minting_account(),
            fly_to_picofly(100_000)
        )
        .is_ok());
        assert_eq!(Balance::total_supply(), fly_to_picofly(8_888_888 - 100_000));
    }
}
