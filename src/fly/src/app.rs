//! # App
//!
//! API implementation for dilazionato canister

mod balance;
mod configuration;
mod inspect;
mod memory;
mod pool;
mod register;
mod reward;
mod roles;
#[cfg(test)]
mod test_utils;

use std::time::Duration;

use candid::{Nat, Principal};
use did::fly::{BalanceError, FlyError, FlyInitData, FlyResult, PicoFly, Role, Transaction};
use did::ID;
use icrc::icrc::generic_metadata_value::MetadataValue;
use icrc::icrc1::account::Account;
use icrc::icrc1::{self, transfer as icrc1_transfer, Icrc1};
use icrc::icrc2::{self, Icrc2};

use self::balance::Balance;
use self::configuration::Configuration;
pub use self::inspect::Inspect;
use self::pool::Pool;
use self::reward::Reward;
use self::roles::RolesManager;
use crate::app::register::Register;
use crate::constants::{
    ICRC1_DECIMALS, ICRC1_FEE, ICRC1_LOGO, ICRC1_NAME, ICRC1_SYMBOL, ICRC1_TX_TIME_SKID,
};
use crate::utils::{self, random_subaccount};

pub struct FlyCanister;

impl FlyCanister {
    /// Init fly canister
    pub fn init(data: FlyInitData) {
        // Set minting account
        Configuration::set_minting_account(Account {
            owner: utils::id(),
            subaccount: Some(random_subaccount()),
        });
        if let Err(err) = RolesManager::set_admins(data.admins) {
            ic_cdk::trap(&format!("Error setting admins: {}", err));
        }
        // Set dilazionato canister
        RolesManager::give_role(data.dilazionato_canister, Role::DilazionatoCanister);
        // init balances
        Balance::init_balances(
            utils::fly_to_picofly(data.total_supply),
            data.initial_balances,
        );
    }

    pub fn post_upgrade() {}

    /// Reserve a pool for the provided contract ID with the provided amount of $picoFly tokens.
    ///
    /// The tokens are withdrawned from the from's wallet.
    /// Obviously `from` wallet must be owned by the caller.
    pub fn reserve_pool(from: Account, contract_id: ID, picofly_amount: u64) -> FlyResult<u64> {
        if !Inspect::inspect_caller_owns_wallet(utils::caller(), from) {
            ic_cdk::trap("You don't own this account");
        }

        Pool::reserve(&contract_id, from, picofly_amount)
    }

    /// Get contract reward.
    ///
    /// This method can be called only by the dilazionato canister.
    ///
    /// If a pool is already reserved for the provided contract ID, the reserved amount will be returned.
    /// Otherwise, the provided amount will be reserved from canister wallet, if possible and returned.
    ///
    /// If the canister wallet doesn't have enough tokens to reserve `InsufficientBalance` error is returned
    pub fn get_contract_reward(contract_id: ID, installments: u64) -> FlyResult<PicoFly> {
        if !Inspect::inspect_is_dilazionato_canister(utils::caller()) {
            ic_cdk::trap("Unauthorized");
        }

        Reward::get_contract_reward(contract_id, installments)
    }

    /// Set role to the provided principal
    pub fn admin_set_role(principal: Principal, role: Role) {
        if !Inspect::inspect_is_admin(utils::caller()) {
            ic_cdk::trap("Unauthorized");
        }
        RolesManager::give_role(principal, role)
    }

    /// Remove role from the provided principal
    pub fn admin_remove_role(principal: Principal, role: Role) -> FlyResult<()> {
        if !Inspect::inspect_is_admin(utils::caller()) {
            ic_cdk::trap("Unauthorized");
        }
        RolesManager::remove_role(principal, role)
    }
}

