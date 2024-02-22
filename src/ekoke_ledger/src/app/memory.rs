use ic_stable_structures::memory_manager::{MemoryId, MemoryManager as IcMemoryManager};
use ic_stable_structures::DefaultMemoryImpl;

pub const BALANCES_MEMORY_ID: MemoryId = MemoryId::new(10);
pub const POOL_MEMORY_ID: MemoryId = MemoryId::new(11);
pub const CANISTER_WALLET_ACCOUNT_MEMORY_ID: MemoryId = MemoryId::new(12);
pub const SPEND_ALLOWANCE_MEMORY_ID: MemoryId = MemoryId::new(13);

// Configuration
pub const MINTING_ACCOUNT_MEMORY_ID: MemoryId = MemoryId::new(20);
pub const ROLES_MEMORY_ID: MemoryId = MemoryId::new(21);
pub const SWAP_ACCOUNT_MEMORY_ID: MemoryId = MemoryId::new(22);
pub const XRC_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(23);
pub const CKBTC_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(24);
pub const ICP_LEDGER_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(25);
pub const ARCHIVE_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(26);

// Liquidity pool
pub const LIQUIDITY_POOL_ACCOUNT_MEMORY_ID: MemoryId = MemoryId::new(40);
pub const LIQUIDITY_POOL_CKBTC_ACCOUNT_MEMORY_ID: MemoryId = MemoryId::new(41);

// Rewards
pub const RMC_MEMORY_ID: MemoryId = MemoryId::new(60);
pub const NEXT_HALVING_MEMORY_ID: MemoryId = MemoryId::new(61);
pub const AVIDITY_MEMORY_ID: MemoryId = MemoryId::new(62);
pub const CPM_MEMORY_ID: MemoryId = MemoryId::new(63);
pub const LAST_CPM_MEMORY_ID: MemoryId = MemoryId::new(64);
pub const LAST_MONTH_MEMORY_ID: MemoryId = MemoryId::new(65);

thread_local! {
    /// Memory manager
    pub static MEMORY_MANAGER: IcMemoryManager<DefaultMemoryImpl> = IcMemoryManager::init(DefaultMemoryImpl::default());
}
