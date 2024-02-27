use ic_stable_structures::memory_manager::{MemoryId, MemoryManager as IcMemoryManager};
use ic_stable_structures::DefaultMemoryImpl;

// Configuration
pub const LEDGER_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(10);
pub const CKETH_LEDGER_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(11);
pub const CKETH_MINTER_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(12);
pub const ERC20_BRIDGE_ADDRESS_MEMORY_ID: MemoryId = MemoryId::new(13);
pub const ADMINS_MEMORY_ID: MemoryId = MemoryId::new(14);

// ERC20 bridge
pub const EKOKE_CANISTER_ETH_ADDRESS_MEMORY_ID: MemoryId = MemoryId::new(20);
pub const ETH_NETWORK_MEMORY_ID: MemoryId = MemoryId::new(21);
pub const ERC20_GAS_PRICE_MEMORY_ID: MemoryId = MemoryId::new(22);
pub const ERC20_LAST_GAS_PRICE_UPDATE_MEMORY_ID: MemoryId = MemoryId::new(23);
pub const ERC20_LOGS_START_BLOCK_MEMORY_ID: MemoryId = MemoryId::new(24);
pub const ETH_PUBKEY_MEMORY_ID: MemoryId = MemoryId::new(25);

thread_local! {
    /// Memory manager
    pub static MEMORY_MANAGER: IcMemoryManager<DefaultMemoryImpl> = IcMemoryManager::init(DefaultMemoryImpl::default());
}
