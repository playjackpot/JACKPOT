use anchor_lang::prelude::*;

#[account]
pub struct NFTState {
    pub total_minted: u32,           // Total PrizePals NFTs minted (max 100,000)
    pub airdropped: u32,            // Total airdropped (max 5,000)
}

#[account]
pub struct PrizePalNFT {
    pub owner: Pubkey,              // Current owner
    pub rarity: Rarity,             // Common, Rare, Epic, Legendary
    pub minted_at: i64,             // Timestamp
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum Rarity {
    Common,                         // 70% (70,000)
    Rare,                           // 20% (20,000)
    Epic,                           // 9% (9,000)
    Legendary,                      // 1% (1,000)
}
