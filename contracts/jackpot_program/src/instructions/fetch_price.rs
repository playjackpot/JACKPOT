use anchor_lang::prelude::*;
use chainlink_solana as chainlink;

use crate::state::GameState;
use crate::errors::*;

#[derive(Accounts)]
pub struct FetchPrice<'info> {
    #[account(mut)]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: Chainlink price feed account
    pub price_feed: AccountInfo<'info>,
    #[account(address = chainlink::ID)]
    pub chainlink_program: Program<'info, chainlink::Program>,
}

pub fn handler(ctx: Context<FetchPrice>) -> Result<()> {
    let game_state = &mut ctx.accounts.game_state;
    let price_feed = &ctx.accounts.price_feed;

    // Fetch latest price from Chainlink
    let price_data = chainlink::latest_round_data(
        ctx.accounts.chainlink_program.to_account_info(),
        price_feed.clone(),
    )?;

    // Validate price
    require!(price_data.answer > 0, ErrorCode::InvalidPriceFeed);
    let price = price_data.answer as u64; // Price in USD with 8 decimals

    // Store price in game_state (converted to $SEEK lamports)
    game_state.seek_price_usd = price;

    emit!(PriceUpdatedEvent {
        price,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct PriceUpdatedEvent {
    pub price: u64,
    pub timestamp: i64,
}
