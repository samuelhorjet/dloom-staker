// FILE: programs/dloom_stake/src/instructions/bucket/unstake.rs

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::{
    errors::StakingError,
    state::{Farm, Staker},
    instructions::reward_math::{update_reward_accumulator, sync_staker_rewards, PRECISION},
};

pub fn handle_unstake_flexible(ctx: Context<UnstakeFlexible>, amount: u64) -> Result<()> {
    // Allow unstake if emergency mode is on, regardless of pause
    if !ctx.accounts.farm.is_emergency_mode {
        require!(!ctx.accounts.farm.is_paused, StakingError::FarmPaused);
    }
    require!(amount > 0, StakingError::ZeroAmount);

    let farm = &mut ctx.accounts.farm;
    let staker = &mut ctx.accounts.staker;

    require!(staker.flexible_balance >= amount, StakingError::InsufficientBalance);

    // 1. Harvest Rewards
    update_reward_accumulator(farm)?;
    sync_staker_rewards(farm, staker)?;

    // 2. Update State
    staker.flexible_balance = staker.flexible_balance
        .checked_sub(amount)
        .ok_or(StakingError::MathOverflow)?;

    let weight_decrease = amount as u128;

    staker.total_active_weight = staker.total_active_weight
        .checked_sub(weight_decrease)
        .ok_or(StakingError::MathOverflow)?;

    farm.total_weighted_stake = farm.total_weighted_stake
        .checked_sub(weight_decrease)
        .ok_or(StakingError::MathOverflow)?;

    // 3. Fix Debt
    staker.reward_debt = staker.total_active_weight
        .checked_mul(farm.reward_per_token_stored)
        .ok_or(StakingError::MathOverflow)?
        .checked_div(PRECISION)
        .ok_or(StakingError::MathOverflow)?;

    // 4. Transfer Tokens Back
    let seeds = &[
        b"farm".as_ref(),
        farm.lp_mint.as_ref(),
        farm.reward_mint.as_ref(),
        &[farm.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.lp_vault.to_account_info(),
        to: ctx.accounts.user_lp_token_account.to_account_info(),
        authority: farm.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

    token::transfer(cpi_ctx, amount)?;

    Ok(())
}

#[derive(Accounts)]
pub struct UnstakeFlexible<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub farm: Account<'info, Farm>,

    #[account(
        mut,
        has_one = owner,
        has_one = farm
    )]
    pub staker: Account<'info, Staker>,

    #[account(mut, address = farm.lp_vault)]
    pub lp_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_lp_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}