impl Icrc1 for FlyCanister {
    fn icrc1_name() -> &'static str {
        ICRC1_NAME
    }

    fn icrc1_symbol() -> &'static str {
        ICRC1_SYMBOL
    }

    fn icrc1_decimals() -> u8 {
        ICRC1_DECIMALS
    }

    fn icrc1_fee() -> Nat {
        ICRC1_FEE.into()
    }

    fn icrc1_metadata() -> Vec<(String, MetadataValue)> {
        vec![
            (
                "icrc1:symbol".to_string(),
                MetadataValue::from(ICRC1_SYMBOL),
            ),
            ("icrc1:name".to_string(), MetadataValue::from(ICRC1_NAME)),
            (
                "icrc1:decimals".to_string(),
                MetadataValue::from(Nat::from(ICRC1_DECIMALS)),
            ),
            (
                "icrc1:fee".to_string(),
                MetadataValue::from(Nat::from(ICRC1_FEE)),
            ),
            ("icrc1:logo".to_string(), MetadataValue::from(ICRC1_LOGO)),
        ]
    }

    fn icrc1_total_supply() -> Nat {
        Balance::total_supply().into()
    }

    fn icrc1_minting_account() -> Account {
        Configuration::get_minting_account()
    }

    fn icrc1_balance_of(account: Account) -> Nat {
        Balance::balance_of(account).unwrap_or_default().into()
    }

    fn icrc1_transfer(
        transfer_args: icrc1_transfer::TransferArg,
    ) -> Result<Nat, icrc1_transfer::TransferError> {
        // get fee and check if fee is at least ICRC1_FEE
        let fee = transfer_args.fee.unwrap_or(ICRC1_FEE.into());
        if fee < ICRC1_FEE {
            return Err(icrc1_transfer::TransferError::BadFee {
                expected_fee: ICRC1_FEE.into(),
            });
        }

        // get u64 values
        let amount = utils::nat_to_u64(transfer_args.amount)?;
        let fee = utils::nat_to_u64(fee)?;

        // check if the transaction is too old
        let now = Duration::from_nanos(utils::time());
        let tx_created_at = Duration::from_nanos(transfer_args.created_at_time.unwrap_or_default());
        if now > tx_created_at && now.saturating_sub(tx_created_at) > ICRC1_TX_TIME_SKID {
            return Err(icrc1_transfer::TransferError::TooOld);
        } else if tx_created_at.saturating_sub(now) > ICRC1_TX_TIME_SKID {
            return Err(icrc1_transfer::TransferError::CreatedInFuture {
                ledger_time: now.as_nanos() as u64,
            });
        }

        // check memo length
        if let Some(memo) = &transfer_args.memo {
            if memo.0.len() < 32 || memo.0.len() > 64 {
                return Err(icrc1_transfer::TransferError::GenericError {
                    error_code: Nat::from(1),
                    message: "Invalid memo length. I must have a length between 32 and 64 bytes"
                        .to_string(),
                });
            }
        }

        // get from account
        let from_account = Account {
            owner: utils::caller(),
            subaccount: transfer_args.from_subaccount,
        };

        // check if it is a burn
        if transfer_args.to == Self::icrc1_minting_account() {
            Balance::transfer_wno_fees(from_account, transfer_args.to, amount)
        } else {
            // make transfer
            Balance::transfer(from_account, transfer_args.to, amount, fee)
        }
        .map_err(|err| match err {
            FlyError::Balance(BalanceError::InsufficientBalance) => {
                icrc1_transfer::TransferError::InsufficientFunds {
                    balance: Self::icrc1_balance_of(from_account),
                }
            }
            _ => icrc1_transfer::TransferError::GenericError {
                error_code: Nat::from(3),
                message: err.to_string(),
            },
        })?;

        // register transaction
        let tx = Transaction {
            from: from_account,
            to: transfer_args.to,
            amount,
            fee,
            memo: transfer_args.memo,
            created_at: tx_created_at.as_nanos() as u64,
        };
        Register::insert_tx(tx).map_err(|_| icrc1_transfer::TransferError::GenericError {
            error_code: Nat::from(4),
            message: "failed to register transaction".to_string(),
        })
    }

    fn icrc1_supported_standards() -> Vec<icrc1::TokenExtension> {
        vec![
            icrc1::TokenExtension::icrc1(),
            icrc1::TokenExtension::icrc2(),
        ]
    }
}

#[cfg(test)]
mod test {

    use icrc::icrc1::transfer::TransferArg;
    use pretty_assertions::{assert_eq, assert_ne};

    use super::test_utils::{alice_account, bob_account, caller_account};
    use super::*;
    use crate::utils::{caller, fly_to_picofly};

