use anchor_lang::prelude::*;
use crate::state::LockupTier;

#[event]
pub struct FarmCreated {
    pub farm_address: Pubkey,
    pub authority: Pubkey,
    pub lp_mint: Pubkey,
    pub reward_mint: Pubkey,
}

#[event]
pub struct LockupTierAdded {
    pub farm_address: Pubkey,
    pub tier: LockupTier,
}

#[event]
pub struct FarmFunded {
    pub farm_address: Pubkey,
    pub funder: Pubkey,
    pub amount: u64,
}

#[event]
pub struct RewardRateUpdated {
    pub farm_address: Pubkey,
    pub new_rate: u64,
}

#[event]
pub struct TokensBurned {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
}

#[event]
pub struct StakerInitialized {
    pub staker_address: Pubkey,
    pub farm_address: Pubkey,
    pub owner: Pubkey,
}

#[event]
pub struct Staked {
    pub owner: Pubkey,
    pub farm_address: Pubkey,
    pub amount: u64,
    pub lockup_end_timestamp: i64,
    pub total_staked_balance: u64,
}

#[event]
pub struct Unstaked {
    pub owner: Pubkey,
    pub farm_address: Pubkey,
    pub amount: u64,
    pub total_staked_balance: u64,
}

#[event]
pub struct RewardsClaimed {
    pub owner: Pubkey,
    pub farm_address: Pubkey,
    pub amount: u64,
}

#[event]
pub struct Compounded {
    pub owner: Pubkey,
    pub farm_address: Pubkey,
    pub reward_amount_compounded: u64,
    pub new_total_staked_balance: u64,
}