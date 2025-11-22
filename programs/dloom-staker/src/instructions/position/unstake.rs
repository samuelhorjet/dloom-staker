// FILE: programs/dloom_stake/src/instructions/position/unstake.rs

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::{
    errors::StakingError,
    state::{Farm, Staker},
    instructions::reward_math::{update_reward_accumulator, sync_staker_rewards, PRECISION},
};

pub fn handle_unstake_locked(ctx: Context<UnstakeLocked>, position_id: u64) -> Result<()> {
    let farm = &mut ctx.accounts.farm;
    
    if !farm.is_emergency_mode {
        require!(!farm.is_paused, StakingError::FarmPaused);
    }

    let staker = &mut ctx.accounts.staker;
    let now = Clock::get()?.unix_timestamp;

    let index = staker.positions.iter().position(|p| p.id == position_id)
        .ok_or(StakingError::PositionNotFound)?; 

    let position = staker.positions[index].clone();

    require!(now >= position.lock_end_timestamp, StakingError::StakeLocked);

    update_reward_accumulator(farm)?;
    sync_staker_rewards(farm, staker)?;

    staker.positions.swap_remove(index);

    staker.total_active_weight = staker.total_active_weight
        .checked_sub(position.weight)
        .ok_or(StakingError::MathOverflow)?;

    farm.total_weighted_stake = farm.total_weighted_stake
        .checked_sub(position.weight)
        .ok_or(StakingError::MathOverflow)?;

    staker.reward_debt = staker.total_active_weight
        .checked_mul(farm.reward_per_token_stored)
        .ok_or(StakingError::MathOverflow)?
        .checked_div(PRECISION)
        .ok_or(StakingError::MathOverflow)?;

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

    token::transfer(cpi_ctx, position.amount)?;

    msg!("Position {} Unstaked. Weight removed: {}", position_id, position.weight);

    Ok(())
}

#[derive(Accounts)]
pub struct UnstakeLocked<'info> {
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