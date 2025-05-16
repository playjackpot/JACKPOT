use anchor_lang::prelude::*;

#[account]
pub struct BTCWalletState {
    pub btc_rewards_wallet: Pubkey,  // WBTC account for 1 BTC hides
    pub btc_drop_wallet: Pubkey,    // WBTC account for excess airdrops
    pub sol_profit_wallet: Pubkay,  // Reference to SOL Profit Wallet
    pub fees_collected: u64,        // Total fees collected
    pub year: u32,                  // Current game year
    pub annual_hide_created: bool,  // Tracks if annual BTC hide is created for the year
    pub annual_hide: Option<Pubkey>, // Reference to the hide account in jackpot_program
    pub btc_price_usd: u64, // BTC/USD price from Chainlink (8 decimals)
}
