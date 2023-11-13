//! # Reward
//!
//! This module defines functions to calculate Dilazionato contracts rewards.

use did::fly::{FlyResult, PicoFly};
use did::ID;

use crate::app::balance::Balance;
use crate::app::pool::Pool;

pub struct Reward;

impl Reward {
    /// Calculate reward for the provided contract ID and installments.
    ///
    /// Returns None if unable to reserve enough tokens.
    pub fn get_contract_reward(contract_id: ID, installments: u64) -> FlyResult<PicoFly> {
        // If a pool is already reserved, return the pool
        if let Ok(reward) = Pool::balance_of(&contract_id) {
            return Ok(reward);
        }

        let reward = Self::calc_reward(installments);
        let amount_to_reserve = reward * installments;

        // reserve pool
        Balance::move_from_canister_to_reward_pool(amount_to_reserve)?;
        Pool::reserve(&contract_id, amount_to_reserve)?;

        Ok(reward)
    }

    fn calc_reward(installments: u64) -> PicoFly {
        todo!();
    }
}
