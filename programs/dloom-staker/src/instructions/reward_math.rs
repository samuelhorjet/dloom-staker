// FILE: programs/dloom_stake/src/instructions/reward_math.rs

use anchor_lang::prelude::*;
use crate::{
    state::{Farm, Staker},
    errors::StakingError,
};

// FIX: Added 'pub' so it can be imported in stake/unstake files
pub const PRECISION: u128 = 1_000_000_000_000;

pub fn update_reward_accumulator(farm: &mut Account<Farm>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let time_since_last_update = now.checked_sub(farm.last_update_timestamp).unwrap_or(0);

    if time_since_last_update > 0 && farm.total_weighted_stake > 0 {
        let reward = (time_since_last_update as u128)
            .checked_mul(farm.reward_rate as u128)
            .ok_or(StakingError::MathOverflow)?;

        let reward_per_token = reward
            .checked_mul(PRECISION)
            .ok_or(StakingError::MathOverflow)?
            .checked_div(farm.total_weighted_stake)
            .ok_or(StakingError::MathOverflow)?;

        farm.reward_per_token_stored = farm.reward_per_token_stored
            .checked_add(reward_per_token)
            .ok_or(StakingError::MathOverflow)?;
    }
    
    farm.last_update_timestamp = now;
    Ok(())
}

pub fn sync_staker_rewards(farm: &Account<Farm>, staker: &mut Account<Staker>) -> Result<()> {
    let acc_reward_per_token = farm.reward_per_token_stored;
    
    let total_accumulated = staker.total_active_weight
        .checked_mul(acc_reward_per_token)
        .ok_or(StakingError::MathOverflow)?
        .checked_div(PRECISION)
        .ok_or(StakingError::MathOverflow)?;
        
    let pending = total_accumulated
        .checked_sub(staker.reward_debt)
        .unwrap_or(0); 

    if pending > 0 {
        staker.earned_rewards = staker.earned_rewards
            .checked_add(pending as u64)
            .ok_or(StakingError::MathOverflow)?;
    }

    staker.reward_debt = total_accumulated;

    Ok(())
}