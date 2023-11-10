//! # Sell Contract
//!
//! API for sell contract

mod configuration;
mod memory;
mod storage;

use async_trait::async_trait;
use candid::{Nat, Principal};
use configuration::Configuration;
use did::dilazionato::{
    Contract, ContractRegistration, SellContractError, SellContractInitData, SellContractResult,
    Token, TokenError,
};
use did::ID;
use dip721::{
    Dip721, GenericValue, Metadata, NftError, Stats, SupportedInterface, TokenIdentifier,
    TokenMetadata, TxEvent,
};

use self::storage::TxHistory;
use crate::app::storage::Storage;
use crate::client::FlyClient;
use crate::utils::caller;

#[derive(Default)]
/// Sell contract canister API
pub struct SellContract;

impl SellContract {
    /// On init set custodians and canisters ids
    pub fn init(init_data: SellContractInitData) {
        Configuration::set_canister_custodians(&init_data.custodians).expect("storage error");
        Configuration::set_fly_canister(init_data.fly_canister).expect("storage error");
        Configuration::set_marketplace_canister(init_data.marketplace_canister)
            .expect("storage error");
    }

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

    /// Returns whether caller is owner or operator of the token
    pub fn inspect_is_owner_or_operator(token_identifier: &Nat) -> Result<Token, NftError> {
        let token = match Storage::get_token(token_identifier) {
            Some(token) => token,
            None => return Err(NftError::TokenNotFound),
        };

        let owner = match token.owner {
            Some(owner) => owner,
            None => return Err(NftError::UnauthorizedOwner),
        };

        if caller() != owner && Some(caller()) != token.operator {
            return Err(NftError::UnauthorizedOperator);
        }

        Ok(token)
    }

    /// Inspect burn, allow burn only if caller is owner or operator and token is owned by a buyer or a seller.
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

        if !contract.buyers.contains(&owner) && owner != contract.seller {
            return Err(NftError::Other(
                "owner is not nor a buyer nor the seller".to_string(),
            ));
        }
        if caller() != owner && Some(caller()) != token.operator {
            return Err(NftError::UnauthorizedOperator);
        }

        Ok(())
    }

    /// Inspect register contract parameters
    pub fn inspect_register_contract(
        id: &ID,
        value: u64,
        installments: u64,
        expiration: &str,
    ) -> SellContractResult<()> {
        if !Self::inspect_is_custodian() {
            return Err(SellContractError::Unauthorized);
        }
        // check if contract already exists
        if Self::get_contract(id).is_some() {
            return Err(SellContractError::Token(TokenError::ContractAlreadyExists(
                id.clone(),
            )));
        }

        // verify value must be multiple of installments
        if value % installments != 0 {
            return Err(SellContractError::Token(
                TokenError::ContractValueIsNotMultipleOfInstallments,
            ));
        }

        // check if expiration is YYYY-MM-DD and is not in the past
        match crate::utils::parse_date(expiration) {
            Ok(timestamp) if timestamp < crate::utils::time() => {
                return Err(SellContractError::Token(TokenError::InvalidExpirationDate));
            }
            Ok(_) => {}
            Err(_) => return Err(SellContractError::Token(TokenError::InvalidExpirationDate)),
        }

        Ok(())
    }

    pub fn inspect_is_buyer(contract: ID) -> SellContractResult<()> {
        let contract = match Storage::get_contract(&contract) {
            Some(contract) => contract,
            None => {
                return Err(SellContractError::Token(TokenError::ContractNotFound(
                    contract,
                )))
            }
        };

        if contract.buyers.contains(&caller()) {
            Ok(())
        } else {
            Err(SellContractError::Unauthorized)
        }
    }

    /// get contract by id
    pub fn get_contract(id: &ID) -> Option<Contract> {
        Storage::get_contract(id)
    }

    /// get contracts ids
    pub fn get_contracts() -> Vec<ID> {
        Storage::get_contracts()
    }

    /// Register contract inside of the canister.
    /// Only a custodian can call this method.
    pub async fn admin_register_contract(data: ContractRegistration) -> SellContractResult<()> {
        Self::inspect_register_contract(&data.id, data.value, data.installments, &data.expiration)?;

        // get reward for contract
        let mfly_reward = FlyClient::from(Configuration::get_fly_canister())
            .get_contract_reward(data.id.clone(), data.installments)
            .await?;

        // make tokens
        let next_token_id = Storage::total_supply();
        let mut tokens = Vec::with_capacity(data.installments as usize);
        let mut tokens_ids = Vec::with_capacity(data.installments as usize);
        let token_value: u64 = data.value / data.installments;
        let marketplace_canister = Configuration::get_marketplace_canister();

        for token_id in next_token_id..next_token_id + data.installments {
            tokens.push(Token {
                approved_at: Some(crate::utils::time()),
                approved_by: Some(caller()),
                burned_at: None,
                burned_by: None,
                contract_id: data.id.clone(),
                id: token_id.into(),
                is_burned: false,
                minted_at: crate::utils::time(),
                minted_by: caller(),
                operator: Some(marketplace_canister), // * the operator must be the marketplace canister
                owner: Some(data.seller),
                transferred_at: None,
                transferred_by: None,
                value: token_value,
            });
            tokens_ids.push(token_id.into());
        }

        // make contract
        let contract = Contract {
            building: data.building,
            buyers: data.buyers,
            expiration: data.expiration,
            id: data.id.clone(),
            mfly_reward,
            seller: data.seller,
            tokens: tokens_ids,
            value: data.value,
            initial_value: data.value,
            currency: data.currency,
        };

        // register contract
        Storage::insert_contract(contract, tokens)?;

        Ok(())
    }

    /// Update marketplace canister id and update the operator for all the tokens
    pub fn admin_set_marketplace_canister(canister: Principal) {
        if !Self::inspect_is_custodian() {
            ic_cdk::trap("Unauthorized");
        }

        if let Err(err) = Configuration::set_marketplace_canister(canister) {
            ic_cdk::trap(&err.to_string());
        }

        // update tokens
        if let Err(err) = Storage::update_tokens_operator(canister) {
            ic_cdk::trap(&err.to_string());
        }
    }

    /// Update fly canister id
    pub fn admin_set_fly_canister(canister: Principal) {
        if !Self::inspect_is_custodian() {
            ic_cdk::trap("Unauthorized");
        }

        if let Err(err) = Configuration::set_fly_canister(canister) {
            ic_cdk::trap(&err.to_string());
        }
    }

    /// Update contract buyers. Only the buyer can call this method.
    pub fn update_contract_buyers(
        contract_id: ID,
        buyers: Vec<Principal>,
    ) -> SellContractResult<()> {
        Self::inspect_is_buyer(contract_id.clone())?;
        Storage::update_contract_buyers(&contract_id, buyers)
    }
}

