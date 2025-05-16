use anchor_lang::prelude::*;

use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct InitGame<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 8 + 8 + 4 + 4 + 4
    )]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitGame>) -> Result<()> {
    let game_state = &mut ctx.accounts.game_state;
    game_state.sol_profit_wallet = ctx.accounts.authority.key();
    game_state.seek_rewards_pool = 290_000_000_000_000; // 290M $SEEK (rewards allocation)
    game_state.btc_rewards_wallet = 0;
    game_state.btc_drop_wallet = 0;
    game_state.player_count = 0;
    game_state.nft_count = 0;
    game_state.year = 1;
    Ok(())
}
