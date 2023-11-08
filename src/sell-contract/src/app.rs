//! # Sell Contract
//!
//! API for sell contract

mod configuration;
mod memory;
mod storage;

use candid::{Nat, Principal};
use configuration::Configuration;
use did::sell_contract::{SellContractError, TokenError};
use dip721::{
    Dip721, GenericValue, Metadata, NftError, Stats, SupportedInterface, TokenIdentifier,
    TokenMetadata, TxEvent,
};

use self::storage::TxHistory;
use crate::app::storage::Storage;
use crate::utils::caller;

#[derive(Default)]
/// Sell contract canister API
pub struct SellContract;

impl SellContract {
    /// Task to execute on init
    pub fn init() {}

    /// Task to execute on post upgrade
    pub fn post_upgrade() {
        // update upgraded at timestamp
        if let Err(err) = Configuration::set_upgraded_at() {
            ic_cdk::trap(&err.to_string());
        }
    }

    /// Returns whether caller is custodian of the canister
    pub fn inspect_is_custodian() -> bool {
        Configuration::is_custodian(caller())
    }

    /// Inspect burn, allow burn only if caller is custodian or operator and token is owned by a buyer
    pub fn inspect_burn(token_identifier: &Nat) -> Result<(), NftError> {
        let token = match Storage::get_token(token_identifier) {
            Some(token) => token,
            None => return Err(NftError::TokenNotFound),
        };
        let contract = match Storage::get_contract(&token.contract_id) {
            Some(contract) => contract,
            None => return Err(NftError::TokenNotFound),
        };
        let owner = match token.owner {
            Some(owner) => owner,
            None => return Err(NftError::UnauthorizedOwner),
        };

        if !contract.buyers.contains(&owner) {
            return Err(NftError::UnauthorizedOperator);
        }
        if caller() != owner && caller() != owner {
            return Err(NftError::UnauthorizedOperator);
        }

        Ok(())
    }
}

impl Dip721 for SellContract {
    /// Returns the Metadata of the NFT canister which includes custodians, logo, name, symbol.
    fn metadata() -> Metadata {
        Metadata {
            created_at: Configuration::get_created_at(),
            custodians: Self::custodians(),
            logo: Self::logo(),
            name: Self::name(),
            symbol: Self::symbol(),
            upgraded_at: Configuration::get_upgraded_at(),
        }
    }

    /// Returns the Stats of the NFT canister which includes cycles, totalSupply, totalTransactions, totalUniqueHolders.
    fn stats() -> Stats {
        Stats {
            cycles: Self::cycles(),
            total_supply: Self::total_supply(),
            total_transactions: Self::total_transactions(),
            total_unique_holders: Self::total_unique_holders(),
        }
    }

    /// Returns the logo of the NFT contract as Base64 encoded text.
    fn logo() -> Option<String> {
        Configuration::get_logo()
    }

    /// Sets the logo of the NFT canister. Base64 encoded text is recommended.
    /// Caller must be the custodian of NFT canister.
    fn set_logo(logo: String) {
        if !Self::inspect_is_custodian() {
            ic_cdk::trap("Unauthorized");
        }
        if let Err(err) = Configuration::set_logo(logo) {
            ic_cdk::trap(&err.to_string());
        }
    }

    /// Returns the name of the NFT canister.
    fn name() -> Option<String> {
        Configuration::get_name()
    }

    /// Sets the name of the NFT contract.
    /// Caller must be the custodian of NFT canister.
    fn set_name(name: String) {
        if !Self::inspect_is_custodian() {
            ic_cdk::trap("Unauthorized");
        }
        if let Err(err) = Configuration::set_name(name) {
            ic_cdk::trap(&err.to_string());
        }
    }

    /// Returns the symbol of the NFT contract.
    fn symbol() -> Option<String> {
        Configuration::get_symbol()
    }

    /// Set symbol
    /// Caller must be the custodian of NFT canister.
    fn set_symbol(symbol: String) {
        if !Self::inspect_is_custodian() {
            ic_cdk::trap("Unauthorized");
        }
        if let Err(err) = Configuration::set_symbol(symbol) {
            ic_cdk::trap(&err.to_string());
        }
    }

    /// Returns a list of the canister custodians
    fn custodians() -> Vec<Principal> {
        Configuration::get_canister_custodians()
    }

    /// Set canister custodians
    /// Caller must be the custodian of NFT canister.
    fn set_custodians(custodians: Vec<Principal>) {
        if !Self::inspect_is_custodian() {
            ic_cdk::trap("Unauthorized");
        }
        if let Err(err) = Configuration::set_canister_custodians(&custodians) {
            ic_cdk::trap(&err.to_string());
        }
    }

    /// Returns canister cycles
    fn cycles() -> candid::Nat {
        ic_cdk::api::canister_balance().into()
    }

    /// Returns total unique holders of tokens
    fn total_unique_holders() -> candid::Nat {
        todo!()
    }

    /// Returns metadata for token
    fn token_metadata(token_identifier: TokenIdentifier) -> Result<TokenMetadata, NftError> {
        todo!()
    }

