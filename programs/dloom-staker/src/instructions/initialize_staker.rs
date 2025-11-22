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
    
    // New Fields
    staker.total_active_weight = 0;
    staker.reward_debt = 0;
    staker.earned_rewards = 0;
    
    staker.flexible_balance = 0;
    
    staker.next_position_id = 1; 
    staker.positions = Vec::new(); 

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
        space = Staker::BASE_SIZE,
        seeds = [b"staker", owner.key().as_ref(), farm.key().as_ref()],
        bump
    )]
    pub staker: Account<'info, Staker>,

    pub system_program: Program<'info, System>,
}