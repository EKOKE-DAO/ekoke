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
pub const SWAP_ACCOUNT_MEMORY_ID: MemoryId = MemoryId::new(22);
pub const XRC_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(23);
pub const CKBTC_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(24);
pub const ICP_LEDGER_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(25);
pub const CKETH_LEDGER_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(26);
pub const CKETH_MINTER_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(27);
pub const ERC20_BRIDGE_ADDRESS_MEMORY_ID: MemoryId = MemoryId::new(28);

// Rewards
pub const RMC_MEMORY_ID: MemoryId = MemoryId::new(30);
pub const NEXT_HALVING_MEMORY_ID: MemoryId = MemoryId::new(31);
pub const AVIDITY_MEMORY_ID: MemoryId = MemoryId::new(32);
pub const CPM_MEMORY_ID: MemoryId = MemoryId::new(33);
pub const LAST_CPM_MEMORY_ID: MemoryId = MemoryId::new(34);
pub const LAST_MONTH_MEMORY_ID: MemoryId = MemoryId::new(35);

// Liquidity pool
pub const LIQUIDITY_POOL_ACCOUNT_MEMORY_ID: MemoryId = MemoryId::new(40);
pub const LIQUIDITY_POOL_CKBTC_ACCOUNT_MEMORY_ID: MemoryId = MemoryId::new(41);

// ERC20 bridge
pub const FLY_CANISTER_ETH_ADDRESS_MEMORY_ID: MemoryId = MemoryId::new(50);
pub const ERC20_SWAP_POOL_ACCOUNT_MEMORY_ID: MemoryId = MemoryId::new(51);

thread_local! {
    /// Memory manager
    pub static MEMORY_MANAGER: IcMemoryManager<DefaultMemoryImpl> = IcMemoryManager::init(DefaultMemoryImpl::default());
}
