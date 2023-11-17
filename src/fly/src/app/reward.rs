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
    pub fn get_contract_reward(contract_id: ID, installments: PicoFly) -> FlyResult<PicoFly> {
        // If a pool is already reserved, return the pool
        if let Ok(reward) = Pool::balance_of(&contract_id) {
            return Ok(reward);
        }

        let reward = Self::calc_reward(installments.clone());
        let amount_to_reserve = reward.clone() * installments;

        // reserve pool
        Pool::reserve(
            &contract_id,
            Balance::canister_wallet_account(),
            amount_to_reserve,
        )?;

        Ok(reward)
    }

    fn calc_reward(installments: PicoFly) -> PicoFly {
        todo!();
    }
}
