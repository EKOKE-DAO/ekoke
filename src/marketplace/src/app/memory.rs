use ic_stable_structures::memory_manager::{MemoryId, MemoryManager as IcMemoryManager};
use ic_stable_structures::DefaultMemoryImpl;

// Configuration
pub const ROLES_MEMORY_ID: MemoryId = MemoryId::new(20);
pub const DEFERRED_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(21);
pub const FLY_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(22);
pub const INTEREST_FOR_BUYER_MEMORY_ID: MemoryId = MemoryId::new(23);
pub const FLY_LIQUIDITY_POOL_ACCOUNT_MEMORY_ID: MemoryId = MemoryId::new(24);

thread_local! {
    /// Memory manager
    pub static MEMORY_MANAGER: IcMemoryManager<DefaultMemoryImpl> = IcMemoryManager::init(DefaultMemoryImpl::default());
}
