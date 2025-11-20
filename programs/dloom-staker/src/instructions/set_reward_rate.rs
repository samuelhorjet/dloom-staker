use crate::{
    events::RewardRateUpdated,
    instructions::add_lockup_tier::ManageFarm,
};
use anchor_lang::prelude::*;

pub fn handle_set_reward_rate(ctx: Context<ManageFarm>, new_rate: u64) -> Result<()> {
    let farm = &mut ctx.accounts.farm;
    farm.reward_rate = new_rate;

    emit!(RewardRateUpdated {
        farm_address: farm.key(),
        new_rate,
    });

    Ok(())
}