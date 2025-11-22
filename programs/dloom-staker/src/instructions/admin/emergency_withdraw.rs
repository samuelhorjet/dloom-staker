// FILE: programs/dloom_stake/src/instructions/emergency_withdraw.rs

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::{state::Farm, errors::StakingError};

pub fn handle_emergency_withdraw(ctx: Context<EmergencyWithdraw>, amount: u64) -> Result<()> {
    let farm = &ctx.accounts.farm;
    
    // --- SECURITY CHECK (Option 2) ---
    // This guarantees users that the Admin cannot "Rug Pull" the LP tokens.
    require!(
        ctx.accounts.vault.key() != farm.lp_vault, 
        StakingError::AdminCannotWithdrawLP
    );
    // ---------------------------------

    let seeds = &[
        b"farm".as_ref(),
        farm.lp_mint.as_ref(),
        farm.reward_mint.as_ref(),
        &[farm.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.admin_token_account.to_account_info(),
        authority: farm.to_account_info(),
    };
    
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

    token::transfer(cpi_ctx, amount)?;
    
    msg!("Emergency withdrawal of {} tokens executed.", amount);

    Ok(())
}

#[derive(Accounts)]
pub struct EmergencyWithdraw<'info> {
    #[account(has_one = authority)]
    pub farm: Account<'info, Farm>,
    
    pub authority: Signer<'info>,

    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub admin_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}