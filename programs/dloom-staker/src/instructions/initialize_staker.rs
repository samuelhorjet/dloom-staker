// FILE: programs/dloom_stake/src/instructions/initialize_staker.rs

use anchor_lang::prelude::*;
use crate::{
    state::{Farm, Staker},
    events::StakerInitialized,
};

pub fn handle_initialize_staker(ctx: Context<InitializeStaker>) -> Result<()> {
    let staker = &mut ctx.accounts.staker;
    
    staker.bump = ctx.bumps.staker;
    staker.owner = ctx.accounts.owner.key();
    staker.farm = ctx.accounts.farm.key();
    staker.balance = 0;
    staker.lockup_end_timestamp = 0;
    staker.reward_multiplier = 10000; 
    staker.rewards_paid = 0;
    staker.reward_per_token_snapshot = 0;

    emit!(StakerInitialized {
        staker_address: staker.key(),
        farm_address: staker.farm,
        owner: staker.owner,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeStaker<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    pub farm: Account<'info, Farm>,

    #[account(
        init,
        payer = owner,
        space = 8 + 1 + 32 + 32 + 8 + 8 + 2 + 16 + 16, 
        seeds = [b"staker", owner.key().as_ref(), farm.key().as_ref()],
        bump
    )]
    pub staker: Account<'info, Staker>,

    pub system_program: Program<'info, System>,
}