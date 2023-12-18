//! # Inspect
//!
//! Deferred inspect message handler

use std::time::Duration;

use candid::{Nat, Principal};
use did::fly::Role;
use did::ID;
use icrc::icrc1::account::Account;
use icrc::icrc1::transfer::{TransferArg, TransferError};
use icrc::icrc2::approve::{ApproveArgs, ApproveError};
use icrc::icrc2::transfer_from::{TransferFromArgs, TransferFromError};

use super::pool::Pool;
use super::roles::RolesManager;
use crate::constants::{ICRC1_FEE, ICRC1_TX_TIME_SKID};
use crate::utils::time;

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

    /// Returns whether caller is marketplace canister
    pub fn inspect_is_marketplace_canister(caller: Principal) -> bool {
        RolesManager::has_role(caller, Role::MarketplaceCanister)
    }

    /// Returns whether caller is owner of the wallet
    pub fn inspect_caller_owns_wallet(caller: Principal, account: Account) -> bool {
        caller == account.owner
    }

    /// inspect whether transfer update is valid
    pub fn inspect_transfer(args: &TransferArg) -> Result<(), TransferError> {
        let fee = args.fee.clone().unwrap_or(ICRC1_FEE.into());
        if fee < ICRC1_FEE {
            return Err(TransferError::BadFee {
                expected_fee: ICRC1_FEE.into(),
            });
        }

        // check if the transaction is too old
        let now = Duration::from_nanos(time());
        let tx_created_at =
            Duration::from_nanos(args.created_at_time.unwrap_or(now.as_nanos() as u64));
        if now > tx_created_at && now.saturating_sub(tx_created_at) > ICRC1_TX_TIME_SKID {
            return Err(TransferError::TooOld);
        } else if tx_created_at.saturating_sub(now) > ICRC1_TX_TIME_SKID {
            return Err(TransferError::CreatedInFuture {
                ledger_time: now.as_nanos() as u64,
            });
        }

        // check memo length
        if let Some(memo) = &args.memo {
            if memo.0.len() < 32 || memo.0.len() > 64 {
                return Err(TransferError::GenericError {
                    error_code: Nat::from(1),
                    message: "Invalid memo length. I must have a length between 32 and 64 bytes"
                        .to_string(),
                });
            }
        }

        Ok(())
    }

    /// inspect icrc2 approve arguments
    pub fn inspect_icrc2_approve(
        caller: Principal,
        args: &ApproveArgs,
    ) -> Result<(), ApproveError> {
        if args.spender.owner == caller {
            return Err(ApproveError::GenericError {
                error_code: 0_u64.into(),
                message: "Spender and owner cannot be equal".to_string(),
            });
        }
        if args
            .fee
            .as_ref()
            .map(|fee| fee < &ICRC1_FEE)
            .unwrap_or(false)
        {
            return Err(ApproveError::BadFee {
                expected_fee: ICRC1_FEE.into(),
            });
        }
        // check if expired
        if args
            .expires_at
            .as_ref()
            .map(|expiry| expiry < &time())
            .unwrap_or(false)
        {
            return Err(ApproveError::Expired {
                ledger_time: time(),
            });
        }
        // check if too old or in the future
        if let Some(created_at) = args.created_at_time {
            let current_time = Duration::from_nanos(time());
            let created_at = Duration::from_nanos(created_at);

            if created_at > current_time {
                return Err(ApproveError::CreatedInFuture {
                    ledger_time: current_time.as_nanos() as u64,
                });
            }

            if current_time - created_at > Duration::from_secs(300) {
                return Err(ApproveError::TooOld);
            }
        }

        Ok(())
    }

    pub fn inspect_icrc2_transfer_from(args: &TransferFromArgs) -> Result<(), TransferFromError> {
        // check fee
        if args
            .fee
            .as_ref()
            .map(|fee| fee < &ICRC1_FEE)
            .unwrap_or(false)
        {
            return Err(TransferFromError::BadFee {
                expected_fee: ICRC1_FEE.into(),
            });
        }

        // check if too old or in the future
        if let Some(created_at) = args.created_at_time {
            let current_time = Duration::from_nanos(time());
            let created_at = Duration::from_nanos(created_at);

            if created_at > current_time {
                return Err(TransferFromError::CreatedInFuture {
                    ledger_time: current_time.as_nanos() as u64,
                });
            }

            if current_time - created_at > Duration::from_secs(300) {
                return Err(TransferFromError::TooOld);
            }
        }

        // check memo length
        if let Some(memo) = &args.memo {
            if memo.0.len() < 32 || memo.0.len() > 64 {
                return Err(TransferFromError::GenericError {
                    error_code: Nat::from(0),
                    message: "Invalid memo length. I must have a length between 32 and 64 bytes"
                        .to_string(),
                });
            }
        }

        Ok(())
    }

    /// inspect whether pool exists
    pub fn inspect_pool_exists(contract_id: &ID) -> bool {
        Pool::has_pool(contract_id)
    }
}

