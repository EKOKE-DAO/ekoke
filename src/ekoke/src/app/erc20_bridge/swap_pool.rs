use candid::Principal;
use did::ekoke::{EkokeResult, PicoEkoke};
use icrc::icrc1::account::Account;

use crate::app::{Balance, Configuration};
use crate::utils;

/// Swap Pool contains the tokens exchanged from ERC20 to FLY
pub struct SwapPool;

impl SwapPool {
    /// Deposit $picoEkoke tokens to the swap pool from the provided account.
    pub async fn deposit(from: Account, amount: PicoEkoke) -> EkokeResult<()> {
        let swap_pool_account = Self::erc20_swap_pool_account().await?;
        Balance::transfer_wno_fees(from, swap_pool_account, amount)
    }

    /// Withdraw $picoEkoke tokens from the swap pool to the provided account.
    pub async fn withdraw(to: Account, amount: PicoEkoke) -> EkokeResult<()> {
        let swap_pool_account = Self::erc20_swap_pool_account().await?;
        Balance::transfer_wno_fees(swap_pool_account, to, amount)
    }

    /// Returns the ERC20 swap pool account.
    ///
    /// If not initialized, it will be initialized.
    async fn erc20_swap_pool_account() -> EkokeResult<Account> {
        let swap_pool_account = Configuration::get_erc20_swap_pool_account();
        // if swap pool account is initialized, return it
        if swap_pool_account.owner != Principal::anonymous() {
            return Ok(swap_pool_account);
        }

        // otherwise initialize it
        loop {
            let new_account = Account {
                owner: utils::id(),
                subaccount: Some(utils::random_subaccount().await),
            };

            // check if account already exists or if it's the minting account
            if Balance::balance_of(new_account).is_ok()
                || new_account == Configuration::get_minting_account()
            {
                // account already exists, try again
                continue;
            }

            // set the new account
            Configuration::set_erc20_swap_pool_account(new_account);

            return Ok(new_account);
        }
    }
}

#[cfg(test)]
mod test {

    use candid::Nat;
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;
    use crate::app::test_utils;

    #[tokio::test]
    async fn test_should_deposit_and_withdraw_from_swap_pool() {
        let alice = test_utils::alice_account();
        Balance::init_balances(Nat::from(100_u64), vec![(alice.clone(), 100_u64.into())]);

        // check if swap pool account is anonymous
        let swap_pool_account = Configuration::get_erc20_swap_pool_account();
        assert_eq!(swap_pool_account.owner, Principal::anonymous());

        assert!(SwapPool::deposit(alice, Nat::from(20_u64)).await.is_ok());

        // check if swap pool account has been initialized
        let swap_pool_account = Configuration::get_erc20_swap_pool_account();
        assert_ne!(swap_pool_account.owner, Principal::anonymous());
        assert_eq!(swap_pool_account.owner, utils::id());
        assert!(swap_pool_account.subaccount.is_some());

        // check if alice has 80 picoEkoke
        assert_eq!(Balance::balance_of(alice).unwrap(), 80_u64);

        // check if swap pool has 20 picoEkoke
        assert_eq!(Balance::balance_of(swap_pool_account).unwrap(), 20_u64);

        // withdraw
        assert!(SwapPool::withdraw(alice, Nat::from(10_u64)).await.is_ok());
        assert_eq!(Balance::balance_of(alice).unwrap(), 90_u64);
        assert_eq!(Balance::balance_of(swap_pool_account).unwrap(), 10_u64);
    }

    #[tokio::test]
    async fn should_not_allow_withdraw_or_deposit_with_insufficient_balance() {
        let alice = test_utils::alice_account();
        Balance::init_balances(Nat::from(100_u64), vec![(alice.clone(), 100_u64.into())]);

        assert!(SwapPool::deposit(alice.clone(), Nat::from(200_u64))
            .await
            .is_err());

        assert!(SwapPool::deposit(alice, Nat::from(20_u64)).await.is_ok());

        assert!(SwapPool::withdraw(alice, Nat::from(30_u64)).await.is_err());
    }
}