    /// Returns the balance of the owner.
    fn balance_of(owner: Principal) -> Result<candid::Nat, NftError> {
        todo!()
    }

    /// Returns the owner of the token.
    fn owner_of(token_identifier: TokenIdentifier) -> Result<Principal, NftError> {
        todo!()
    }

    /// Returns the list of the token_identifier of the NFT associated with owner.
    /// Returns an error containing NftError if principal is invalid.
    fn owner_token_identifiers(owner: Principal) -> Result<Vec<TokenIdentifier>, NftError> {
        todo!()
    }

    /// Returns the list of the token_metadata of the NFT associated with owner.
    /// Returns an error containing NftError if principal is invalid.
    fn owner_token_metadata(owner: Principal) -> Result<Vec<TokenMetadata>, NftError> {
        todo!()
    }

    /// Returns the Principal of the operator of the NFT associated with token_identifier.
    fn operator_of(token_identifier: TokenIdentifier) -> Result<Principal, NftError> {
        todo!()
    }

    /// Returns the list of the token_identifier of the NFT associated with operator.
    fn operator_token_identifiers(operator: Principal) -> Result<Vec<TokenIdentifier>, NftError> {
        todo!()
    }

    /// Returns the list of the token_metadata of the NFT associated with operator.
    fn operator_token_metadata(operator: Principal) -> Result<Vec<TokenMetadata>, NftError> {
        todo!()
    }

    /// Returns the list of the interfaces supported by this canister
    fn supported_interfaces() -> Vec<SupportedInterface> {
        vec![
            SupportedInterface::Approval,
            SupportedInterface::Burn,
            SupportedInterface::TransactionHistory,
        ]
    }

    /// Returns the total supply of the NFT.
    /// NFTs that are minted and later burned explicitly or sent to the zero address should also count towards totalSupply.
    fn total_supply() -> candid::Nat {
        todo!()
    }

    // Calling approve grants the operator the ability to make update calls to the specificied token_identifier.
    // Approvals given by the approve function are independent from approvals given by the setApprovalForAll.
    //
    // If the approval goes through, returns a nat that represents the CAP History transaction ID that can be used at the transaction method.
    /// Interface: approval
    fn approve(
        operator: Principal,
        token_identifier: TokenIdentifier,
    ) -> Result<candid::Nat, NftError> {
        todo!()
    }

    /// Enable or disable an operator to manage all of the tokens for the caller of this function. The contract allows multiple operators per owner.
    /// Approvals granted by the approve function are independent from the approvals granted by setApprovalForAll function.
    /// If the approval goes through, returns a nat that represents the CAP History transaction ID that can be used at the transaction method.
    /// Interface: approval
    fn set_approval_for_all(operator: Principal, approved: bool) -> Result<candid::Nat, NftError> {
        todo!()
    }

    /// Returns true if the given operator is an approved operator for all the tokens owned by the caller through the use of the setApprovalForAll method, returns false otherwise.
    /// Interface: approval
    fn is_approved_for_all(owner: Principal, operator: Principal) -> Result<bool, NftError> {
        todo!()
    }

    /// Caller of this method is able to transfer the NFT token_identifier that is in from's balance to to's balance if the caller is an approved operator to do so.
    ///
    /// If the transfer goes through, returns a nat that represents the CAP History transaction ID that can be used at the transaction method.
    fn transfer_from(
        owner: Principal,
        to: Principal,
        token_identifier: TokenIdentifier,
    ) -> Result<candid::Nat, NftError> {
        todo!()
    }

    fn mint(
        _to: Principal,
        _token_identifier: TokenIdentifier,
        _properties: Vec<(String, GenericValue)>,
    ) -> Result<candid::Nat, NftError> {
        Err(NftError::Other("Not implemented".to_string()))
    }

    /// Burn an NFT identified by token_identifier. Calling burn on a token sets the owner to None and will no longer be useable.
    /// Burned tokens do still count towards totalSupply.
    /// Implementations are encouraged to only allow burning by the owner of the token_identifier.
    fn burn(token_identifier: TokenIdentifier) -> Result<candid::Nat, NftError> {
        Self::inspect_burn(&token_identifier)?;

        match Storage::burn_token(&token_identifier) {
            Ok(()) => Ok(token_identifier),
            Err(SellContractError::Token(TokenError::TokenNotFound(_))) => {
                Err(NftError::TokenNotFound)
            }
            Err(_) => Err(NftError::UnauthorizedOperator),
        }
    }

    /// Returns the TxEvent that corresponds with tx_id.
    /// If there is no TxEvent that corresponds with the tx_id entered, returns a NftError.TxNotFound.
    fn transaction(tx_id: candid::Nat) -> Result<TxEvent, NftError> {
        match TxHistory::get_transaction_by_id(tx_id) {
            Some(ev) => Ok(ev),
            None => Err(NftError::TxNotFound),
        }
    }

    /// Returns a nat that represents the total number of transactions that have occurred on the NFT canister.
    fn total_transactions() -> candid::Nat {
        TxHistory::count().into()
    }
}
