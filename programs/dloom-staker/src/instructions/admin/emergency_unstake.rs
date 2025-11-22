// FILE: programs/dloom_stake/src/instructions/admin/emergency_unstake.rs

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::{
    errors::StakingError,
    state::{Farm, Staker},
};

pub fn handle_emergency_unstake(ctx: Context<EmergencyUnstake>) -> Result<()> {
    let farm = &ctx.accounts.farm;
    let staker = &mut ctx.accounts.staker;

    // 1. Check if Escape Hatch is open
    require!(farm.is_emergency_mode, StakingError::EmergencyModeNotEnabled);

    // 2. Calculate Total Principal (Flexible + All Positions)
    let locked_sum: u64 = staker.positions.iter().map(|p| p.amount).sum();
    
    let total_principal = staker.flexible_balance
        .checked_add(locked_sum)
        .ok_or(StakingError::MathOverflow)?;

    require!(total_principal > 0, StakingError::ZeroAmount);

    staker.flexible_balance = 0;
    staker.positions.clear();
    staker.total_active_weight = 0;
    staker.reward_debt = 0;
    staker.earned_rewards = 0;

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

    token::transfer(cpi_ctx, total_principal)?;

    msg!("Emergency Unstake: {} LP tokens returned to user.", total_principal);

    Ok(())
}

#[derive(Accounts)]
pub struct EmergencyUnstake<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
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

    #[account(mut, address = farm.lp_vault)]
    pub lp_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = user_lp_token_account.mint == farm.lp_mint,
        constraint = user_lp_token_account.owner == owner.key()
    )]
    pub user_lp_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}