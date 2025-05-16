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
use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient BTC balance")]
    InsufficientBTCBalance,
    #[msg("Player not in top 100")]
    NotGoldPlayer,
    #[msg("Player not in 101–200")]
    NotSilverPlayer,
    #[msg("Annual BTC hide already created for this year")]
    AnnualHideAlreadyCreated,
    #[msg("KYC not verified")]
    KYCNotVerified,
    #[msg("Invalid or zero price from Chainlink feed")]
    InvalidPriceFeed,
}
}