#[cfg(test)]
mod test {

    use icrc::icrc1::transfer::Memo;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::app::balance::Balance;
    use crate::app::test_utils::{self, alice_account};

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

    #[test]
    fn test_should_inspect_transfer() {
        let args = TransferArg {
            from_subaccount: None,
            to: test_utils::bob_account(),
            amount: 100.into(),
            fee: Some((ICRC1_FEE - 1).into()),
            memo: None,
            created_at_time: None,
        };

        assert!(Inspect::inspect_transfer(&args).is_err());

        let args = TransferArg {
            from_subaccount: None,
            to: test_utils::bob_account(),
            amount: 100.into(),
            fee: Some(ICRC1_FEE.into()),
            memo: None,
            created_at_time: None,
        };

        assert!(Inspect::inspect_transfer(&args).is_ok());

        let args = TransferArg {
            from_subaccount: None,
            to: test_utils::bob_account(),
            amount: 100.into(),
            fee: None,
            memo: None,
            created_at_time: None,
        };

        assert!(Inspect::inspect_transfer(&args).is_ok());

        let args = TransferArg {
            from_subaccount: None,
            to: test_utils::bob_account(),
            amount: 100.into(),
            fee: None,
            memo: Some(Memo::from(vec![0; 31])),
            created_at_time: None,
        };

        assert!(Inspect::inspect_transfer(&args).is_err());

        let args = TransferArg {
            from_subaccount: None,
            to: test_utils::bob_account(),
            amount: 100.into(),
            fee: None,
            memo: Some(Memo::from(vec![0; 65])),
            created_at_time: None,
        };

        assert!(Inspect::inspect_transfer(&args).is_err());

        let args = TransferArg {
            from_subaccount: None,
            to: test_utils::bob_account(),
            amount: 100.into(),
            fee: None,
            memo: Some(Memo::from(vec![0; 32])),
            created_at_time: None,
        };

        assert!(Inspect::inspect_transfer(&args).is_ok());

        let args = TransferArg {
            from_subaccount: None,
            to: test_utils::bob_account(),
            amount: 100.into(),
            fee: None,
            memo: Some(Memo::from(vec![0; 64])),
            created_at_time: None,
        };

        assert!(Inspect::inspect_transfer(&args).is_ok());
    }

