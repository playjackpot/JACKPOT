use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer};

use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct BuybackSeek<'info> {
    #[account(mut)]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    pub seek_rewards_pool: Account<'info, TokenAccount>,
    #[account(mut)]
    pub sol_profit_wallet: SystemAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<BuybackSeek>) -> Result<()> {
    let game_state = &mut ctx.accounts.game_state;

    // Skip if pool is at 200M $SEEK
    if game_state.seek_rewards_pool >= 200_000_000_000_000 {
        return Ok(());
    }

    // Circulating supply buyback (Year 6+)
    if game_state.year >= 6 {
        let circulating_supply = 1_000_000_000_000_000 - game_state.seek_rewards_pool; // Simplified
        let buyback_amount = (circulating_supply as f64 * 0.01) as u64; // 1%
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.sol_profit_wallet.to_account_info(),
                    to: ctx.accounts.seek_rewards_pool.to_account_info(),
                    authority: ctx.accounts.game_state.to_account_info(),
                }
            ),
            buyback_amount
        )?;
        game_state.seek_rewards_pool += buyback_amount;
    }

    // SOL Profit Wallet buyback (Year 8+)
    if game_state.year >= 8 && game_state.seek_rewards_pool < 200_000_000_000_000 {
        let sol_balance = ctx.accounts.sol_profit_wallet.lamports();
        let buyback_sol = (sol_balance as f64 * 0.005) as u64; // 0.5%
        let seek_price = 10_000_000; // $0.01/$SEEK in lamports
        let buyback_amount = buyback_sol / seek_price;
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.sol_profit_wallet.to_account_info(),
                    to: ctx.accounts.seek_rewards_pool.to_account_info(),
                    authority: ctx.accounts.game_state.to_account_info(),
                }
            ),
            buyback_amount
        )?;
        game_state.seek_rewards_pool += buyback_amount;
        **ctx.accounts.sol_profit_wallet.lamports.borrow_mut() -= buyback_sol;
    }

    Ok(())
}
