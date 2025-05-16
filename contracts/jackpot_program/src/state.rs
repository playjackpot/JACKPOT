use anchor_lang::prelude::*;

#[account]
pub struct GameState {
    pub sol_profit_wallet: Pubkey,      // SOL Profit Wallet
    pub seek_rewards_pool: u64,         // $SEEK balance (150M–200M target)
    pub btc_rewards_wallet: u64,        // WBTC balance for 1 BTC hide
    pub btc_drop_wallet: u64,           // Excess WBTC for airdrops
    pub player_count: u32,              // Active players
    pub nft_count: u32,                 // Issued PrizePals NFTs
    pub year: u32,                      // Current game year
}

#[account]
pub struct Player {
    pub pubkey: Pubkey,                 // Player's public key
    pub hides: u32,                     // Number of hides created
    pub seeks: u32,                     // Number of seeks completed
    pub rank: u32,                      // Leaderboard rank
    pub rewards: RewardData,            // Earned rewards
}

#[account]
pub struct Hide {
    pub creator: Pubkey,                // Hider's public key
    pub coordinates: (f64, f64),        // Geolocation (lat, lng)
    pub nft: Option<Pubkey>,            // PrizePals NFT (Years 1–5)
    pub treasure_box: Option<bool>,     // Treasure box (Year 6+, true if 1 BTC)
    pub reward: u64,                    // SOL reward (0.001–0.01 SOL)
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RewardData {
    pub seek: u64,                      // $SEEK earned
    pub sol: u64,                       // SOL earned
    pub btc: u64,                       // WBTC earned
}
