// FILE: programs/dloom_stake/src/instructions/bucket/stake.rs

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::{
    errors::StakingError,
    state::{Farm, Staker},
    instructions::reward_math::{update_reward_accumulator, sync_staker_rewards, PRECISION},
};

pub fn handle_stake_flexible(ctx: Context<StakeFlexible>, amount: u64) -> Result<()> {
    require!(!ctx.accounts.farm.is_paused, StakingError::FarmPaused);
    require!(amount > 0, StakingError::ZeroAmount);

    let farm = &mut ctx.accounts.farm;
    let staker = &mut ctx.accounts.staker;

    update_reward_accumulator(farm)?;
    sync_staker_rewards(farm, staker)?;

    staker.flexible_balance = staker.flexible_balance
        .checked_add(amount)
        .ok_or(StakingError::MathOverflow)?;

    let weight_increase = amount as u128;

    staker.total_active_weight = staker.total_active_weight
        .checked_add(weight_increase)
        .ok_or(StakingError::MathOverflow)?;

    farm.total_weighted_stake = farm.total_weighted_stake
        .checked_add(weight_increase)
        .ok_or(StakingError::MathOverflow)?;

    staker.reward_debt = staker.total_active_weight
        .checked_mul(farm.reward_per_token_stored)
        .ok_or(StakingError::MathOverflow)?
        .checked_div(PRECISION)
        .ok_or(StakingError::MathOverflow)?;

    // 4. Transfer Tokens
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_lp_token_account.to_account_info(),
        to: ctx.accounts.lp_vault.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    msg!("Flexible Stake: {} added. New Balance: {}", amount, staker.flexible_balance);

    Ok(())
}

#[derive(Accounts)]
pub struct StakeFlexible<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub farm: Account<'info, Farm>,

    #[account(
        mut,
        seeds = [b"staker", owner.key().as_ref(), farm.key().as_ref()],
        bump = staker.bump,
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