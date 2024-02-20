use async_trait::async_trait;
use candid::Nat;
use icrc_ledger_types::icrc2::allowance::{Allowance, AllowanceArgs};
use icrc_ledger_types::icrc2::approve::{ApproveArgs, ApproveError};
use icrc_ledger_types::icrc2::transfer_from::{TransferFromArgs, TransferFromError};
pub use icrc_ledger_types::icrc2::*;

/// ICRC-1 methods
/// See: <https://github.com/dfinity/ICRC-1/blob/main/standards/ICRC-2/README.md>
#[async_trait]
pub trait Icrc2 {
    /// ## Description
    ///
    /// This method entitles the spender to transfer token amount on behalf of the caller
    /// from account { owner = caller; subaccount = from_subaccount }.
    /// The number of transfers the spender can initiate from the caller's account is unlimited as long as the
    /// total amounts and fees of these transfers do not exceed the allowance.
    /// The caller does not need to have the full token amount on the specified account for the approval to succeed,
    /// just enough tokens to pay the approval fee.
    /// The call resets the allowance and the expiration date for the spender account to the given values.
    ///
    /// The ledger SHOULD reject the call if the spender account owner is equal to the source account owner.
    /// If the expected_allowance field is set, the ledger MUST ensure that the current allowance for the spender
    /// from the caller's account is equal to the given value and return the AllowanceChanged error otherwise.
    ///
    /// The ledger MAY cap the allowance if it is too large (for example, larger than the total token supply).
    /// For example, if there are only 100 tokens, and the ledger receives an approval for 120 tokens,
    /// the ledger may cap the allowance to 100.
    ///
    /// ## Preconditions
    ///
    /// - The caller has enough fees on the { owner = caller; subaccount = from_subaccount } account to pay the approval fee.
    /// - If the expires_at field is set, it's greater than the current ledger time.
    /// - If the expected_allowance field is set, it's equal to the current allowance for the spender.
    ///
    /// ## Postconditions
    ///
    /// - The spender's allowance for the { owner = caller; subaccount = from_subaccount } is equal to the given amount.
    async fn icrc2_approve(args: ApproveArgs) -> Result<Nat, ApproveError>;

    /// ## Description
    ///
    /// Transfers a token amount from the from account to the to account using the allowance of the
    /// spender's account (SpenderAccount = { owner = caller; subaccount = spender_subaccount }).
    /// The ledger draws the fees from the from account.
    ///
    /// ## Preconditions
    ///
    /// - The allowance for the SpenderAccount from the from account is large enough to cover the transfer amount
    ///     and the fees (icrc2_allowance({ account = from; spender = SpenderAccount }).allowance >= amount + fee).
    ///     Otherwise, the ledger MUST return an InsufficientAllowance error.
    /// - The from account holds enough funds to cover the transfer amount and the fees.
    ///     (icrc1_balance_of(from) >= amount + fee).
    ///     Otherwise, the ledger MUST return an InsufficientFunds error.
    ///
    /// ## Postconditions
    ///
    /// - If the from account is not equal to the SpenderAccount, the (from, SpenderAccount)
    ///     allowance decreases by the transfer amount and the fees.
    /// - The ledger debited the specified amount of tokens and fees from the from account.
    /// - The ledger credited the specified amount to the to account.
    async fn icrc2_transfer_from(args: TransferFromArgs) -> Result<Nat, TransferFromError>;

    /// Returns the token allowance that the spender account can transfer from the specified account,
    /// and the expiration time for that allowance, if any.
    /// If there is no active approval, the ledger MUST return { allowance = 0; expires_at = null }.
    fn icrc2_allowance(args: AllowanceArgs) -> Allowance;
}
