// FILE: programs/dloom_stake/src/instructions/position/stake.rs

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::Staker; 
use crate::{
    errors::StakingError,
    state::{Farm, LockedPosition},
    instructions::reward_math::{update_reward_accumulator, sync_staker_rewards, PRECISION},
};

pub fn handle_stake_locked(ctx: Context<StakeLocked>, amount: u64, duration: i64) -> Result<()> {
    // --- FIX: Define variables at the very top ---
    let farm = &mut ctx.accounts.farm;
    let staker = &mut ctx.accounts.staker;
    let now = Clock::get()?.unix_timestamp;
    // ---------------------------------------------

    require!(!farm.is_paused, StakingError::FarmPaused);
    require!(amount > 0, StakingError::ZeroAmount);
    require!(duration > 0, StakingError::TierNotFound); 
    
    // --- FIX: Max Positions Check ---
    require!(
        staker.positions.len() < Staker::MAX_POSITIONS, 
        StakingError::TooManyPositions 
    );

    let tier = farm.lockup_tiers.iter()
        .find(|t| t.duration == duration)
        .ok_or(StakingError::TierNotFound)?;
    let multiplier = tier.multiplier;

    update_reward_accumulator(farm)?;
    sync_staker_rewards(farm, staker)?;

    let weight = (amount as u128)
        .checked_mul(multiplier as u128)
        .ok_or(StakingError::MathOverflow)?
        .checked_div(10000)
        .ok_or(StakingError::MathOverflow)?;

    // 4. Create Position
    let position_id = staker.next_position_id;
    staker.next_position_id += 1;

    let new_position = LockedPosition {
        id: position_id,
        amount,
        lock_start_timestamp: now,
        lock_end_timestamp: now.checked_add(duration).ok_or(StakingError::MathOverflow)?,
        multiplier,
        weight,
    };

    staker.positions.push(new_position);

    // 5. Update Aggregators
    staker.total_active_weight = staker.total_active_weight
        .checked_add(weight)
        .ok_or(StakingError::MathOverflow)?;

    farm.total_weighted_stake = farm.total_weighted_stake
        .checked_add(weight)
        .ok_or(StakingError::MathOverflow)?;

    // 6. Fix Debt
    staker.reward_debt = staker.total_active_weight
        .checked_mul(farm.reward_per_token_stored)
        .ok_or(StakingError::MathOverflow)?
        .checked_div(PRECISION)
        .ok_or(StakingError::MathOverflow)?;

    // 7. Transfer Tokens
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_lp_token_account.to_account_info(),
        to: ctx.accounts.lp_vault.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    msg!("Locked Stake Created: ID {} Amount {} Weight {}", position_id, amount, weight);

    Ok(())
}

#[derive(Accounts)]
pub struct StakeLocked<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub farm: Account<'info, Farm>,

    #[account(
        mut,
        seeds = [b"staker", owner.key().as_ref(), farm.key().as_ref()],
        bump = staker.bump,
        has_one = owner,
        has_one = farm,
        realloc = Staker::BASE_SIZE + ((staker.positions.len() + 1) * Staker::POSITION_SIZE),
        realloc::payer = owner,
        realloc::zero = false
    )]
    pub staker: Account<'info, Staker>,

    #[account(mut, address = farm.lp_vault)]
    pub lp_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_lp_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>, 
}