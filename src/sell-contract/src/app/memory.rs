use ic_stable_structures::memory_manager::{MemoryId, MemoryManager as IcMemoryManager};
use ic_stable_structures::DefaultMemoryImpl;

pub const TOKENS_MEMORY_ID: MemoryId = MemoryId::new(10);
pub const CONTRACTS_MEMORY_ID: MemoryId = MemoryId::new(11);
pub const FLY_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(12);

#[derive(Default)]
pub struct Storage;

thread_local! {

    /// Memory manager
    pub static MEMORY_MANAGER: IcMemoryManager<DefaultMemoryImpl> = IcMemoryManager::init(DefaultMemoryImpl::default());
}
