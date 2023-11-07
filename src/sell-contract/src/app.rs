//! # Sell Contract
//!
//! API for sell contract

mod configuration;
mod memory;
mod storage;

use candid::Principal;
use configuration::Configuration;
use dip721::{
    Dip721, GenericValue, Metadata, NftError, Stats, SupportedInterface, TokenIdentifier,
    TokenMetadata, TxEvent,
};

use self::storage::TxHistory;

#[derive(Default)]
/// Sell contract canister API
pub struct SellContract;

impl SellContract {
    /// Returns whether caller is custodian of the canister
    pub fn is_custodian(caller: Principal) -> bool {
        Configuration::is_custodian(caller)
    }
}

impl Dip721 for SellContract {
    fn metadata() -> Metadata {
        todo!()
    }

    fn stats() -> Stats {
        todo!()
    }

    fn logo() -> Option<String> {
        todo!()
    }

    fn set_logo(logo: String) {
        todo!()
    }

    fn name() -> Option<String> {
        todo!()
    }

    fn set_name(name: String) {
        todo!()
    }

    fn symbol() -> Option<String> {
        todo!()
    }

    fn custodians() -> Vec<Principal> {
        todo!()
    }

    fn set_custodians(custodians: Vec<Principal>) {
        todo!()
    }

    fn cycles() -> candid::Nat {
        todo!()
    }

    fn total_unique_holders() -> candid::Nat {
        todo!()
    }

    fn token_metadata(token_identifier: TokenIdentifier) -> Result<TokenMetadata, NftError> {
        todo!()
    }

    fn balance_of(owner: Principal) -> Result<candid::Nat, NftError> {
        todo!()
    }

    fn owner_of(token_identifier: TokenIdentifier) -> Result<Principal, NftError> {
        todo!()
    }

    fn owner_token_identifiers(owner: Principal) -> Result<Vec<TokenIdentifier>, NftError> {
        todo!()
    }

    fn owner_token_metadata(owner: Principal) -> Result<Vec<TokenMetadata>, NftError> {
        todo!()
    }

    fn operator_of(token_identifier: TokenIdentifier) -> Result<Principal, NftError> {
        todo!()
    }

    fn operator_token_identifiers(operator: Principal) -> Result<Vec<TokenIdentifier>, NftError> {
        todo!()
    }

    fn operator_token_metadata(operator: Principal) -> Result<Vec<TokenMetadata>, NftError> {
        todo!()
    }

    fn supported_interfaces() -> Vec<SupportedInterface> {
        vec![
            SupportedInterface::Approval,
            SupportedInterface::Burn,
            SupportedInterface::Mint,
            SupportedInterface::TransactionHistory,
        ]
    }

    fn total_supply() -> candid::Nat {
        todo!()
    }

    fn approve(
        operator: Principal,
        token_identifier: TokenIdentifier,
    ) -> Result<candid::Nat, NftError> {
        todo!()
    }

    fn set_approval_for_all(operator: Principal, approved: bool) -> Result<candid::Nat, NftError> {
        todo!()
    }

    fn is_approved_for_all(owner: Principal, operator: Principal) -> Result<bool, NftError> {
        todo!()
    }

    fn transfer_from(
        owner: Principal,
        to: Principal,
        token_identifier: TokenIdentifier,
    ) -> Result<candid::Nat, NftError> {
        todo!()
    }

    fn mint(
        to: Principal,
        token_identifier: TokenIdentifier,
        properties: Vec<(String, GenericValue)>,
    ) -> Result<candid::Nat, NftError> {
        todo!()
    }

    fn burn(token_identifier: TokenIdentifier) -> Result<candid::Nat, NftError> {
        todo!()
    }

    fn transaction(tx_id: candid::Nat) -> Result<Vec<TxEvent>, NftError> {
        todo!()
    }

    fn total_transactions() -> candid::Nat {
        TxHistory::count().into()
    }
}
