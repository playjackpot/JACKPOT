use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid geolocation coordinates")]
    InvalidLocation,
    #[msg("Player has not completed required hides")]
    InvalidSeekSequence,
    #[msg("Insufficient $SEEK balance")]
    InsufficientSeekBalance,
    #[msg("No NFT provided for hint")]
    NoNFTProvided,
    #[msg("KYC not verified for BTC hide")]
    KYCNotVerified,
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Player ineligible for airdrop")]
    IneligibleForAirdrop,
    #[msg("Airdrop limit reached")]
    AirdropLimitReached,
    #[msg("Microtransactions not available before Year 8")]
    MicrotransactionsNotAvailable,
    #[msg("Invalid or zero price from Chainlink feed")]
    InvalidPriceFeed,
    #[error_code]
    #[msg("Player ineligible for community event")]
    IneligibleForEvent,
}

