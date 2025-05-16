use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid geolocation coordinates")]
    InvalidLocation,
    #[msg("Player has not completed required hides")]
    InvalidHideSequence,
    #[msg("Insufficient $SEEK balance")]
    InsufficientSeekBalance,
}
