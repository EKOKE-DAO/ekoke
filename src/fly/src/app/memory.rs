use ic_stable_structures::memory_manager::{MemoryId, MemoryManager as IcMemoryManager};
use ic_stable_structures::DefaultMemoryImpl;

pub const BALANCES_MEMORY_ID: MemoryId = MemoryId::new(10);
pub const POOL_MEMORY_ID: MemoryId = MemoryId::new(11);

thread_local! {

    /// Memory manager
    pub static MEMORY_MANAGER: IcMemoryManager<DefaultMemoryImpl> = IcMemoryManager::init(DefaultMemoryImpl::default());


}
