// FILE: programs/dloom_stake/src/instructions/mod.rs

pub mod claim_rewards;
pub mod compound;
pub mod initialize_staker;
pub mod reward_math;
pub mod close_staker;

// Sub-folders
pub mod admin;
pub mod bucket;
pub mod position;

// Export Top Level
pub use claim_rewards::*;
pub use compound::*;
pub use initialize_staker::*;
pub use reward_math::*;
pub use close_staker::*;

// Export Sub-modules
pub use admin::add_lockup_tier::*;
pub use admin::create_farm::*;
pub use admin::fund_farm::*;
pub use admin::set_reward_rate::*;
pub use admin::toggle_pause::*;
pub use admin::emergency_withdraw::*;
pub use admin::set_emergency_mode::*;
pub use admin::emergency_unstake::*;

pub use bucket::stake::*;
pub use bucket::unstake::*;

pub use position::stake::*;
pub use position::unstake::*;