    #[test]
    fn test_should_init_canister() {
        init_canister();

        assert_ne!(
            Configuration::get_minting_account().owner,
            Principal::anonymous()
        );
        assert_eq!(RolesManager::get_admins(), vec![caller()]);
        assert!(RolesManager::has_role(caller(), Role::DilazionatoCanister));
        // init balance
        assert_eq!(
            Balance::balance_of(alice_account()).unwrap(),
            fly_to_picofly(50_000)
        );
        assert_eq!(
            Balance::balance_of(bob_account()).unwrap(),
            fly_to_picofly(50_000)
        );
        assert_eq!(
            Balance::balance_of(caller_account()).unwrap(),
            fly_to_picofly(100_000)
        );
        // supply
        assert_eq!(
            Balance::balance_of(Balance::canister_wallet_account()).unwrap(),
            fly_to_picofly(8_688_888)
        );
    }

    #[test]
    fn test_should_reserve_pool() {
        init_canister();
        let contract_id = 1.into();
        let picofly_amount = 1000;

        let result =
            FlyCanister::reserve_pool(test_utils::caller_account(), contract_id, picofly_amount);

        assert_eq!(result, Ok(picofly_amount));
    }

    #[test]
    #[should_panic]
    fn test_should_not_allow_reserve_pool() {
        init_canister();
        let contract_id = 1.into();
        let picofly_amount = 1000;

        assert!(
            FlyCanister::reserve_pool(test_utils::bob_account(), contract_id, picofly_amount)
                .is_err()
        );
    }

    #[test]
    fn test_should_set_role() {
        init_canister();
        let principal = Principal::management_canister();
        let role = Role::Admin;
        FlyCanister::admin_set_role(principal, role);
        assert!(RolesManager::is_admin(principal));
    }

    #[test]
    fn test_should_remove_role() {
        init_canister();
        let principal = Principal::management_canister();
        let role = Role::Admin;
        FlyCanister::admin_set_role(principal, role);
        assert!(RolesManager::is_admin(principal));
        FlyCanister::admin_remove_role(principal, role).unwrap();
        assert!(!RolesManager::is_admin(principal));
    }

    #[test]
    fn test_should_get_name() {
        init_canister();
        assert_eq!(FlyCanister::icrc1_name(), ICRC1_NAME);
    }

    #[test]
    fn test_should_get_symbol() {
        init_canister();
        assert_eq!(FlyCanister::icrc1_symbol(), ICRC1_SYMBOL);
    }

    #[test]
    fn test_should_get_decimals() {
        init_canister();
        assert_eq!(FlyCanister::icrc1_decimals(), ICRC1_DECIMALS);
    }

    #[test]
    fn test_should_get_fee() {
        init_canister();
        assert_eq!(FlyCanister::icrc1_fee(), Nat::from(ICRC1_FEE));
    }

    #[test]
    fn test_should_get_metadata() {
        init_canister();
        let metadata = FlyCanister::icrc1_metadata();
        assert_eq!(metadata.len(), 5);
        assert_eq!(
            metadata.get(0).unwrap(),
            &(
                "icrc1:symbol".to_string(),
                MetadataValue::from(ICRC1_SYMBOL)
            )
        );
        assert_eq!(
            metadata.get(1).unwrap(),
            &("icrc1:name".to_string(), MetadataValue::from(ICRC1_NAME))
        );
        assert_eq!(
            metadata.get(2).unwrap(),
            &(
                "icrc1:decimals".to_string(),
                MetadataValue::from(Nat::from(ICRC1_DECIMALS))
            )
        );
        assert_eq!(
            metadata.get(3).unwrap(),
            &(
                "icrc1:fee".to_string(),
                MetadataValue::from(Nat::from(ICRC1_FEE))
            )
        );
        assert_eq!(
            metadata.get(4).unwrap(),
            &("icrc1:logo".to_string(), MetadataValue::from(ICRC1_LOGO))
        );
    }

    #[test]
    fn test_should_get_total_supply() {
        init_canister();
        assert_eq!(
            FlyCanister::icrc1_total_supply(),
            Nat::from(fly_to_picofly(8_888_888))
        );
    }

    #[test]
    fn test_should_get_minting_account() {
        init_canister();
        assert_eq!(
            FlyCanister::icrc1_minting_account(),
            Configuration::get_minting_account()
        );
    }

