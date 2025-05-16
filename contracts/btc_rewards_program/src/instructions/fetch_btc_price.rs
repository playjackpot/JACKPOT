use anchor_lang::prelude::*;
use chainlink_solana as chainlink;

use crate::state::BTCWalletState;
use crate::errors::*;

#[derive(Accounts)]
pub struct FetchBTCPrice<'info> {
    #[account(mut)]
    pub wallet_state: Account<'info, BTCWalletState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: Chainlink price feed account
    pub price_feed: AccountInfo<'info>,
    #[account(address = chainlink::ID)]
    pub chainlink_program: Program<'info, chainlink::Program>,
}

pub fn handler(ctx: Context<FetchBTCPrice>) -> Result<()> {
    let wallet_state = &mut ctx.accounts.wallet_state;
    let price_feed = &ctx.accounts.price_feed;

    // Fetch latest BTC/USD price
    let price_data = chainlink::latest_round_data(
        ctx.accounts.chainlink_program.to_account_info(),
        price_feed.clone(),
    )?;

    // Validate price
    require!(price_data.answer > 0, ErrorCode::InvalidPriceFeed);
    let price = price_data.answer as u64; // USD with 8 decimals

    // Store price
    wallet_state.btc_price_usd = price;

    emit!(BTCPriceUpdatedEvent {
        price,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct BTCPriceUpdatedEvent {
    pub price: u64,
    pub timestamp: i64,
}
