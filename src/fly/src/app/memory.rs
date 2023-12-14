use ic_stable_structures::memory_manager::{MemoryId, MemoryManager as IcMemoryManager};
use ic_stable_structures::DefaultMemoryImpl;

pub const BALANCES_MEMORY_ID: MemoryId = MemoryId::new(10);
pub const POOL_MEMORY_ID: MemoryId = MemoryId::new(11);
pub const CANISTER_WALLET_ACCOUNT_MEMORY_ID: MemoryId = MemoryId::new(12);
pub const REGISTER_MEMORY_ID: MemoryId = MemoryId::new(13);
pub const SPEND_ALLOWANCE_MEMORY_ID: MemoryId = MemoryId::new(14);

// Configuration
pub const MINTING_ACCOUNT_MEMORY_ID: MemoryId = MemoryId::new(20);
pub const ROLES_MEMORY_ID: MemoryId = MemoryId::new(21);

// Rewards
pub const RMC_MEMORY_ID: MemoryId = MemoryId::new(30);
pub const NEXT_HALVING_MEMORY_ID: MemoryId = MemoryId::new(31);
pub const AVIDITY_MEMORY_ID: MemoryId = MemoryId::new(32);
pub const CPM_MEMORY_ID: MemoryId = MemoryId::new(33);
pub const LAST_CPM_MEMORY_ID: MemoryId = MemoryId::new(34);
pub const LAST_MONTH_MEMORY_ID: MemoryId = MemoryId::new(35);

thread_local! {
    /// Memory manager
    pub static MEMORY_MANAGER: IcMemoryManager<DefaultMemoryImpl> = IcMemoryManager::init(DefaultMemoryImpl::default());
}
