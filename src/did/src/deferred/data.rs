mod error;

use candid::{CandidType, Deserialize, Principal};
use ic_log::LogSettingsV2;

pub use self::error::{ConfigurationError, ContractError, DeferredDataError};

/// These are the arguments which are taken by the deferred data canister at creation
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct DeferredDataInitData {
    /// Log settings
    pub log_settings: LogSettingsV2,
    /// minter canister
    pub minter: Principal,
}
