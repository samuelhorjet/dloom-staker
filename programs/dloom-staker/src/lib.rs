// FILE: programs/dloom_stake/src/lib.rs

use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("D67Cj1mwDuJZKJ9DW9MksWAXZQk3hh7nB6AGzcf6hkph");

#[program]
pub mod dloom_staker {
    use super::*;

    // --- ADMIN INSTRUCTIONS ---

    pub fn create_farm(ctx: Context<CreateFarm>) -> Result<()> {
        instructions::admin::create_farm::handle_create_farm(ctx)
    }

    pub fn add_lockup_tier(
        ctx: Context<ManageFarm>,
        duration: i64,
        multiplier: u16,
    ) -> Result<()> {
        instructions::admin::add_lockup_tier::handle_add_lockup_tier(ctx, duration, multiplier)
    }

    pub fn fund_farm(ctx: Context<FundFarm>, amount: u64) -> Result<()> {
        instructions::admin::fund_farm::handle_fund_farm(ctx, amount)
    }

    pub fn set_reward_rate(ctx: Context<ManageFarm>, new_rate: u64) -> Result<()> {
        instructions::admin::set_reward_rate::handle_set_reward_rate(ctx, new_rate)
    }

    pub fn toggle_pause(ctx: Context<TogglePause>) -> Result<()> {
        instructions::admin::toggle_pause::handle_toggle_pause(ctx)
    }

    pub fn emergency_withdraw(ctx: Context<EmergencyWithdraw>, amount: u64) -> Result<()> {
        instructions::admin::emergency_withdraw::handle_emergency_withdraw(ctx, amount)
    }

    pub fn set_emergency_mode(ctx: Context<SetEmergencyMode>, mode: bool) -> Result<()> {
        instructions::admin::set_emergency_mode::handle_set_emergency_mode(ctx, mode)
    }

    pub fn emergency_unstake(ctx: Context<EmergencyUnstake>) -> Result<()> {
        instructions::admin::emergency_unstake::handle_emergency_unstake(ctx)
    }

    // --- USER INSTRUCTIONS (BUCKET / FLEXIBLE) ---

    pub fn initialize_staker(ctx: Context<InitializeStaker>) -> Result<()> {
        instructions::initialize_staker::handle_initialize_staker(ctx)
    }

    pub fn stake_flexible(ctx: Context<StakeFlexible>, amount: u64) -> Result<()> {
        instructions::bucket::stake::handle_stake_flexible(ctx, amount)
    }

    pub fn unstake_flexible(ctx: Context<UnstakeFlexible>, amount: u64) -> Result<()> {
        instructions::bucket::unstake::handle_unstake_flexible(ctx, amount)
    }

    // --- USER INSTRUCTIONS (POSITION / LOCKED) ---

    pub fn stake_locked(ctx: Context<StakeLocked>, amount: u64, duration: i64) -> Result<()> {
        instructions::position::stake::handle_stake_locked(ctx, amount, duration)
    }

    pub fn unstake_locked(ctx: Context<UnstakeLocked>, position_id: u64) -> Result<()> {
        instructions::position::unstake::handle_unstake_locked(ctx, position_id)
    }

    // --- REWARDS & ACCOUNT MANAGEMENT ---

    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        instructions::claim_rewards::handle_claim_rewards(ctx)
    }

    pub fn compound(ctx: Context<Compound>) -> Result<()> {
        instructions::compound::handle_compound(ctx)
    }

    // --- THIS WAS MISSING ---
    pub fn close_staker(ctx: Context<CloseStaker>) -> Result<()> {
        instructions::close_staker::handle_close_staker(ctx)
    }
}