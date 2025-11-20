use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug)]
pub struct Staker {
    pub bump: u8,
    pub owner: Pubkey,
    pub farm: Pubkey,
    pub balance: u64, 
    pub lockup_end_timestamp: i64,
    pub reward_multiplier: u16,
    pub rewards_paid: u128,
    pub reward_per_token_snapshot: u128,
}