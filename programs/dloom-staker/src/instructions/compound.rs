// FILE: programs/dloom_stake/src/instructions/compound.rs

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::{
    errors::StakingError,
    events::Compounded,
    state::{Farm, Staker},
    // FIX: Use the new sync function and PRECISION
    instructions::reward_math::{update_reward_accumulator, sync_staker_rewards, PRECISION},
};

pub fn handle_compound(ctx: Context<Compound>) -> Result<()> {
     require!(!ctx.accounts.farm.is_paused, StakingError::FarmPaused);
     
    let farm = &mut ctx.accounts.farm;
    let staker = &mut ctx.accounts.staker;

    // 1. Validation
    require!(
        farm.reward_mint == farm.lp_mint,
        StakingError::CompoundingNotSupported
    );
    
    // 2. Math (Sync Rewards)
    update_reward_accumulator(farm)?;
    sync_staker_rewards(farm, staker)?;

    let pending_rewards = staker.earned_rewards;
    require!(pending_rewards > 0, StakingError::NoRewardsToClaim);

    // Reset earned cache
    staker.earned_rewards = 0;

    // 3. Transfer from Reward Vault -> LP Vault (Staking the rewards)
    let seeds = &[
        b"farm".as_ref(),
        farm.lp_mint.as_ref(),
        farm.reward_mint.as_ref(),
        &[farm.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.reward_vault.to_account_info(),
        to: ctx.accounts.lp_vault.to_account_info(),
        authority: farm.to_account_info(),
    };
    
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

    token::transfer(cpi_ctx, pending_rewards)?;

    // 4. Stake the Rewards (Into Flexible Bucket)
    // We treat compounded rewards as "Flexible" stake (1x multiplier)
    
    staker.flexible_balance = staker.flexible_balance
        .checked_add(pending_rewards)
        .ok_or(StakingError::MathOverflow)?;
    
    let weight_increase = pending_rewards as u128; // 1x Multiplier

    staker.total_active_weight = staker.total_active_weight
        .checked_add(weight_increase)
        .ok_or(StakingError::MathOverflow)?;

    farm.total_weighted_stake = farm.total_weighted_stake
        .checked_add(weight_increase)
        .ok_or(StakingError::MathOverflow)?;

    // 5. Fix Debt
    // We added weight, so we must update debt to prevent double claiming
    staker.reward_debt = staker.total_active_weight
        .checked_mul(farm.reward_per_token_stored)
        .ok_or(StakingError::MathOverflow)?
        .checked_div(PRECISION)
        .ok_or(StakingError::MathOverflow)?;

    emit!(Compounded {
        owner: staker.owner,
        farm_address: farm.key(),
        reward_amount_compounded: pending_rewards,
        new_total_staked_balance: staker.flexible_balance, // Just showing flex balance here
    });

    Ok(())
}

#[derive(Accounts)]
pub struct Compound<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"farm", farm.lp_mint.as_ref(), farm.reward_mint.as_ref()],
        bump = farm.bump
    )]
    pub farm: Account<'info, Farm>,
    
    #[account(
        mut,
        seeds = [b"staker", owner.key().as_ref(), farm.key().as_ref()],
        bump = staker.bump,
        has_one = owner,
        has_one = farm
    )]
    pub staker: Account<'info, Staker>,

    // Source of tokens (rewards)
    #[account(mut, address = farm.reward_vault)]
    pub reward_vault: Account<'info, TokenAccount>,

    // Destination of tokens (staked back into LP vault)
    #[account(mut, address = farm.lp_vault)]
    pub lp_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}