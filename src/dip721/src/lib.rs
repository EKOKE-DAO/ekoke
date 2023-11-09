//! # DIP721
//!
//! DIP 721 interface and types for Canisters

mod data;

use async_trait::async_trait;
use candid::{Nat, Principal};
pub use data::{
    GenericValue, Metadata, NftError, Stats, SupportedInterface, TokenIdentifier, TokenMetadata,
    TxEvent,
};

#[async_trait]
/// Represents the method a DIP721 canister must implement
pub trait Dip721 {
    /// Returns the Metadata of the NFT canister which includes custodians, logo, name, symbol.
    fn metadata() -> Metadata;

    /// Returns the Stats of the NFT canister which includes cycles, totalSupply, totalTransactions, totalUniqueHolders.
    fn stats() -> Stats;

    /// Returns the logo of the NFT contract as Base64 encoded text.
    fn logo() -> Option<String>;

    /// Sets the logo of the NFT canister. Base64 encoded text is recommended.
    /// Caller must be the custodian of NFT canister.
    fn set_logo(logo: String);

    /// Returns the name of the NFT canister.
    fn name() -> Option<String>;

    /// Sets the name of the NFT contract.
    /// Caller must be the custodian of NFT canister.
    fn set_name(name: String);

    /// Returns the symbol of the NFT contract.
    fn symbol() -> Option<String>;

    /// Set symbol
    /// Caller must be the custodian of NFT canister.
    fn set_symbol(symbol: String);

    /// Returns a list of the canister custodians
    fn custodians() -> Vec<Principal>;

    /// Set canister custodians
    /// Caller must be the custodian of NFT canister.
    fn set_custodians(custodians: Vec<Principal>);

    /// Returns canister cycles
    fn cycles() -> Nat;

    /// Returns total unique holders of tokens
    fn total_unique_holders() -> Nat;

    /// Returns metadata for token
    fn token_metadata(token_identifier: TokenIdentifier) -> Result<TokenMetadata, NftError>;

    /// Returns the count of NFTs owned by user.
    /// If the user does not own any NFTs, returns an error containing NftError.
    fn balance_of(owner: Principal) -> Result<Nat, NftError>;

    /// Returns the owner of the token.
    /// Returns an error containing NftError if token_identifier is invalid.
    fn owner_of(token_identifier: TokenIdentifier) -> Result<Option<Principal>, NftError>;

    /// Returns the list of the token_identifier of the NFT associated with owner.
    /// Returns an error containing NftError if principal is invalid.
    fn owner_token_identifiers(owner: Principal) -> Result<Vec<TokenIdentifier>, NftError>;

    /// Returns the list of the token_metadata of the NFT associated with owner.
    /// Returns an error containing NftError if principal is invalid.
    fn owner_token_metadata(owner: Principal) -> Result<Vec<TokenMetadata>, NftError>;

    /// Returns the Principal of the operator of the NFT associated with token_identifier.
    fn operator_of(token_identifier: TokenIdentifier) -> Result<Option<Principal>, NftError>;

    /// Returns the list of the token_identifier of the NFT associated with operator.
    fn operator_token_identifiers(operator: Principal) -> Result<Vec<TokenIdentifier>, NftError>;

    /// Returns the list of the token_metadata of the NFT associated with operator.
    fn operator_token_metadata(operator: Principal) -> Result<Vec<TokenMetadata>, NftError>;

    /// Returns the list of the interfaces supported by this canister
    fn supported_interfaces() -> Vec<SupportedInterface>;

    /// Returns the total supply of the NFT.
    /// NFTs that are minted and later burned explicitly or sent to the zero address should also count towards totalSupply.
    fn total_supply() -> Nat;

    // Calling approve grants the operator the ability to make update calls to the specificied token_identifier.
    // Approvals given by the approve function are independent from approvals given by the setApprovalForAll.
    //
    // If the approval goes through, returns a nat that represents the CAP History transaction ID that can be used at the transaction method.
    /// Interface: approval
    fn approve(operator: Principal, token_identifier: TokenIdentifier) -> Result<Nat, NftError>;

    /// Enable or disable an operator to manage all of the tokens for the caller of this function. The contract allows multiple operators per owner.
    /// Approvals granted by the approve function are independent from the approvals granted by setApprovalForAll function.
    /// If the approval goes through, returns a nat that represents the CAP History transaction ID that can be used at the transaction method.
    /// Interface: approval
    fn set_approval_for_all(operator: Principal, approved: bool) -> Result<Nat, NftError>;

    /// Returns true if the given operator is an approved operator for all the tokens owned by the caller through the use of the setApprovalForAll method, returns false otherwise.
    /// Interface: approval
    fn is_approved_for_all(owner: Principal, operator: Principal) -> Result<bool, NftError>;

    /// Sends the callers nft token_identifier to `to`` and returns a nat that represents a
    /// transaction id that can be used at the transaction method.
    async fn transfer(to: Principal, token_identifier: TokenIdentifier) -> Result<Nat, NftError>;

    /// Caller of this method is able to transfer the NFT token_identifier that is in from's balance to to's balance if the caller is an approved operator to do so.
    ///
    /// If the transfer goes through, returns a nat that represents the CAP History transaction ID that can be used at the transaction method.
    async fn transfer_from(
        owner: Principal,
        to: Principal,
        token_identifier: TokenIdentifier,
    ) -> Result<Nat, NftError>;

    /// Mint an NFT for principal to that has an ID of token_identifier and metadata akin to properties. Implementations are encouraged to only allow minting by the owner of the canister.
    /// If the mint goes through, returns a nat that represents the CAP History transaction ID that can be used at the transaction method.
    ///
    /// Interface: mint
    fn mint(
        to: Principal,
        token_identifier: TokenIdentifier,
        properties: Vec<(String, GenericValue)>,
    ) -> Result<Nat, NftError>;

    /// Burn an NFT identified by token_identifier. Calling burn on a token sets the owner to None and will no longer be useable. Burned tokens do still count towards totalSupply.
    /// Implementations are encouraged to only allow burning by the owner of the token_identifier.
    fn burn(token_identifier: TokenIdentifier) -> Result<Nat, NftError>;

    /// Returns the TxEvent that corresponds with tx_id.
    /// If there is no TxEvent that corresponds with the tx_id entered, returns a NftError.TxNotFound.
    fn transaction(tx_id: Nat) -> Result<TxEvent, NftError>;

    /// Returns a nat that represents the total number of transactions that have occurred on the NFT canister.
    fn total_transactions() -> Nat;
}
