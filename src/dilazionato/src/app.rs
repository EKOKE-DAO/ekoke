//! # Sell Contract
//!
//! API for sell contract

mod configuration;
mod inspect;
mod memory;
mod minter;
mod roles;
pub mod storage;
#[cfg(test)]
mod test_utils;

use async_trait::async_trait;
use candid::{Nat, Principal};
use configuration::Configuration;
use did::dilazionato::{
    Contract, ContractRegistration, DilazionatoError, DilazionatoInitData, DilazionatoResult, Role,
    TokenError,
};
use did::ID;
use dip721::{
    Dip721, GenericValue, Metadata, NftError, Stats, SupportedInterface, TokenIdentifier,
    TokenMetadata, TxEvent,
};

pub use self::inspect::Inspect;
use self::minter::Minter;
use self::roles::RolesManager;
use self::storage::{ContractStorage, TxHistory};
use crate::client::{fly_client, FlyClient};
use crate::utils::caller;

#[derive(Default)]
/// Sell contract canister API
pub struct Dilazionato;

impl Dilazionato {
    /// On init set custodians and canisters ids
    pub fn init(init_data: DilazionatoInitData) {
        RolesManager::set_custodians(init_data.custodians).expect("storage error");
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

    /// get contract by id
    pub fn get_contract(id: &ID) -> Option<Contract> {
        ContractStorage::get_contract(id)
    }

    /// get contracts ids
    pub fn get_contracts() -> Vec<ID> {
        ContractStorage::get_contracts()
    }

    /// Update contract buyers. Only the buyer can call this method.
    pub fn update_contract_buyers(
        contract_id: ID,
        buyers: Vec<Principal>,
    ) -> DilazionatoResult<()> {
        Inspect::inspect_is_buyer(caller(), contract_id.clone())?;
        ContractStorage::update_contract_buyers(&contract_id, buyers)
    }

    /// Increment contract value. Only the seller can call this method.
    pub async fn seller_increment_contract_value(
        contract_id: ID,
        incr_by: u64,
        installments: u64,
    ) -> DilazionatoResult<()> {
        Inspect::inspect_is_seller(caller(), contract_id.clone())?;

        // mint new tokens
        let (tokens, _) = Minter::mint(&contract_id, installments, incr_by).await?;

        // update contract
        ContractStorage::add_tokens_to_contract(
            &contract_id,
            tokens,
            Configuration::get_marketplace_canister(),
        )
    }

    /// Register contract inside of the canister.
    /// Only a custodian can call this method.
    pub async fn register_contract(data: ContractRegistration) -> DilazionatoResult<()> {
        Inspect::inspect_register_contract(
            caller(),
            &data.id,
            data.value,
            data.installments,
            &data.expiration,
        )?;

        let (tokens, tokens_ids) = Minter::mint(&data.id, data.installments, data.value).await?;

        // make contract
        let contract = Contract {
            buyers: data.buyers,
            currency: data.currency,
            expiration: data.expiration,
            is_signed: false, // MUST BE NOT SIGNED
            id: data.id.clone(),
            initial_value: data.value,
            properties: data.properties,
            r#type: data.r#type,
            seller: data.seller,
            tokens: tokens_ids,
            value: data.value,
        };

        // register contract
        ContractStorage::insert_contract(contract, tokens)?;

        Ok(())
    }

    /// Sign provided contract
    pub fn admin_sign_contract(contract_id: ID) -> DilazionatoResult<()> {
        if !Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized");
        }

        ContractStorage::sign_contract(&contract_id, Configuration::get_marketplace_canister())
    }

    /// Update marketplace canister id and update the operator for all the tokens
    pub fn admin_set_marketplace_canister(canister: Principal) {
        if !Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized");
        }

        if let Err(err) = Configuration::set_marketplace_canister(canister) {
            ic_cdk::trap(&err.to_string());
        }

