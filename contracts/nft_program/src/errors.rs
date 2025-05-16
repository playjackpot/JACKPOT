use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("NFT mint limit reached")]
    MintLimitReached,
    #[msg("Rarity limit reached")]
    RarityLimitReached,
    #[msg("Airdrop limit reached")]
    AirdropLimitReached,
    #[msg("Invalid rarity upgrade")]
    InvalidRarityUpgrade,
    #[msg("Rarity halving not allowed before 2028")]
    InvalidRarityHalvingTime,
}
