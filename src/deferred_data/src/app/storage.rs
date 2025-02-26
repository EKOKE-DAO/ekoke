use std::cell::RefCell;

use did::deferred::{
    Contract, DataContractError, DeferredDataError, DeferredDataResult, RealEstate, RealEstateError,
};
use did::{StorableNat, ID};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{BTreeMap, DefaultMemoryImpl, StableCell};

use crate::app::memory::{
    CONTRACTS_MEMORY_ID, DOCUMENTS_MEMORY_ID, MEMORY_MANAGER, NEXT_DOCUMENT_ID_MEMORY_ID,
    REAL_ESTATE_MEMORY_ID,
};

mod contracts;
mod documents;
mod real_estate;

use documents::DocumentStorage;

pub use self::contracts::ContractStorage;
pub use self::real_estate::RealEstateStorage;

thread_local! {

    /// ContractStorage storage (1 contract has many tokens)
    static CONTRACTS: RefCell<BTreeMap<StorableNat, Contract, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(BTreeMap::new(MEMORY_MANAGER.with(|mm| mm.get(CONTRACTS_MEMORY_ID))));

    /// Documents storage storage (assoc between ID and document data)
    static DOCUMENTS: RefCell<BTreeMap<u64, Vec<u8>, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(BTreeMap::new(MEMORY_MANAGER.with(|mm| mm.get(DOCUMENTS_MEMORY_ID))));

    /// Next document ID
    static NEXT_DOCUMENT_ID: RefCell<StableCell<u64, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableCell::new(MEMORY_MANAGER.with(|mm| mm.get(NEXT_DOCUMENT_ID_MEMORY_ID)), 0u64).unwrap()
    );

    /// Real estate storage
    static REAL_ESTATES: RefCell<BTreeMap<StorableNat, RealEstate, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(BTreeMap::new(MEMORY_MANAGER.with(|mm| mm.get(REAL_ESTATE_MEMORY_ID))));

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

fn with_real_estate<T, F>(id: &ID, f: F) -> DeferredDataResult<T>
where
    F: FnOnce(&RealEstate) -> DeferredDataResult<T>,
{
    REAL_ESTATES.with_borrow(|storage| {
        if let Some(real_estate) = storage.get(&StorableNat::from(id.clone())) {
            f(&real_estate)
        } else {
            Err(DeferredDataError::RealEstate(RealEstateError::NotFound(
                id.clone(),
            )))
        }
    })
}

fn with_real_estate_mut<T, F>(id: &ID, f: F) -> DeferredDataResult<T>
where
    F: FnOnce(&mut RealEstate) -> DeferredDataResult<T>,
{
    REAL_ESTATES.with_borrow_mut(|storage| {
        if let Some(mut re) = storage.get(&StorableNat::from(id.clone())) {
            let res = f(&mut re)?;
            // update contract
            storage.insert(StorableNat::from(id.clone()), re.clone());

            Ok(res)
        } else {
            Err(DeferredDataError::RealEstate(RealEstateError::NotFound(
                id.clone(),
            )))
        }
    })
}

fn with_real_estates<T, F>(f: F) -> T
where
    F: FnOnce(&BTreeMap<StorableNat, RealEstate, VirtualMemory<DefaultMemoryImpl>>) -> T,
{
    REAL_ESTATES.with_borrow(|contracts| f(contracts))
}

fn with_real_estate_storage_mut<T, F>(f: F) -> T
where
    F: FnOnce(&mut BTreeMap<StorableNat, RealEstate, VirtualMemory<DefaultMemoryImpl>>) -> T,
{
    REAL_ESTATES.with_borrow_mut(|contracts| f(contracts))
}