    #[test]
    fn test_should_get_balance_of() {
        init_canister();
        assert_eq!(
            FlyCanister::icrc1_balance_of(alice_account()),
            Nat::from(fly_to_picofly(50_000))
        );
        assert_eq!(
            FlyCanister::icrc1_balance_of(bob_account()),
            Nat::from(fly_to_picofly(50_000))
        );
        assert_eq!(
            FlyCanister::icrc1_balance_of(caller_account()),
            Nat::from(fly_to_picofly(100_000))
        );
        assert_eq!(
            FlyCanister::icrc1_balance_of(Account {
                owner: utils::id(),
                subaccount: Some(utils::random_subaccount()),
            }),
            Nat::from(0)
        );
    }

    #[test]
    fn test_should_transfer() {
        init_canister();
        let transfer_args = TransferArg {
            from_subaccount: caller_account().subaccount,
            to: bob_account(),
            amount: Nat::from(fly_to_picofly(10_000)),
            fee: Some(Nat::from(ICRC1_FEE)),
            created_at_time: Some(utils::time()),
            memo: None,
        };
        assert!(FlyCanister::icrc1_transfer(transfer_args).is_ok());
        assert_eq!(
            FlyCanister::icrc1_balance_of(caller_account()),
            Nat::from(fly_to_picofly(90_000) - ICRC1_FEE)
        );
        assert_eq!(
            FlyCanister::icrc1_balance_of(bob_account()),
            Nat::from(fly_to_picofly(60_000))
        );
    }

    #[test]
    fn test_should_not_transfer_with_bad_time() {
        init_canister();
        let transfer_args = TransferArg {
            from_subaccount: caller_account().subaccount,
            to: bob_account(),
            amount: Nat::from(fly_to_picofly(10_000)),
            fee: Some(Nat::from(ICRC1_FEE)),
            created_at_time: None,
            memo: None,
        };
        assert!(matches!(
            FlyCanister::icrc1_transfer(transfer_args).unwrap_err(),
            icrc1_transfer::TransferError::TooOld { .. }
        ));
    }

    #[test]
    fn test_should_not_transfer_with_old_time() {
        init_canister();
        let transfer_args = TransferArg {
            from_subaccount: caller_account().subaccount,
            to: bob_account(),
            amount: Nat::from(fly_to_picofly(10_000)),
            fee: Some(Nat::from(ICRC1_FEE)),
            created_at_time: Some(utils::time() - (ICRC1_TX_TIME_SKID.as_nanos() as u64 * 2)),
            memo: None,
        };
        assert!(matches!(
            FlyCanister::icrc1_transfer(transfer_args).unwrap_err(),
            icrc1_transfer::TransferError::TooOld { .. }
        ));
    }

    #[test]
    fn test_should_not_transfer_with_time_in_future() {
        init_canister();
        let transfer_args = TransferArg {
            from_subaccount: caller_account().subaccount,
            to: bob_account(),
            amount: Nat::from(fly_to_picofly(10_000)),
            fee: Some(Nat::from(ICRC1_FEE)),
            created_at_time: Some(utils::time() + (ICRC1_TX_TIME_SKID.as_nanos() as u64 * 2)),
            memo: None,
        };
        assert!(matches!(
            FlyCanister::icrc1_transfer(transfer_args).unwrap_err(),
            icrc1_transfer::TransferError::CreatedInFuture { .. }
        ));
    }

    #[test]
    fn test_should_not_transfer_with_bad_fee() {
        init_canister();
        let transfer_args = TransferArg {
            from_subaccount: caller_account().subaccount,
            to: bob_account(),
            amount: Nat::from(fly_to_picofly(10_000)),
            fee: Some(Nat::from(ICRC1_FEE / 2)),
            created_at_time: Some(utils::time()),
            memo: None,
        };

        assert!(matches!(
            FlyCanister::icrc1_transfer(transfer_args).unwrap_err(),
            icrc1_transfer::TransferError::BadFee { .. }
        ));
    }

    #[test]
    fn test_should_transfer_with_null_fee() {
        init_canister();
        let transfer_args = TransferArg {
            from_subaccount: caller_account().subaccount,
            to: bob_account(),
            amount: Nat::from(fly_to_picofly(10_000)),
            fee: None,
            created_at_time: Some(utils::time()),
            memo: None,
        };
        assert!(FlyCanister::icrc1_transfer(transfer_args).is_ok());
        assert_eq!(
            FlyCanister::icrc1_balance_of(caller_account()),
            Nat::from(fly_to_picofly(90_000) - ICRC1_FEE)
        );
    }

