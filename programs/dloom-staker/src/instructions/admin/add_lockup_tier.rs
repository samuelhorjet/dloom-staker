use crate::{
    errors::StakingError,
    events::LockupTierAdded,
    state::{Farm, LockupTier},
};
use anchor_lang::prelude::*;

pub fn handle_add_lockup_tier(ctx: Context<ManageFarm>, duration: i64, multiplier: u16) -> Result<()> {
    let farm = &mut ctx.accounts.farm;
    let new_tier = LockupTier {
        duration,
        multiplier,
    };

    require!(
        !farm.lockup_tiers.contains(&new_tier),
        StakingError::DuplicateLockupTier
    );

    farm.lockup_tiers.push(new_tier.clone());

    emit!(LockupTierAdded {
        farm_address: farm.key(),
        tier: new_tier,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct ManageFarm<'info> {
    #[account(mut, has_one = authority)]
    pub farm: Account<'info, Farm>,
    pub authority: Signer<'info>,
}