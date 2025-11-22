// FILE: programs/dloom_stake/src/errors.rs

use anchor_lang::prelude::*;

#[error_code]
pub enum StakingError {
    #[msg("Cannot unstake, the stake is still locked.")]
    StakeLocked,
    #[msg("The specified lock-up duration is not available for this farm.")]
    TierNotFound,
    #[msg("The lock-up duration cannot be changed while a stake is active.")]
    CannotChangeLockup,
    #[msg("A staker account can only be initialized once.")]
    StakerAlreadyInitialized,
    #[msg("The amount must be greater than zero.")]
    ZeroAmount,
    #[msg("Insufficient balance to perform this action.")]
    InsufficientBalance,
    #[msg("A math operation resulted in an overflow or underflow.")]
    MathOverflow,
    #[msg("The specified reward rate is invalid.")]
    InvalidRewardRate,
    #[msg("This farm does not support compounding because the reward token is not the same as the LP token.")]
    CompoundingNotSupported,
    #[msg("There are no rewards to claim at this time.")]
    NoRewardsToClaim,
    #[msg("This lock-up tier already exists.")]
    DuplicateLockupTier,
    #[msg("The farm is currently paused.")]
    FarmPaused,
    #[msg("Admin cannot withdraw from the LP Vault.")]
    AdminCannotWithdrawLP,
    #[msg("Emergency mode is not enabled.")]
    EmergencyModeNotEnabled,
    #[msg("The specified position could not be found.")]
    PositionNotFound,
    #[msg("You have exceeded the maximum allowed positions.")]
    TooManyPositions,
}
