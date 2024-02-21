use std::cell::RefCell;

use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{DefaultMemoryImpl, StableLog};
use serde_bytes::ByteBuf;

use crate::app::memory::{BLOCKS_DATA_MEMORY_ID, BLOCKS_INDEX_MEMORY_ID, MEMORY_MANAGER};

const MAX_CAPACITY: u64 = 1024 * 1024 * 1024 * 10; // 10GB

thread_local! {

    static BLOCKS: RefCell<StableLog<Vec<u8>, VirtualMemory<DefaultMemoryImpl>, VirtualMemory<DefaultMemoryImpl>>> =
        RefCell::new(StableLog::init(
            MEMORY_MANAGER.with(|mm| mm.get(BLOCKS_INDEX_MEMORY_ID)),
            MEMORY_MANAGER.with(|mm| mm.get(BLOCKS_DATA_MEMORY_ID))
        )
        .unwrap());

}

pub struct Blocks;

impl Blocks {
    /// Append blocks
    pub fn append_blocks(blocks: Vec<ByteBuf>) -> Result<(), String> {
        // get blocks size
        let blocks_size: u64 = blocks.iter().map(|block| block.len() as u64).sum();
        if blocks_size > Self::remaining_capacity() {
            return Err("Not enough space".to_string());
        }
        BLOCKS.with_borrow_mut(|log| {
            for block in blocks {
                log.append(&block.to_vec()).unwrap();
            }
        });

        Ok(())
    }

    /// Get total size of blocks
    pub fn total_size() -> u64 {
        BLOCKS.with(|blocks| blocks.borrow().log_size_bytes())
    }

    /// Returns the remaining capacity in bytes
    pub fn remaining_capacity() -> u64 {
        MAX_CAPACITY - Self::total_size()
    }
}
