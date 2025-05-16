use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;

use instructions::*;
use state::*;
use errors::*;

declare_id!("YourBTCProgramIDHere"); // Replace with actual program ID

#[program]
pub mod btc_rewards_program {
    use super::*;

    pub fn init_wallets(ctx: Context<InitWallets>) -> Result<()> {
        instructions::init_wallets::handler(ctx)
    }

    pub fn collect_fees(ctx: Context<CollectFees>, amount: u64) -> Result<()> {
        instructions::collect_fees::handler(ctx, amount)
    }

    pub fn airdrop_btc(ctx: Context<AirdropBTC>) -> Result<()> {
        instructions::airdrop_btc::handler(ctx)
    }

    pub fn distribute_btc(ctx: Context<DistributeBTC>) -> Result<()> {
        instructions::distribute_btc::handler(ctx)
    }
}