        // update tokens
        if let Err(err) = ContractStorage::update_tokens_operator(canister) {
            ic_cdk::trap(&err.to_string());
        }
    }

    /// Update fly canister id
    pub fn admin_set_fly_canister(canister: Principal) {
        if !Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized");
        }

        if let Err(err) = Configuration::set_fly_canister(canister) {
            ic_cdk::trap(&err.to_string());
        }
    }

    /// Give role to the provied principal
    pub fn admin_set_role(principal: Principal, role: Role) {
        if !Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized");
        }
        RolesManager::give_role(principal, role);
    }

    /// Remove role from principal.
    ///
    /// Fails if trying to remove the only custodian of the canister
    pub fn admin_remove_role(principal: Principal, role: Role) -> DilazionatoResult<()> {
        if !Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized");
        }

        RolesManager::remove_role(principal, role)
    }
}

#[async_trait]
impl Dip721 for Dilazionato {
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
        if !Inspect::inspect_is_custodian(caller()) {
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
        if !Inspect::inspect_is_custodian(caller()) {
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
        if !Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized");
        }
        if let Err(err) = Configuration::set_symbol(symbol) {
            ic_cdk::trap(&err.to_string());
        }
    }

    /// Returns a list of the canister custodians
    fn custodians() -> Vec<Principal> {
        RolesManager::get_custodians()
    }

    /// Set canister custodians
    /// Caller must be the custodian of NFT canister.
    fn set_custodians(custodians: Vec<Principal>) {
        if !Inspect::inspect_is_custodian(caller()) {
            ic_cdk::trap("Unauthorized");
        }
        if let Err(err) = RolesManager::set_custodians(custodians) {
            ic_cdk::trap(&err.to_string());
        }
    }

    /// Returns canister cycles
    fn cycles() -> Nat {
        crate::utils::cycles()
    }

    /// Returns total unique holders of tokens
    fn total_unique_holders() -> Nat {
        ContractStorage::total_unique_holders().into()
    }

    /// Returns metadata for token
    fn token_metadata(token_identifier: TokenIdentifier) -> Result<TokenMetadata, NftError> {
        let token = match ContractStorage::get_token(&token_identifier) {
            Some(token) => token,
            None => return Err(NftError::TokenNotFound),
        };

        Ok(token.into())
    }

    /// Returns the count of NFTs owned by user.
    /// If the user does not own any NFTs, returns an error containing NftError.
    fn balance_of(owner: Principal) -> Result<Nat, NftError> {
        match ContractStorage::tokens_by_owner(owner) {
            tokens if tokens.is_empty() => Err(NftError::OwnerNotFound),
            tokens => Ok(tokens.len().into()),
        }
    }

    /// Returns the owner of the token.
    /// Returns an error containing NftError if token_identifier is invalid.
    fn owner_of(token_identifier: TokenIdentifier) -> Result<Option<Principal>, NftError> {
        match ContractStorage::get_token(&token_identifier).map(|token| token.owner) {
            Some(owner) => Ok(owner),
            None => Err(NftError::TokenNotFound),
        }
    }

    /// Returns the list of the token_identifier of the NFT associated with owner.
    /// Returns an error containing NftError if principal is invalid.
    fn owner_token_identifiers(owner: Principal) -> Result<Vec<TokenIdentifier>, NftError> {
        match ContractStorage::tokens_by_owner(owner) {
            tokens if tokens.is_empty() => Err(NftError::OwnerNotFound),
            tokens => Ok(tokens),
        }
    }

    /// Returns the list of the token_metadata of the NFT associated with owner.
    /// Returns an error containing NftError if principal is invalid.
    fn owner_token_metadata(owner: Principal) -> Result<Vec<TokenMetadata>, NftError> {
        let tokens = Self::owner_token_identifiers(owner)?;
        let mut metadata = Vec::with_capacity(tokens.len());
        for token in tokens {
            metadata.push(Self::token_metadata(token)?);
        }

        if metadata.is_empty() {
            return Err(NftError::OwnerNotFound);
        }

        Ok(metadata)
    }

    /// Returns the Principal of the operator of the NFT associated with token_identifier.
    fn operator_of(token_identifier: TokenIdentifier) -> Result<Option<Principal>, NftError> {
        match ContractStorage::get_token(&token_identifier) {
            Some(token) => Ok(token.operator),
            None => Err(NftError::TokenNotFound),
        }
    }