#[async_trait]
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
    fn cycles() -> Nat {
        ic_cdk::api::canister_balance().into()
    }

    /// Returns total unique holders of tokens
    fn total_unique_holders() -> Nat {
        Storage::total_unique_holders().into()
    }

    /// Returns metadata for token
    fn token_metadata(token_identifier: TokenIdentifier) -> Result<TokenMetadata, NftError> {
        let token = match Storage::get_token(&token_identifier) {
            Some(token) => token,
            None => return Err(NftError::TokenNotFound),
        };

        Ok(token.into())
    }

    /// Returns the count of NFTs owned by user.
    /// If the user does not own any NFTs, returns an error containing NftError.
    fn balance_of(owner: Principal) -> Result<Nat, NftError> {
        match Storage::tokens_by_owner(owner) {
            tokens if tokens.is_empty() => Err(NftError::OwnerNotFound),
            tokens => Ok(tokens.len().into()),
        }
    }

    /// Returns the owner of the token.
    /// Returns an error containing NftError if token_identifier is invalid.
    fn owner_of(token_identifier: TokenIdentifier) -> Result<Option<Principal>, NftError> {
        match Storage::get_token(&token_identifier).map(|token| token.owner) {
            Some(owner) => Ok(owner),
            None => Err(NftError::TokenNotFound),
        }
    }

    /// Returns the list of the token_identifier of the NFT associated with owner.
    /// Returns an error containing NftError if principal is invalid.
    fn owner_token_identifiers(owner: Principal) -> Result<Vec<TokenIdentifier>, NftError> {
        Ok(Storage::tokens_by_owner(owner))
    }

    /// Returns the list of the token_metadata of the NFT associated with owner.
    /// Returns an error containing NftError if principal is invalid.
    fn owner_token_metadata(owner: Principal) -> Result<Vec<TokenMetadata>, NftError> {
        let tokens = Self::owner_token_identifiers(owner)?;
        let mut metadata = Vec::with_capacity(tokens.len());
        for token in tokens {
            metadata.push(Self::token_metadata(token)?);
        }

        Ok(metadata)
    }

    /// Returns the Principal of the operator of the NFT associated with token_identifier.
    fn operator_of(token_identifier: TokenIdentifier) -> Result<Option<Principal>, NftError> {
        match Storage::get_token(&token_identifier) {
            Some(token) => Ok(token.operator),
            None => Err(NftError::TokenNotFound),
        }
    }

    /// Returns the list of the token_identifier of the NFT associated with operator.
    fn operator_token_identifiers(operator: Principal) -> Result<Vec<TokenIdentifier>, NftError> {
        Ok(Storage::tokens_by_operator(operator))
    }

    /// Returns the list of the token_metadata of the NFT associated with operator.
    fn operator_token_metadata(operator: Principal) -> Result<Vec<TokenMetadata>, NftError> {
        let tokens = Self::operator_token_identifiers(operator)?;
        let mut metadata = Vec::with_capacity(tokens.len());
        for token in tokens {
            metadata.push(Self::token_metadata(token)?);
        }

        Ok(metadata)
    }

    /// Returns the list of the interfaces supported by this canister
    fn supported_interfaces() -> Vec<SupportedInterface> {
        vec![
            SupportedInterface::Burn,
            SupportedInterface::TransactionHistory,
        ]
    }

    /// Returns the total supply of the NFT.
    /// NFTs that are minted and later burned explicitly or sent to the zero address should also count towards totalSupply.
    fn total_supply() -> Nat {
        Storage::total_supply().into()
    }

    // Calling approve grants the operator the ability to make update calls to the specificied token_identifier.
    // Approvals given by the approve function are independent from approvals given by the setApprovalForAll.
    //
    // If the approval goes through, returns a nat that represents the CAP History transaction ID that can be used at the transaction method.
    /// Interface: approval
    fn approve(_operator: Principal, _token_identifier: TokenIdentifier) -> Result<Nat, NftError> {
        Err(NftError::Other("Not implemented".to_string()))
    }

    /// Enable or disable an operator to manage all of the tokens for the caller of this function. The contract allows multiple operators per owner.
    /// Approvals granted by the approve function are independent from the approvals granted by setApprovalForAll function.
    /// If the approval goes through, returns a nat that represents the CAP History transaction ID that can be used at the transaction method.
    /// Interface: approval
    fn set_approval_for_all(_operator: Principal, _approved: bool) -> Result<Nat, NftError> {
        Err(NftError::Other("Not implemented".to_string()))
    }

    /// Returns true if the given operator is an approved operator for all the tokens owned by the caller through the use of the setApprovalForAll method, returns false otherwise.
    /// Interface: approval
    fn is_approved_for_all(_owner: Principal, _operator: Principal) -> Result<bool, NftError> {
        Err(NftError::Other("Not implemented".to_string()))
    }

    /// Sends the callers nft token_identifier to `to`` and returns a nat that represents a
    /// transaction id that can be used at the transaction method.
    async fn transfer(to: Principal, token_identifier: TokenIdentifier) -> Result<Nat, NftError> {
        Self::transfer_from(caller(), to, token_identifier).await
    }

    /// Caller of this method is able to transfer the NFT token_identifier that is in from's balance to to's balance
    /// if the caller is an approved operator to do so.
    ///
    /// If the transfer goes through, returns a nat that represents the CAP History transaction ID
    /// that can be used at the transaction method.
    async fn transfer_from(
        owner: Principal,
        to: Principal,
        token_identifier: TokenIdentifier,
    ) -> Result<Nat, NftError> {
        let token = Self::inspect_is_owner_or_operator(&token_identifier)?;
        let last_owner = token.owner;
        let contract = match Storage::get_contract(&token.contract_id) {
            Some(contract) => contract,
            None => return Err(NftError::TokenNotFound),
        };
        // verify that from owner is the same as the token's
        if token.owner != Some(owner) {
            return Err(NftError::OwnerNotFound);
        }
        // verify that owner is not the same as to
        if token.owner == Some(to) {
            return Err(NftError::SelfTransfer);
        }

        // transfer token to the new owner
        let tx_id = match Storage::transfer(&token_identifier, to) {
            Ok(tx_id) => tx_id,
            Err(SellContractError::Token(TokenError::TokenNotFound(_))) => {
                return Err(NftError::TokenNotFound)
            }
            Err(_) => return Err(NftError::UnauthorizedOperator),
        };

        // if the previous owner, was the seller, notify fly canister to transfer reward to the new owner
        if last_owner == Some(contract.seller) {
            FlyClient::from(Configuration::get_fly_canister())
                .send_reward(token.contract_id, contract.mfly_reward, to)
                .await
                .map_err(|_| NftError::Other("fly canister error".to_string()))?;
        }

        Ok(tx_id)
    }

    fn mint(
        _to: Principal,
        _token_identifier: TokenIdentifier,
        _properties: Vec<(String, GenericValue)>,
    ) -> Result<Nat, NftError> {
        Err(NftError::Other("Not implemented".to_string()))
    }

    /// Burn an NFT identified by token_identifier. Calling burn on a token sets the owner to None and
    /// will no longer be useable.
    /// Burned tokens do still count towards totalSupply.
    /// Implementations are encouraged to only allow burning by the owner of the token_identifier.
    ///
    /// The burn will also reduce the contract value by the token value
    fn burn(token_identifier: TokenIdentifier) -> Result<Nat, NftError> {
        Self::inspect_burn(&token_identifier)?;

        match Storage::burn_token(&token_identifier) {
            Ok(tx_id) => Ok(tx_id),
            Err(SellContractError::Token(TokenError::TokenNotFound(_))) => {
                Err(NftError::TokenNotFound)
            }
            Err(_) => Err(NftError::UnauthorizedOperator),
        }
    }

    /// Returns the TxEvent that corresponds with tx_id.
    /// If there is no TxEvent that corresponds with the tx_id entered, returns a NftError.TxNotFound.
    fn transaction(tx_id: Nat) -> Result<TxEvent, NftError> {
        match TxHistory::get_transaction_by_id(tx_id) {
            Some(ev) => Ok(ev),
            None => Err(NftError::TxNotFound),
        }
    }

    /// Returns a nat that represents the total number of transactions that have occurred on the NFT canister.
    fn total_transactions() -> Nat {
        TxHistory::count().into()
    }
}
