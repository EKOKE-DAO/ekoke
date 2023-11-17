use ic_stable_structures::memory_manager::{MemoryId, MemoryManager as IcMemoryManager};
use ic_stable_structures::DefaultMemoryImpl;

pub const BALANCES_MEMORY_ID: MemoryId = MemoryId::new(10);
pub const POOL_MEMORY_ID: MemoryId = MemoryId::new(11);
pub const CANISTER_WALLET_ACCOUNT_MEMORY_ID: MemoryId = MemoryId::new(12);
pub const REGISTER_MEMORY_ID: MemoryId = MemoryId::new(13);

// Configuration
pub const MINTING_ACCOUNT_MEMORY_ID: MemoryId = MemoryId::new(20);
pub const ROLES_MEMORY_ID: MemoryId = MemoryId::new(21);

thread_local! {
    /// Memory manager
    pub static MEMORY_MANAGER: IcMemoryManager<DefaultMemoryImpl> = IcMemoryManager::init(DefaultMemoryImpl::default());
}
