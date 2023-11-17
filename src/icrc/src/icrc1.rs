use candid::Nat;
use icrc_ledger_types::icrc1::transfer::TransferError;
pub use icrc_ledger_types::icrc1::*;

use self::account::Account;
use self::transfer::TransferArg;
use super::icrc::generic_metadata_value::MetadataValue;

/// ICRC-1 methods
/// See: <https://github.com/dfinity/ICRC-1/blob/main/standards/ICRC-1/README.md>
pub trait Icrc1 {
    /// Returns the name of the token (e.g., MyToken).
    fn icrc1_name() -> &'static str;

    /// Returns the symbol of the token (e.g., ICP).
    fn icrc1_symbol() -> &'static str;

    /// Returns the number of decimals the token uses
    /// (e.g., 8 means to divide the token amount by 100000000 to get its user representation).
    fn icrc1_decimals() -> u8;

    /// Returns the default transfer fee.
    fn icrc1_fee() -> Nat;

    /// Returns the list of metadata entries for this ledger. See the "Metadata" section below.
    fn icrc1_metadata() -> Vec<(String, MetadataValue)>;

    /// Returns the total number of tokens on all accounts except for the minting account.
    fn icrc1_total_supply() -> Nat;

    /// Returns the minting account if this ledger supports minting and burning tokens.
    fn icrc1_minting_account() -> Account;

    /// Returns the balance of the account given as an argument.
    fn icrc1_balance_of(account: Account) -> Nat;

    /// Transfers amount of tokens from account record { of = caller; subaccount = from_subaccount } to the to account.
    ///
    /// ## Fee
    ///
    /// The caller pays fee tokens for the transfer.
    /// If the caller does not set the fee argument, the ledger applies the default transfer fee.
    /// If the fee argument does not agree with the ledger fee,
    /// the ledger MUST return variant { BadFee = record { expected_fee = ... } } error.
    ///
    /// ## Memo
    ///
    /// The memo parameter is an arbitrary blob that has no meaning to the ledger.
    /// The ledger SHOULD allow memos of at least 32 bytes in length.
    /// The ledger SHOULD use the memo argument for transaction deduplication.
    ///
    /// ## Created at time
    ///
    /// The created_at_time parameter indicates the time (as nanoseconds since the UNIX epoch in the UTC timezone)
    /// at which the client constructed the transaction.
    /// The ledger SHOULD reject transactions that have created_at_time argument too far in the past or the future,
    /// returning variant { TooOld } and variant { CreatedInFuture = record { ledger_time = ... } } errors correspondingly.
    ///
    /// ## Transaction ID
    ///
    /// The result is either the transaction index of the transfer or an error.
    fn icrc1_transfer(transfer_args: TransferArg) -> Result<Nat, TransferError>;

    /// Returns the list of standards this ledger implements.
    fn icrc1_supported_standards() -> Vec<TokenExtension>;
}

pub struct TokenExtension {
    pub name: String,
    pub url: String,
}

impl TokenExtension {
    /// Returns extension for icrc-1
    pub fn icrc1() -> Self {
        Self {
            name: "ICRC-1".to_string(),
            url: "https://github.com/dfinity/ICRC-1".to_string(),
        }
    }

    /// Returns extension for icrc-2
    pub fn icrc2() -> Self {
        Self {
            name: "ICRC-2".to_string(),
            url: "https://github.com/dfinity/ICRC-1".to_string(),
        }
    }
}
