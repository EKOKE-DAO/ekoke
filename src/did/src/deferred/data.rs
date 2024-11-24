mod error;

use candid::{CandidType, Deserialize, Principal};

pub use self::error::{ConfigurationError, ContractError, DeferredDataError};

/// These are the arguments which are taken by the deferred data canister at creation
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct DeferredDataInitData {
    /// minter canister
    pub minter: Principal,
}