    /// Returns the list of the token_identifier of the NFT associated with operator.
    fn operator_token_identifiers(operator: Principal) -> Result<Vec<TokenIdentifier>, NftError> {
        match ContractStorage::tokens_by_operator(operator) {
            tokens if tokens.is_empty() => Err(NftError::OperatorNotFound),
            tokens => Ok(tokens),
        }
    }

    /// Returns the list of the token_metadata of the NFT associated with operator.
    fn operator_token_metadata(operator: Principal) -> Result<Vec<TokenMetadata>, NftError> {
        let tokens = Self::operator_token_identifiers(operator)?;
        let mut metadata = Vec::with_capacity(tokens.len());
        for token in tokens {
            metadata.push(Self::token_metadata(token)?);
        }

        if metadata.is_empty() {
            return Err(NftError::OperatorNotFound);
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
        ContractStorage::total_supply().into()
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
        let token = Inspect::inspect_is_owner_or_operator(caller(), &token_identifier)?;
        let last_owner = token.owner;
        let contract = match ContractStorage::get_contract(&token.contract_id) {
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
        let tx_id = match ContractStorage::transfer(&token_identifier, to) {
            Ok(tx_id) => tx_id,
            Err(DilazionatoError::Token(TokenError::TokenNotFound(_))) => {
                return Err(NftError::TokenNotFound)
            }
            Err(_) => return Err(NftError::UnauthorizedOperator),
        };

        // if the previous owner, was the seller, notify fly canister to transfer reward to the new owner
        if last_owner == Some(contract.seller) {
            fly_client(Configuration::get_fly_canister())
                .send_reward(token.contract_id, token.mfly_reward, to)
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
        Inspect::inspect_burn(caller(), &token_identifier)?;

        match ContractStorage::burn_token(&token_identifier) {
            Ok(tx_id) => Ok(tx_id),
            Err(DilazionatoError::Token(TokenError::TokenNotFound(_))) => {
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

#[cfg(test)]
mod test {

    use std::time::Duration;

    use pretty_assertions::assert_eq;

    use super::test_utils::store_mock_contract;
    use super::*;
    use crate::app::test_utils::{alice, mock_token, store_mock_contract_with};
    use crate::constants::{DEFAULT_LOGO, DEFAULT_NAME, DEFAULT_SYMBOL};

    #[test]
    fn test_should_init_canister() {
        Dilazionato::init(DilazionatoInitData {
            custodians: vec![caller()],
            fly_canister: caller(),
            marketplace_canister: caller(),
        });

        assert_eq!(Dilazionato::custodians(), vec![caller()]);
        assert_eq!(Configuration::get_fly_canister(), caller());
        assert_eq!(Configuration::get_marketplace_canister(), caller());
    }

    #[test]
    fn test_should_set_upgrade_time_on_post_upgrade() {
        init_canister();
        let metadata = Dilazionato::metadata();
        assert!(metadata.upgraded_at == metadata.created_at);
        std::thread::sleep(Duration::from_millis(100));
        Dilazionato::post_upgrade();
        let metadata = Dilazionato::metadata();
        assert!(metadata.upgraded_at > metadata.created_at);
    }

    #[test]
    fn test_should_get_contract() {
        init_canister();
        store_mock_contract(&[1, 2], 1);
        assert_eq!(
            Dilazionato::get_contract(&1.into()).unwrap().id,
            Nat::from(1)
        );
    }

    #[test]
    fn test_should_get_contracts() {
        init_canister();
        store_mock_contract(&[1, 2], 1);
        store_mock_contract(&[3, 4], 2);
        assert_eq!(
            Dilazionato::get_contracts(),
            vec![Nat::from(1), Nat::from(2)]
        );
    }

    #[tokio::test]
    async fn test_should_register_contract() {
        init_canister();
        let contract = ContractRegistration {
            buyers: vec![caller()],
            currency: "EUR".to_string(),
            expiration: "2040-01-01".to_string(),
            id: 1.into(),
            installments: 10,
            properties: vec![],
            r#type: did::dilazionato::ContractType::Financing,
            seller: caller(),
            value: 100,
        };
        assert!(Dilazionato::register_contract(contract).await.is_ok());
        assert_eq!(Dilazionato::get_contracts(), vec![Nat::from(1)]);
        assert_eq!(Dilazionato::total_supply(), Nat::from(10));
    }

    #[tokio::test]
    async fn test_should_sign_contract() {
        init_canister();
        assert!(Configuration::set_marketplace_canister(alice()).is_ok());
        let contract = ContractRegistration {
            buyers: vec![caller()],
            currency: "EUR".to_string(),
            expiration: "2040-01-01".to_string(),
            id: 1.into(),
            installments: 10,
            properties: vec![],
            r#type: did::dilazionato::ContractType::Financing,
            seller: caller(),
            value: 100,
        };
        assert!(Dilazionato::register_contract(contract).await.is_ok());
        assert!(Dilazionato::admin_sign_contract(1.into()).is_ok());
        assert_eq!(
            Dilazionato::get_contract(&1.into()).unwrap().is_signed,
            true
        );
        // verify operator
        assert_eq!(Dilazionato::operator_of(1.into()).unwrap(), Some(alice()));
    }

    #[tokio::test]
    async fn test_should_increment_contract_value() {
        init_canister();
        let contract = ContractRegistration {
            buyers: vec![caller()],
            currency: "EUR".to_string(),
            expiration: "2040-01-01".to_string(),
            id: 1.into(),
            installments: 10,
            properties: vec![],
            r#type: did::dilazionato::ContractType::Financing,
            seller: caller(),
            value: 100,
        };
        assert!(Dilazionato::register_contract(contract).await.is_ok());
        assert!(Dilazionato::admin_sign_contract(1.into()).is_ok());

        // increment value
        assert!(
            Dilazionato::seller_increment_contract_value(1.into(), 50, 10)
                .await
                .is_ok()
        );
        assert_eq!(Dilazionato::total_supply(), Nat::from(20));
    }

    #[test]
    fn test_should_update_contract_buyers() {
        init_canister();
        store_mock_contract_with(
            &[1, 2],
            1,
            |contract| {
                contract.buyers = vec![caller(), Principal::management_canister()];
            },
            |_| {},
        );
        assert!(Dilazionato::update_contract_buyers(
            1.into(),
            vec![Principal::management_canister(), caller()]
        )
        .is_ok());
        assert_eq!(
            Dilazionato::get_contract(&1.into()).unwrap().buyers,
            vec![Principal::management_canister(), caller()]
        );
    }

    #[test]
    fn test_should_set_role() {
        init_canister();
        let principal = Principal::management_canister();
        let role = Role::Custodian;
        Dilazionato::admin_set_role(principal, role);
        assert!(RolesManager::is_custodian(principal));
    }

    #[test]
    fn test_should_remove_role() {
        init_canister();
        let principal = Principal::management_canister();
        let role = Role::Custodian;
        Dilazionato::admin_set_role(principal, role);
        assert!(RolesManager::is_custodian(principal));
        Dilazionato::admin_remove_role(principal, role).unwrap();
        assert!(!RolesManager::is_custodian(principal));
    }

    #[test]
    fn test_should_get_metadata() {
        init_canister();
        let metadata = Dilazionato::metadata();
        assert_eq!(metadata.custodians, vec![caller()]);
        assert_eq!(metadata.logo.as_deref(), Some(DEFAULT_LOGO));
        assert_eq!(metadata.name.as_deref(), Some(DEFAULT_NAME));
        assert_eq!(metadata.symbol.as_deref(), Some(DEFAULT_SYMBOL));
    }

    #[test]
    fn test_should_get_stats() {
        init_canister();
        let stats = Dilazionato::stats();
        assert_eq!(stats.cycles, crate::utils::cycles());
        assert_eq!(stats.total_supply, 0);
        assert_eq!(stats.total_transactions, 0);
        assert_eq!(stats.total_unique_holders, 0);
    }

    #[test]
    fn test_should_set_logo() {
        init_canister();
        let logo = "logo";
        Dilazionato::set_logo(logo.to_string());
        assert_eq!(Dilazionato::logo().as_deref(), Some(logo));
    }

    #[test]
    fn test_should_set_name() {
        init_canister();
        let name = "name";
        Dilazionato::set_name(name.to_string());
        assert_eq!(Dilazionato::name().as_deref(), Some(name));
    }

    #[test]
    fn test_should_set_symbol() {
        init_canister();
        let symbol = "symbol";
        Dilazionato::set_symbol(symbol.to_string());
        assert_eq!(Dilazionato::symbol().as_deref(), Some(symbol));
    }

    #[test]
    fn test_should_set_custodians() {
        init_canister();
        let custodians = vec![caller(), Principal::management_canister()];
        Dilazionato::set_custodians(custodians.clone());
        assert_eq!(Dilazionato::custodians().len(), custodians.len());
    }

    #[test]
    fn test_should_get_cycles() {
        init_canister();
        assert_eq!(Dilazionato::cycles(), crate::utils::cycles());
    }

    #[test]
    fn test_should_get_unique_holders() {
        init_canister();
        store_mock_contract(&[1, 2], 1);
        assert_eq!(Dilazionato::total_unique_holders(), Nat::from(1));
    }

    #[test]
    fn test_should_get_token_metadata() {
        init_canister();
        store_mock_contract(&[1, 2], 1);
        let metadata = Dilazionato::token_metadata(1.into()).unwrap();
        assert_eq!(metadata.owner, Some(caller()));
        assert_eq!(metadata.token_identifier, Nat::from(1));

        // unexisting token
        assert!(Dilazionato::token_metadata(5.into()).is_err());
    }

    #[test]
    fn test_should_get_balance_of() {
        init_canister();
        store_mock_contract(&[1, 2], 1);
        assert_eq!(Dilazionato::balance_of(caller()).unwrap(), Nat::from(2));
        assert!(Dilazionato::balance_of(Principal::management_canister()).is_err());
    }

    #[test]
    fn test_should_get_owner_of() {
        init_canister();
        store_mock_contract(&[1, 2], 1);
        assert_eq!(Dilazionato::owner_of(1.into()).unwrap(), Some(caller()));
        assert!(Dilazionato::owner_of(5.into()).is_err());
    }

    #[test]
    fn test_should_get_owner_token_identifiers() {
        init_canister();
        store_mock_contract(&[1, 2], 1);
        assert_eq!(
            Dilazionato::owner_token_identifiers(caller()).unwrap(),
            vec![Nat::from(1), Nat::from(2)]
        );
        assert!(Dilazionato::owner_token_identifiers(Principal::management_canister()).is_err());
    }

    #[test]
    fn test_should_get_owner_token_metadata() {
        init_canister();
        store_mock_contract(&[1, 2], 1);
        let metadata = Dilazionato::owner_token_metadata(caller()).unwrap();
        assert_eq!(metadata.len(), 2);
        assert_eq!(metadata[0].owner, Some(caller()));
        assert_eq!(metadata[0].token_identifier, Nat::from(1));
        assert_eq!(metadata[1].owner, Some(caller()));
        assert_eq!(metadata[1].token_identifier, Nat::from(2));

        // unexisting owner
        assert!(Dilazionato::owner_token_metadata(Principal::management_canister()).is_err());
    }

    #[test]
    fn test_should_get_operator_of() {
        init_canister();
        store_mock_contract(&[1, 2], 1);

        assert_eq!(Dilazionato::operator_of(1.into()).unwrap(), Some(alice()));

        assert!(Dilazionato::operator_of(5.into()).is_err());
    }

    #[test]
    fn test_should_get_operator_token_identifiers() {
        init_canister();
        // no owner
        store_mock_contract_with(&[1, 2], 1, |contract| contract.seller = alice(), |_| {});
        assert!(Dilazionato::operator_token_identifiers(caller()).is_err());

        // with operator
        assert!(Configuration::set_marketplace_canister(Principal::management_canister()).is_ok());
        store_mock_contract_with(&[3, 4], 2, |contract| contract.seller = alice(), |_| {});
        assert_eq!(
            Dilazionato::operator_token_identifiers(Principal::management_canister()).unwrap(),
            vec![Nat::from(3), Nat::from(4)]
        );
        assert!(Dilazionato::operator_of(5.into()).is_err());
    }

    #[test]
    fn test_should_get_operator_token_metadata() {
        init_canister();
        assert!(Dilazionato::operator_token_metadata(Principal::anonymous()).is_err());
        // no owner or operator
        assert!(Configuration::set_marketplace_canister(alice()).is_ok());
        store_mock_contract_with(
            &[1, 2],
            1,
            |contract| {
                contract.seller = alice();
            },
            |_| {},
        );

        // with operator
        assert!(Configuration::set_marketplace_canister(Principal::management_canister()).is_ok());
        store_mock_contract_with(
            &[3, 4],
            2,
            |contract| {
                contract.seller = alice();
            },
            |_| {},
        );
        let metadata =
            Dilazionato::operator_token_metadata(Principal::management_canister()).unwrap();
        assert_eq!(metadata.len(), 2);
        assert_eq!(metadata[0].owner, Some(alice()));
        assert_eq!(metadata[0].token_identifier, Nat::from(3));
        assert_eq!(metadata[1].owner, Some(alice()));
        assert_eq!(metadata[1].token_identifier, Nat::from(4));

        assert!(Dilazionato::operator_of(5.into()).is_err());
    }

    #[test]
    fn test_should_get_supported_interfaces() {
        init_canister();
        assert_eq!(
            Dilazionato::supported_interfaces(),
            vec![
                SupportedInterface::Burn,
                SupportedInterface::TransactionHistory
            ]
        );
    }

    #[test]
    fn test_should_get_total_supply() {
        init_canister();
        store_mock_contract(&[1, 2], 1);
        store_mock_contract(&[3, 4], 2);
        assert_eq!(Dilazionato::total_supply(), Nat::from(4));
    }

    #[tokio::test]
    async fn test_should_transfer() {
        init_canister();
        store_mock_contract(&[1, 2], 1);
        // self transfer
        assert!(Dilazionato::transfer(caller(), 1.into()).await.is_err());

        // transfer
        assert!(
            Dilazionato::transfer(Principal::management_canister(), 1.into())
                .await
                .is_ok()
        );
        assert_eq!(Dilazionato::balance_of(caller()).unwrap(), Nat::from(1));
        assert_eq!(
            Dilazionato::balance_of(Principal::management_canister()).unwrap(),
            Nat::from(1)
        );
        // transfer unexisting
        assert!(
            Dilazionato::transfer(Principal::management_canister(), 5.into())
                .await
                .is_err()
        );
    }

    #[test]
    fn test_should_burn() {
        init_canister();
        store_mock_contract(&[1, 2], 1);
        assert!(Dilazionato::burn(1.into()).is_ok());
        assert_eq!(Dilazionato::balance_of(caller()).unwrap(), Nat::from(1));

        assert!(Dilazionato::burn(5.into()).is_err());
    }

    #[test]
    fn test_should_get_tx() {
        assert!(Dilazionato::transaction(Nat::from(1)).is_err());
        let id = TxHistory::register_token_mint(&mock_token(1, 1));
        assert!(Dilazionato::transaction(id).is_ok());
    }

    #[test]
    fn test_should_get_total_transactions() {
        assert_eq!(Dilazionato::total_transactions(), Nat::from(0));
        let _ = TxHistory::register_token_mint(&mock_token(1, 1));
        assert_eq!(Dilazionato::total_transactions(), Nat::from(1));
    }

    fn init_canister() {
        Dilazionato::init(DilazionatoInitData {
            custodians: vec![caller()],
            fly_canister: alice(),
            marketplace_canister: alice(),
        });
    }
}
