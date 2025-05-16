use anchor_lang::prelude::*;

#[account]
pub struct BTCWalletState {
    pub btc_rewards_wallet: Pubkey,  // WBTC account for 1 BTC hides
    pub btc_drop_wallet: Pubkey,    // WBTC account for excess airdrops
    pub sol_profit_wallet: Pubkey,  // Reference to SOL Profit Wallet
    pub fees_collected: u64,        // Total fees collected
    pub year: u32,                  // Current game year
}