    #[test]
    fn test_should_inspect_icrc2_approve() {
        let caller = Principal::from_text("aaaaa-aa").unwrap();
        let args = ApproveArgs {
            spender: test_utils::alice_account(),
            amount: 100.into(),
            fee: None,
            expires_at: None,
            created_at_time: None,
            memo: None,
            from_subaccount: None,
            expected_allowance: None,
        };

        assert!(Inspect::inspect_icrc2_approve(caller, &args).is_ok());

        let args = ApproveArgs {
            spender: test_utils::alice_account(),
            amount: 100.into(),
            fee: Some((ICRC1_FEE - 1).into()),
            expires_at: None,
            created_at_time: None,
            memo: None,
            from_subaccount: None,
            expected_allowance: None,
        };

        assert!(Inspect::inspect_icrc2_approve(caller, &args).is_err());

        let args = ApproveArgs {
            spender: test_utils::alice_account(),
            amount: 100.into(),
            fee: None,
            expires_at: None,
            created_at_time: Some(0),
            memo: None,
            from_subaccount: None,
            expected_allowance: None,
        };

        assert!(Inspect::inspect_icrc2_approve(caller, &args).is_err());

        let args = ApproveArgs {
            spender: test_utils::alice_account(),
            amount: 100.into(),
            fee: None,
            expires_at: Some(0),
            created_at_time: None,
            memo: None,
            from_subaccount: None,
            expected_allowance: None,
        };

        assert!(Inspect::inspect_icrc2_approve(caller, &args).is_err());

        let args = ApproveArgs {
            spender: test_utils::alice_account(),
            amount: 100.into(),
            fee: None,
            expires_at: None,
            created_at_time: Some(crate::utils::time() * 2),
            memo: None,
            from_subaccount: None,
            expected_allowance: None,
        };

        assert!(Inspect::inspect_icrc2_approve(caller, &args).is_err());
    }

    #[test]
    fn test_should_inspect_transfer_from() {
        let args = TransferFromArgs {
            spender_subaccount: None,
            from: test_utils::alice_account(),
            to: test_utils::bob_account(),
            amount: 100.into(),
            fee: Some((ICRC1_FEE - 1).into()),
            memo: None,
            created_at_time: None,
        };

        assert!(Inspect::inspect_icrc2_transfer_from(&args).is_err());

        let args = TransferFromArgs {
            spender_subaccount: None,
            from: test_utils::alice_account(),
            to: test_utils::bob_account(),
            amount: 100.into(),
            fee: Some(ICRC1_FEE.into()),
            memo: None,
            created_at_time: None,
        };

        assert!(Inspect::inspect_icrc2_transfer_from(&args).is_ok());

        let args = TransferFromArgs {
            spender_subaccount: None,
            from: test_utils::alice_account(),
            to: test_utils::bob_account(),
            amount: 100.into(),
            fee: None,
            memo: None,
            created_at_time: None,
        };

        assert!(Inspect::inspect_icrc2_transfer_from(&args).is_ok());

        let args = TransferFromArgs {
            spender_subaccount: None,
            from: test_utils::alice_account(),
            to: test_utils::bob_account(),
            amount: 100.into(),
            fee: None,
            memo: Some(Memo::from(vec![0; 31])),
            created_at_time: None,
        };

        assert!(Inspect::inspect_icrc2_transfer_from(&args).is_err());

        let args = TransferFromArgs {
            spender_subaccount: None,
            from: test_utils::alice_account(),
            to: test_utils::bob_account(),
            amount: 100.into(),
            fee: None,
            memo: Some(Memo::from(vec![0; 65])),
            created_at_time: None,
        };

        assert!(Inspect::inspect_icrc2_transfer_from(&args).is_err());

        let args = TransferFromArgs {
            spender_subaccount: None,
            from: test_utils::alice_account(),
            to: test_utils::bob_account(),
            amount: 100.into(),
            fee: None,
            memo: Some(Memo::from(vec![0; 32])),
            created_at_time: None,
        };

        assert!(Inspect::inspect_icrc2_transfer_from(&args).is_ok());
    }

    #[tokio::test]
    async fn test_should_inspect_pool_exists() {
        let contract_id = ID::from(0);
        assert!(!Inspect::inspect_pool_exists(&contract_id));

        Balance::init_balances(50_000.into(), vec![(alice_account(), 100.into())]);
        assert!(
            Pool::reserve(&contract_id, test_utils::alice_account(), 100.into())
                .await
                .is_ok()
        );

        assert!(Inspect::inspect_pool_exists(&contract_id));
    }
}
