use std::cell::RefCell;

use did::deferred::{
    Agency, Contract, DeferredError, DeferredResult, StorableTxEvent, Token, TokenError,
};
use did::{StorableNat, StorablePrincipal, ID};
use dip721_rs::TokenIdentifier;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{BTreeMap, DefaultMemoryImpl};

use crate::app::memory::{
    AGENCIES_MEMORY_ID, CONTRACTS_MEMORY_ID, MEMORY_MANAGER, TOKENS_MEMORY_ID,
    TRANSACTIONS_MEMORY_ID,
};

mod agents;
mod contracts;
mod tx_history;

pub use agents::Agents;
pub use contracts::ContractStorage;
pub use tx_history::TxHistory;

thread_local! {
    /// Agencies storage (1 wallet has 1 agency)
    static AGENCIES: RefCell<BTreeMap<StorablePrincipal, Agency, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(BTreeMap::new(MEMORY_MANAGER.with(|mm| mm.get(AGENCIES_MEMORY_ID))));

    /// ContractStorage storage (1 contract has many tokens)
    static CONTRACTS: RefCell<BTreeMap<StorableNat, Contract, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(BTreeMap::new(MEMORY_MANAGER.with(|mm| mm.get(CONTRACTS_MEMORY_ID))));

    /// Tokens storage (NFTs)
    static TOKENS: RefCell<BTreeMap<StorableNat, Token, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(BTreeMap::new(MEMORY_MANAGER.with(|mm| mm.get(TOKENS_MEMORY_ID))));

    /// Transactions history
    static TX_HISTORY: RefCell<BTreeMap<StorableNat, StorableTxEvent, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(BTreeMap::new(MEMORY_MANAGER.with(|mm| mm.get(TRANSACTIONS_MEMORY_ID))));
}

fn with_contract<T, F>(id: &ID, f: F) -> DeferredResult<T>
where
    F: FnOnce(&Contract) -> DeferredResult<T>,
{
    CONTRACTS.with_borrow(|contracts| {
        if let Some(contract) = contracts.get(&StorableNat::from(id.clone())) {
            f(&contract)
        } else {
            Err(DeferredError::Token(TokenError::ContractNotFound(
                id.clone(),
            )))
        }
    })
}

fn with_contract_mut<T, F>(id: &ID, f: F) -> DeferredResult<T>
where
    F: FnOnce(&mut Contract) -> DeferredResult<T>,
{
    CONTRACTS.with_borrow_mut(|contracts| {
        if let Some(mut contract) = contracts.get(&StorableNat::from(id.clone())) {
            let res = f(&mut contract)?;
            // update contract
            contracts.insert(StorableNat::from(id.clone()), contract.clone());

            Ok(res)
        } else {
            Err(DeferredError::Token(TokenError::ContractNotFound(
                id.clone(),
            )))
        }
    })
}

fn with_contracts<T, F>(f: F) -> T
where
    F: FnOnce(&BTreeMap<StorableNat, Contract, VirtualMemory<DefaultMemoryImpl>>) -> T,
{
    CONTRACTS.with_borrow(|contracts| f(contracts))
}

fn with_contracts_mut<T, F>(f: F) -> T
where
    F: FnOnce(&mut BTreeMap<StorableNat, Contract, VirtualMemory<DefaultMemoryImpl>>) -> T,
{
    CONTRACTS.with_borrow_mut(|contracts| f(contracts))
}

fn with_tokens<T, F>(f: F) -> T
where
    F: FnOnce(&BTreeMap<StorableNat, Token, VirtualMemory<DefaultMemoryImpl>>) -> T,
{
    TOKENS.with_borrow(|tokens| f(tokens))
}

fn with_token<T, F>(id: &TokenIdentifier, f: F) -> DeferredResult<T>
where
    F: FnOnce(&Token) -> DeferredResult<T>,
{
    TOKENS.with_borrow(|tokens| {
        if let Some(token) = tokens.get(&StorableNat::from(id.clone())) {
            f(&token)
        } else {
            Err(DeferredError::Token(TokenError::ContractNotFound(
                id.clone(),
            )))
        }
    })
}

fn with_token_mut<T, F>(id: &TokenIdentifier, f: F) -> DeferredResult<T>
where
    F: FnOnce(&mut Token) -> DeferredResult<T>,
{
    TOKENS.with_borrow_mut(|tokens| {
        if let Some(mut token) = tokens.get(&StorableNat::from(id.clone())) {
            let res = f(&mut token)?;
            // update token
            tokens.insert(StorableNat::from(id.clone()), token.clone());

            Ok(res)
        } else {
            Err(DeferredError::Token(TokenError::ContractNotFound(
                id.clone(),
            )))
        }
    })
}

fn with_tokens_mut<T, F>(f: F) -> T
where
    F: FnOnce(&mut BTreeMap<StorableNat, Token, VirtualMemory<DefaultMemoryImpl>>) -> T,
{
    TOKENS.with_borrow_mut(|tokens| f(tokens))
}

fn with_tx_history<T, F>(f: F) -> T
where
    F: FnOnce(&BTreeMap<StorableNat, StorableTxEvent, VirtualMemory<DefaultMemoryImpl>>) -> T,
{
    TX_HISTORY.with_borrow(|tx_history| f(tx_history))
}

fn with_tx_history_mut<T, F>(f: F) -> T
where
    F: FnOnce(&mut BTreeMap<StorableNat, StorableTxEvent, VirtualMemory<DefaultMemoryImpl>>) -> T,
{
    TX_HISTORY.with_borrow_mut(|tx_history| f(tx_history))
}
