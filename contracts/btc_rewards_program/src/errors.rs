use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient BTC balance")]
    InsufficientBTCBalance,
    #[msg("Player not in top 100")]
    NotGoldPlayer,
    #[msg("Player not in 101–200")]
    NotSilverPlayer,
}
