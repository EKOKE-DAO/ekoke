//! Types associated to the "Deferred" canister

mod agency;
mod contract;
mod data;
mod minter;
mod real_estate;

pub type DeferredMinterResult<T> = Result<T, DeferredMinterError>;
pub type DeferredDataResult<T> = Result<T, DeferredDataError>;

pub use self::agency::{Agency, AgencyId, Continent};
pub use self::contract::{
    Contract, ContractDocument, ContractDocumentData, ContractDocuments, ContractProperties,
    ContractRegistration, ContractType, GenericValue, RestrictedContractProperties,
    RestrictedProperty, RestrictionLevel, Seller, ID,
};
pub use self::data::{
    ConfigurationError as DataConfigurationError, ContractError as DataContractError,
    DeferredDataError, DeferredDataInitData,
};
pub use self::minter::{
    CloseContractError, ConfigurationError, ContractError, DeferredMinterError,
    DeferredMinterInitData, EcdsaError, EcdsaKey, Role, Roles,
};
