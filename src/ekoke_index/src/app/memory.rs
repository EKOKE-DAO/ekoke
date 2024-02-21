use ic_stable_structures::memory_manager::{MemoryId, MemoryManager as IcMemoryManager};
use ic_stable_structures::DefaultMemoryImpl;

pub const ACCOUNTS_FROM_MEMORY_ID: MemoryId = MemoryId::new(10);
pub const ACCOUNTS_TO_MEMORY_ID: MemoryId = MemoryId::new(11);
pub const ACCOUNTS_SPENDER_MEMORY_ID: MemoryId = MemoryId::new(12);

// Configuration
pub const LEDGER_CANISTER_ID_MEMORY_ID: MemoryId = MemoryId::new(20);
pub const ARCHIVE_CANISTER_ID_MEMORY_ID: MemoryId = MemoryId::new(21);

thread_local! {
    /// Memory manager
    pub static MEMORY_MANAGER: IcMemoryManager<DefaultMemoryImpl> = IcMemoryManager::init(DefaultMemoryImpl::default());
}
