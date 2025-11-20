use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer}; // Cleaned imports
use crate::{
    errors::StakingError,
    events::Unstaked,
    instructions::{
        stake::UpdateStake,
        reward_math::{update_reward_accumulator, calculate_pending_rewards},
    },
};

pub fn handle_unstake(ctx: Context<UpdateStake>, amount: u64) -> Result<()> {
    require!(amount > 0, StakingError::ZeroAmount);

    let farm = &mut ctx.accounts.farm;
    let staker = &mut ctx.accounts.staker;
    let now = Clock::get()?.unix_timestamp;

    require!(now >= staker.lockup_end_timestamp, StakingError::StakeLocked);
    require!(staker.balance >= amount, StakingError::InsufficientBalance);

    update_reward_accumulator(farm)?;
    let pending_rewards = calculate_pending_rewards(farm, staker)?;
    
    // FIX: Cast u64 to u128
    staker.rewards_paid = pending_rewards as u128;
    staker.reward_per_token_snapshot = farm.reward_per_token_stored;

    // Update weighted stake logic
    let old_weighted_stake = (staker.balance as u128)
        .checked_mul(staker.reward_multiplier as u128)
        .ok_or(StakingError::MathOverflow)?
        .checked_div(10000)
        .ok_or(StakingError::MathOverflow)?;
    farm.total_weighted_stake = farm.total_weighted_stake
        .checked_sub(old_weighted_stake)
        .ok_or(StakingError::MathOverflow)?;

    staker.balance = staker.balance.checked_sub(amount).ok_or(StakingError::MathOverflow)?;

    if staker.balance > 0 {
        let new_weighted_stake = (staker.balance as u128)
            .checked_mul(staker.reward_multiplier as u128)
            .ok_or(StakingError::MathOverflow)?
            .checked_div(10000)
            .ok_or(StakingError::MathOverflow)?;
        farm.total_weighted_stake = farm.total_weighted_stake
            .checked_add(new_weighted_stake)
            .ok_or(StakingError::MathOverflow)?;
    } else {
        // Reset Staker if fully unstaked
        staker.lockup_end_timestamp = 0;
        staker.reward_multiplier = 10000; 
    }

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

    emit!(Unstaked {
        owner: staker.owner,
        farm_address: farm.key(),
        amount,
        total_staked_balance: staker.balance,
    });

    Ok(())
}