    #[test]
    fn test_should_transfer_with_higher_fee() {
        init_canister();
        let transfer_args = TransferArg {
            from_subaccount: caller_account().subaccount,
            to: bob_account(),
            amount: Nat::from(fly_to_picofly(10_000)),
            fee: Some(Nat::from(ICRC1_FEE * 2)),
            created_at_time: Some(utils::time()),
            memo: None,
        };
        assert!(FlyCanister::icrc1_transfer(transfer_args).is_ok());
        assert_eq!(
            FlyCanister::icrc1_balance_of(caller_account()),
            Nat::from(fly_to_picofly(90_000) - (ICRC1_FEE * 2))
        );
    }

    #[test]
    fn test_should_not_allow_bad_memo() {
        init_canister();
        let transfer_args = TransferArg {
            from_subaccount: caller_account().subaccount,
            to: bob_account(),
            amount: Nat::from(fly_to_picofly(10_000)),
            fee: None,
            created_at_time: Some(utils::time()),
            memo: Some("9888".as_bytes().to_vec().into()),
        };

        assert!(matches!(
            FlyCanister::icrc1_transfer(transfer_args).unwrap_err(),
            icrc1_transfer::TransferError::GenericError { .. }
        ));

        let transfer_args = TransferArg {
            from_subaccount: caller_account().subaccount,
            to: bob_account(),
            amount: Nat::from(fly_to_picofly(10_000)),
            fee: None,
            created_at_time: Some(utils::time()),
            memo: Some("988898889888988898889888988898889888988898889888988898889888988898889888988898889888988898889888".as_bytes().to_vec().into()),
        };

        assert!(matches!(
            FlyCanister::icrc1_transfer(transfer_args).unwrap_err(),
            icrc1_transfer::TransferError::GenericError { .. }
        ));
    }

    #[test]
    fn test_should_transfer_with_memo() {
        init_canister();
        let transfer_args = TransferArg {
            from_subaccount: caller_account().subaccount,
            to: bob_account(),
            amount: Nat::from(fly_to_picofly(10_000)),
            fee: Some(Nat::from(ICRC1_FEE)),
            created_at_time: Some(utils::time()),
            memo: Some(
                "293458234690283506958436839246024563"
                    .to_string()
                    .as_bytes()
                    .to_vec()
                    .into(),
            ),
        };
        assert!(FlyCanister::icrc1_transfer(transfer_args).is_ok());
        assert_eq!(
            FlyCanister::icrc1_balance_of(caller_account()),
            Nat::from(fly_to_picofly(90_000) - ICRC1_FEE)
        );
        assert_eq!(
            FlyCanister::icrc1_balance_of(bob_account()),
            Nat::from(fly_to_picofly(60_000))
        );
    }

    #[test]
    fn test_should_burn() {
        init_canister();
        let transfer_args = TransferArg {
            from_subaccount: caller_account().subaccount,
            to: FlyCanister::icrc1_minting_account(),
            amount: Nat::from(fly_to_picofly(10_000)),
            fee: None,
            created_at_time: Some(utils::time()),
            memo: None,
        };
        assert!(FlyCanister::icrc1_transfer(transfer_args).is_ok());
        assert_eq!(
            FlyCanister::icrc1_balance_of(caller_account()),
            Nat::from(fly_to_picofly(90_000))
        );
        assert_eq!(
            FlyCanister::icrc1_total_supply(),
            Nat::from(fly_to_picofly(8_888_888 - 10_000))
        );
    }

    #[test]
    fn test_should_get_supported_extensions() {
        init_canister();
        let extensions = FlyCanister::icrc1_supported_standards();
        assert_eq!(extensions.len(), 2);
        assert_eq!(
            extensions.get(0).unwrap().name,
            icrc1::TokenExtension::icrc1().name
        );
        assert_eq!(
            extensions.get(1).unwrap().name,
            icrc1::TokenExtension::icrc2().name
        );
    }

    fn init_canister() {
        let data = FlyInitData {
            admins: vec![caller()],
            total_supply: 8_888_888,
            dilazionato_canister: caller(),
            initial_balances: vec![
                (alice_account(), fly_to_picofly(50_000)),
                (bob_account(), fly_to_picofly(50_000)),
                (caller_account(), fly_to_picofly(100_000)),
            ],
        };
        FlyCanister::init(data);
    }
}
