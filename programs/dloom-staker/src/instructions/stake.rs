// FILE: programs/dloom_stake/src/instructions/stake.rs

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::{
    errors::StakingError,
    events::Staked,
    state::{Farm, Staker},
    instructions::reward_math::{update_reward_accumulator, calculate_pending_rewards},
};

pub fn handle_stake(ctx: Context<UpdateStake>, amount: u64, duration: i64) -> Result<()> {
    require!(amount > 0, StakingError::ZeroAmount);

    let farm = &mut ctx.accounts.farm;
    let staker = &mut ctx.accounts.staker;
    let now = Clock::get()?.unix_timestamp;

    // 1. Update reward accumulator
    update_reward_accumulator(farm)?;
    let pending_rewards = calculate_pending_rewards(farm, staker)?;
    
    staker.rewards_paid = pending_rewards as u128;
    staker.reward_per_token_snapshot = farm.reward_per_token_stored;

    // 2. Remove old weighted stake
    if staker.balance > 0 {
        let old_weighted_stake = (staker.balance as u128)
            .checked_mul(staker.reward_multiplier as u128)
            .ok_or(StakingError::MathOverflow)?
            .checked_div(10000)
            .ok_or(StakingError::MathOverflow)?;
        farm.total_weighted_stake = farm.total_weighted_stake
            .checked_sub(old_weighted_stake)
            .ok_or(StakingError::MathOverflow)?;
    }

    // 3. Validate and Set Lockup
    if staker.balance == 0 {
        // New Stake
        let tier = farm.lockup_tiers.iter()
            .find(|t| t.duration == duration)
            .ok_or(StakingError::TierNotFound)?;
        
        staker.reward_multiplier = tier.multiplier;
        if duration > 0 {
            staker.lockup_end_timestamp = now.checked_add(duration).ok_or(StakingError::MathOverflow)?;
        } else {
            staker.lockup_end_timestamp = 0; // Flexible stake
        }
    } else {
        // Existing Stake Logic...
        require!(
            (staker.lockup_end_timestamp == 0 && duration == 0) || (staker.lockup_end_timestamp > 0 && duration > 0),
            StakingError::CannotChangeLockup
        );
        if staker.lockup_end_timestamp > 0 {
            let new_lockup_end = now.checked_add(duration).ok_or(StakingError::MathOverflow)?;
            if new_lockup_end > staker.lockup_end_timestamp {
                staker.lockup_end_timestamp = new_lockup_end;
            }
        }
    }

    // 4. Update Balance
    staker.balance = staker.balance.checked_add(amount).ok_or(StakingError::MathOverflow)?;

    // DEBUG LOG
    msg!("Staked! New Balance: {}", staker.balance);

    // 5. Add new weighted stake
    let new_weighted_stake = (staker.balance as u128)
        .checked_mul(staker.reward_multiplier as u128)
        .ok_or(StakingError::MathOverflow)?
        .checked_div(10000)
        .ok_or(StakingError::MathOverflow)?;
    farm.total_weighted_stake = farm.total_weighted_stake
        .checked_add(new_weighted_stake)
        .ok_or(StakingError::MathOverflow)?;

    // 6. Transfer tokens
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_lp_token_account.to_account_info(),
        to: ctx.accounts.lp_vault.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    emit!(Staked {
        owner: staker.owner,
        farm_address: farm.key(),
        amount,
        lockup_end_timestamp: staker.lockup_end_timestamp,
        total_staked_balance: staker.balance,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateStake<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"farm", farm.lp_mint.as_ref(), farm.reward_mint.as_ref()],
        bump = farm.bump
    )]
    pub farm: Account<'info, Farm>,
    
    #[account(
        mut, // <--- CRITICAL: Ensures the balance update is saved!
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