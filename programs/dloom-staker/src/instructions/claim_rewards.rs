// FILE: programs/dloom_stake/src/instructions/claim_rewards.rs

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};
use crate::{
    errors::StakingError,
    events::RewardsClaimed,
    state::{Farm, Staker},
    // Import the new sync function used for the Hybrid model
    instructions::reward_math::{update_reward_accumulator, sync_staker_rewards},
};

pub fn handle_claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
    let farm = &mut ctx.accounts.farm;
    let staker = &mut ctx.accounts.staker;

    require!(!farm.is_paused, StakingError::FarmPaused);

    update_reward_accumulator(farm)?;

    sync_staker_rewards(farm, staker)?;

    let rewards_to_claim = staker.earned_rewards;
    require!(rewards_to_claim > 0, StakingError::NoRewardsToClaim);

    staker.earned_rewards = 0;

    let seeds = &[
        b"farm".as_ref(),
        farm.lp_mint.as_ref(),
        farm.reward_mint.as_ref(),
        &[farm.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.reward_vault.to_account_info(),
        to: ctx.accounts.user_reward_token_account.to_account_info(),
        authority: farm.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

    token::transfer(cpi_ctx, rewards_to_claim)?;

    // 6. Emit Event
    emit!(RewardsClaimed {
        owner: staker.owner,
        farm_address: farm.key(),
        amount: rewards_to_claim,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"farm", farm.lp_mint.as_ref(), farm.reward_mint.as_ref()],
        bump = farm.bump
    )]
    pub farm: Account<'info, Farm>,
    
    #[account(address = farm.reward_mint)]
    pub reward_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"staker", owner.key().as_ref(), farm.key().as_ref()],
        bump = staker.bump,
        has_one = owner,
        has_one = farm
    )]
    pub staker: Account<'info, Staker>,

    #[account(
        mut,
        address = farm.reward_vault
    )]
    pub reward_vault: Account<'info, TokenAccount>,
    
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = reward_mint,
        associated_token::authority = owner,
    )]
    pub user_reward_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}