// FILE: programs/dloom_stake/src/state/staker.rs

use anchor_lang::prelude::*;

#[account]
#[derive(Debug)] // Removed Default, we init manually
pub struct Staker {
    pub bump: u8,
    pub owner: Pubkey,
    pub farm: Pubkey,
    pub total_active_weight: u128, 
    pub reward_debt: u128,         
    pub earned_rewards: u64,       
    pub flexible_balance: u64,     
    pub next_position_id: u64,     
    pub positions: Vec<LockedPosition>, 
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct LockedPosition {
    pub id: u64,
    pub amount: u64,
    pub lock_start_timestamp: i64,
    pub lock_end_timestamp: i64,
    pub multiplier: u16,
    pub weight: u128, 
}

// Custom Init size helper
impl Staker {
    pub const BASE_SIZE: usize = 8 + 1 + 32 + 32 + 16 + 16 + 8 + 8 + 8 + 4;
    pub const POSITION_SIZE: usize = 8 + 8 + 8 + 8 + 2 + 16; 
    pub const MAX_POSITIONS: usize = 50; 
}