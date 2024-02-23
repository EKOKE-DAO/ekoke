//! Types associated to the "Ekoke" canister

mod role;

use candid::{CandidType, Deserialize, Principal};

pub use self::role::{Role, Roles};

/// These are the arguments which are taken by the ekoke canister on init
#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct EkokeRewardPoolInitData {
    pub admins: Vec<Principal>,
    /// Id of the ekoke ledger canister
    pub ledger_canister: Principal,
    /// Deferred canister
    pub deferred_canister: Principal,
    /// Marketplace canister
    pub marketplace_canister: Principal,
}
