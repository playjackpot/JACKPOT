use anchor_lang::prelude::*;

#[account]
pub struct GameState {
    pub sol_profit_wallet: Pubkey,
    pub seek_rewards_pool: u64,
    pub btc_rewards_wallet: u64,
    pub btc_drop_wallet: u64,
    pub player_count: u32,
    pub nft_count: u32,
    pub year: u32,
    pub seek_price_usd: u64, // $SEEK/USD price from Chainlink (8 decimals)
}

#[account]
pub struct Player {
    pub pubkey: Pubkey,
    pub hides: u32,
    pub seeks: u32,
    pub rank: u32,
    pub rewards: RewardData,
}

#[account]
pub struct Hide {
    pub creator: Pubkey,
    pub coordinates: (f64, f64),
    pub nft: Option<Pubkey>,
    pub treasure_box: Option<bool>,
    pub reward: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RewardData {
    pub seek: u64,
    pub sol: u64,
    pub btc: u64,
}
