// FILE: programs/dloom_stake/src/instructions/compound.rs

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::{
    errors::StakingError,
    events::Compounded,
    state::{Farm, Staker},
    instructions::reward_math::{update_reward_accumulator, calculate_pending_rewards},
};

pub fn handle_compound(ctx: Context<Compound>) -> Result<()> {
    let farm = &mut ctx.accounts.farm;
    let staker = &mut ctx.accounts.staker;

    // 1. Validation
    require!(
        farm.reward_mint == farm.lp_mint,
        StakingError::CompoundingNotSupported
    );
    
    // 2. Math
    update_reward_accumulator(farm)?;
    let pending_rewards = calculate_pending_rewards(farm, staker)?;
    require!(pending_rewards > 0, StakingError::NoRewardsToClaim);

    staker.rewards_paid = pending_rewards as u128;
    staker.reward_per_token_snapshot = farm.reward_per_token_stored;

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

    // 4. Update Weighted Stake Logic
    // Remove old weight
    let old_weighted_stake = (staker.balance as u128)
        .checked_mul(staker.reward_multiplier as u128)
        .ok_or(StakingError::MathOverflow)?
        .checked_div(10000)
        .ok_or(StakingError::MathOverflow)?;
    farm.total_weighted_stake = farm.total_weighted_stake
        .checked_sub(old_weighted_stake)
        .ok_or(StakingError::MathOverflow)?;
        
    // Update balance
    staker.balance = staker.balance.checked_add(pending_rewards).ok_or(StakingError::MathOverflow)?;
    
    // Add new weight
    let new_weighted_stake = (staker.balance as u128)
        .checked_mul(staker.reward_multiplier as u128)
        .ok_or(StakingError::MathOverflow)?
        .checked_div(10000)
        .ok_or(StakingError::MathOverflow)?;
    farm.total_weighted_stake = farm.total_weighted_stake
        .checked_add(new_weighted_stake)
        .ok_or(StakingError::MathOverflow)?;

    emit!(Compounded {
        owner: staker.owner,
        farm_address: farm.key(),
        reward_amount_compounded: pending_rewards,
        new_total_staked_balance: staker.balance,
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