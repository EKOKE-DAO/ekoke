use candid::{CandidType, Nat, Principal};
use did::ekoke::{EkokeError, EkokeResult};
use did::H160;
use icrc::icrc1::account::Account;
use icrc::IcrcLedgerClient;
use serde::Deserialize;

use crate::app::configuration::Configuration;
use crate::constants::ETH_MIN_WITHDRAWAL_AMOUNT;

pub struct CkEthWithdrawal;

impl CkEthWithdrawal {
    /// Withdraws current ckETH balance converting it to ETH and sending them to the ETH canister wallet.
    pub async fn withdraw_cketh() -> EkokeResult<()> {
        let cketh_client = IcrcLedgerClient::new(Configuration::get_cketh_ledger_canister());
        let erc20_swap_account = Configuration::get_erc20_swap_pool_account();
        // get current ckETH balance
        let cketh_balance = cketh_client
            .icrc1_balance_of(erc20_swap_account)
            .await
            .map_err(|(code, msg)| EkokeError::CanisterCall(code, msg))?;
        // check if balance is enough to withdraw
        if cketh_balance < ETH_MIN_WITHDRAWAL_AMOUNT {
            return Ok(());
        }
        // give allowance to the minter ledger of ckEth
        let minter_ledger = Configuration::get_cketh_minter_canister();
        cketh_client
            .icrc2_approve(
                Account::from(minter_ledger),
                cketh_balance.clone(),
                erc20_swap_account.subaccount,
            )
            .await
            .map_err(|(code, msg)| EkokeError::CanisterCall(code, msg))?
            .map_err(EkokeError::Icrc2Approve)?;

        // call withdraw
        let eth_canister_account = Configuration::get_erc20_bridge_address();
        Self::minter_ledger_withdraw_eth(minter_ledger, cketh_balance, eth_canister_account)
            .await
            .unwrap();

        Ok(())
    }

    async fn minter_ledger_withdraw_eth(
        minter_ledger: Principal,
        amount: Nat,
        recipient: H160,
    ) -> Result<WithdrawEthOk, WithdrawEthErr> {
        let args = WithdawEth {
            amount,
            recipient: recipient.to_hex_str(),
        };

        let result: (Result<WithdrawEthOk, WithdrawEthErr>,) =
            ic_cdk::call(minter_ledger, "withdraw_eth", (args,))
                .await
                .unwrap();

        result.0
    }
}

#[derive(CandidType)]
struct WithdawEth {
    amount: Nat,
    recipient: String,
}

#[derive(Debug, CandidType, Deserialize)]
struct WithdrawEthOk {
    block_index: Nat,
}

#[derive(Debug, CandidType, Deserialize)]
enum WithdrawEthErr {
    TemporarilyUnavailable(String),
    InsufficientAllowance { allowance: Nat },
    AmountTooLow { min_withdrawal_amount: Nat },
    RecipientAddressBlocked { address: String },
    InsufficientFunds { balance: Nat },
}
