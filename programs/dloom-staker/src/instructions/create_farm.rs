// FILE: programs/dloom_stake/src/instructions/create_farm.rs
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::{
    events::FarmCreated,
    state::{Farm, LockupTier},
};

pub fn handle_create_farm(ctx: Context<CreateFarm>) -> Result<()> {
    let farm = &mut ctx.accounts.farm;
    
    farm.bump = ctx.bumps.farm;
    farm.authority = ctx.accounts.authority.key();
    farm.lp_mint = ctx.accounts.lp_mint.key();
    farm.lp_vault = ctx.accounts.lp_vault.key();
    farm.reward_mint = ctx.accounts.reward_mint.key();
    farm.reward_vault = ctx.accounts.reward_vault.key();
    farm.reward_rate = 0;
    farm.last_update_timestamp = Clock::get()?.unix_timestamp;
    farm.total_weighted_stake = 0;
    farm.reward_per_token_stored = 0;

    // Add a default "Flexible" tier
    farm.lockup_tiers.push(LockupTier {
        duration: 0,
        multiplier: 10000, // 1.0x
    });

    emit!(FarmCreated {
        farm_address: farm.key(),
        authority: farm.authority,
        lp_mint: farm.lp_mint,
        reward_mint: farm.reward_mint,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct CreateFarm<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + 1 + 32 + 32 + 32 + 32 + 32 + 8 + 8 + 16 + 16 + 128, 
        seeds = [b"farm", lp_mint.key().as_ref(), reward_mint.key().as_ref()],
        bump
    )]
    pub farm: Account<'info, Farm>,

    pub lp_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        seeds = [b"lp_vault", farm.key().as_ref()],
        bump,
        token::mint = lp_mint,
        token::authority = farm
    )]
    pub lp_vault: Account<'info, TokenAccount>,

    pub reward_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = reward_mint,
        associated_token::authority = farm
    )]
    pub reward_vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}