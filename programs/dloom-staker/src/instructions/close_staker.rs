// FILE: programs/dloom_stake/src/instructions/close_staker.rs

use anchor_lang::prelude::*;
use crate::{
    errors::StakingError,
    state::{Farm, Staker},
    instructions::reward_math::{update_reward_accumulator, sync_staker_rewards},
};

pub fn handle_close_staker(ctx: Context<CloseStaker>) -> Result<()> {
    let farm = &mut ctx.accounts.farm;
    let staker = &mut ctx.accounts.staker;

    update_reward_accumulator(farm)?;
    sync_staker_rewards(farm, staker)?;

    require!(staker.total_active_weight == 0, StakingError::InsufficientBalance);
    require!(staker.flexible_balance == 0, StakingError::InsufficientBalance);
    require!(staker.positions.is_empty(), StakingError::InsufficientBalance);

    require!(staker.earned_rewards == 0, StakingError::NoRewardsToClaim);

    
    msg!("Staker Account Closed. Rent returned to owner.");

    Ok(())
}

#[derive(Accounts)]
pub struct CloseStaker<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    pub farm: Account<'info, Farm>,

    #[account(
        mut,
        close = owner, 
        seeds = [b"staker", owner.key().as_ref(), farm.key().as_ref()],
        bump = staker.bump,
        has_one = owner,
        has_one = farm
    )]
    pub staker: Account<'info, Staker>,
}