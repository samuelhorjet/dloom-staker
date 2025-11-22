// FILE: programs/dloom_stake/src/instructions/set_emergency_mode.rs

use anchor_lang::prelude::*;
use crate::state::Farm;

pub fn handle_set_emergency_mode(ctx: Context<SetEmergencyMode>, mode: bool) -> Result<()> {
    let farm = &mut ctx.accounts.farm;
    farm.is_emergency_mode = mode;
    
    if mode {
        msg!("EMERGENCY MODE ENABLED. Users can now withdraw principal without rewards.");
    } else {
        msg!("Emergency mode disabled.");
    }
    
    Ok(())
}

#[derive(Accounts)]
pub struct SetEmergencyMode<'info> {
    #[account(mut, has_one = authority)]
    pub farm: Account<'info, Farm>,
    pub authority: Signer<'info>,
}