use ic_stable_structures::memory_manager::{MemoryId, MemoryManager as IcMemoryManager};
use ic_stable_structures::DefaultMemoryImpl;

// Liquidity pool
pub const LIQUIDITY_POOL_ACCOUNT_MEMORY_ID: MemoryId = MemoryId::new(10);
pub const LIQUIDITY_POOL_CKBTC_ACCOUNT_MEMORY_ID: MemoryId = MemoryId::new(11);

// Configuration
pub const ADMINS_MEMORY_ID: MemoryId = MemoryId::new(20);
pub const CKBTC_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(21);
pub const ICP_LEDGER_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(22);
pub const SWAP_ACCOUNT_MEMORY_ID: MemoryId = MemoryId::new(23);
pub const XRC_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(24);

thread_local! {
    /// Memory manager
    pub static MEMORY_MANAGER: IcMemoryManager<DefaultMemoryImpl> = IcMemoryManager::init(DefaultMemoryImpl::default());
}
