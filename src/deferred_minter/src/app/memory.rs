use ic_stable_structures::memory_manager::{MemoryId, MemoryManager as IcMemoryManager};
use ic_stable_structures::DefaultMemoryImpl;

pub const AGENCIES_MEMORY_ID: MemoryId = MemoryId::new(10);

pub const ALLOWED_CURRENCIES_MEMORY_ID: MemoryId = MemoryId::new(20);
pub const DEFERRED_DATA_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(21);
pub const DEFERRED_ERC721_CONTRACT_MEMORY_ID: MemoryId = MemoryId::new(22);
pub const REWARD_POOL_CONTRACT_MEMORY_ID: MemoryId = MemoryId::new(23);
pub const ECDSA_KEY_MEMORY_ID: MemoryId = MemoryId::new(24);
pub const CHAIN_ID_MEMORY_ID: MemoryId = MemoryId::new(25);
pub const EVM_RPC_MEMORY_ID: MemoryId = MemoryId::new(26);
pub const EVM_CUSTOM_RPC_API_MEMORY_ID: MemoryId = MemoryId::new(27);
pub const EVM_GAS_PRICE_MEMORY_ID: MemoryId = MemoryId::new(28);
pub const LOG_SETTINGS_MEMORY_ID: MemoryId = MemoryId::new(29);

pub const ROLES_MEMORY_ID: MemoryId = MemoryId::new(30);

pub const ETH_WALLET_ADDRESS_MEMORY_ID: MemoryId = MemoryId::new(40);
pub const ETH_WALLET_PUBKEY_MEMORY_ID: MemoryId = MemoryId::new(41);

pub const NEXT_CONTRACT_ID_MEMORY_ID: MemoryId = MemoryId::new(50);

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
