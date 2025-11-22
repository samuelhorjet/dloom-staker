// FILE: programs/dloom_stake/src/instructions/toggle_pause.rs

use crate::{
    state::Farm,
    // You might want to emit an event here too
};
use anchor_lang::prelude::*;

pub fn handle_toggle_pause(ctx: Context<TogglePause>) -> Result<()> {
    let farm = &mut ctx.accounts.farm;
    farm.is_paused = !farm.is_paused;
    msg!("Farm paused state: {}", farm.is_paused);
    Ok(())
}

#[derive(Accounts)]
pub struct TogglePause<'info> {
    #[account(mut, has_one = authority)]
    pub farm: Account<'info, Farm>,
    pub authority: Signer<'info>,
}