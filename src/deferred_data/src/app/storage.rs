use std::cell::RefCell;

use candid::Nat;
use did::deferred::{Contract, DataContractError, DeferredDataError, DeferredDataResult};
use did::{StorableNat, ID};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{BTreeMap, DefaultMemoryImpl, StableCell};

use crate::app::memory::{
    CONTRACTS_MEMORY_ID, DOCUMENTS_MEMORY_ID, MEMORY_MANAGER, NEXT_DOCUMENT_ID_MEMORY_ID,
};

mod contracts;
mod documents;

pub use contracts::ContractStorage;
use documents::DocumentStorage;

thread_local! {

    /// ContractStorage storage (1 contract has many tokens)
    static CONTRACTS: RefCell<BTreeMap<StorableNat, Contract, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(BTreeMap::new(MEMORY_MANAGER.with(|mm| mm.get(CONTRACTS_MEMORY_ID))));

    /// Documents storage storage (assoc between ID and document data)
    static DOCUMENTS: RefCell<BTreeMap<StorableNat, Vec<u8>, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(BTreeMap::new(MEMORY_MANAGER.with(|mm| mm.get(DOCUMENTS_MEMORY_ID))));

    /// Next document ID
    static NEXT_DOCUMENT_ID: RefCell<StableCell<StorableNat, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(NEXT_DOCUMENT_ID_MEMORY_ID)), Nat::from(0u64).into()).unwrap()
    );

}

fn with_contract<T, F>(id: &ID, f: F) -> DeferredDataResult<T>
where
    F: FnOnce(&Contract) -> DeferredDataResult<T>,
{
    CONTRACTS.with_borrow(|contracts| {
        if let Some(contract) = contracts.get(&StorableNat::from(id.clone())) {
            f(&contract)
        } else {
            Err(DeferredDataError::Contract(
                DataContractError::ContractNotFound(id.clone()),
            ))
        }
    })
}

fn with_contract_mut<T, F>(id: &ID, f: F) -> DeferredDataResult<T>
where
    F: FnOnce(&mut Contract) -> DeferredDataResult<T>,
{
    CONTRACTS.with_borrow_mut(|contracts| {
        if let Some(mut contract) = contracts.get(&StorableNat::from(id.clone())) {
            let res = f(&mut contract)?;
            // update contract
            contracts.insert(StorableNat::from(id.clone()), contract.clone());

            Ok(res)
        } else {
            Err(DeferredDataError::Contract(
                DataContractError::ContractNotFound(id.clone()),
            ))
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
