//! # Reward
//!
//! This module defines functions to calculate Dilazionato contracts rewards.

use did::{fly::FlyResult, ID};

use crate::app::pool::Pool;

pub struct Reward;

impl Reward {
    /// Calculate reward for the provided contract ID and installments.
    ///
    /// Returns None if unable to reserve enough tokens.
    pub fn get_contract_reward(contract_id: ID, installments: u64) -> Option<u64> {
        // If a pool is already reserved, return the pool
        if let Ok(reward) = Pool::balance_of(&contract_id) {
            return Some(reward);
        }

        todo!();
    }
}
