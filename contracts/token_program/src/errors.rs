use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid vesting amount")]
    InvalidVestingAmount,
    #[msg("Invalid vesting duration")]
    InvalidVestingDuration,
    #[msg("Invalid vesting start time")]
    InvalidVestingStart,
    #[msg("No vested tokens available")]
    NoVestedTokensAvailable,
}
