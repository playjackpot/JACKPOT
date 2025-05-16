use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer};

use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct DistributeBTC<'info> {
    #[account(mut)]
    pub wallet_state: Account<'info, BTCWalletState>,
    #[account(mut)]
    pub sol_profit_wallet: SystemAccount<'info>,
    #[account(mut)]
    pub player_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DistributeBTC>) -> Result<()> {
    let wallet_state = &ctx.accounts.wallet_state;

    // Assume player rank is validated (1–100 Gold, 101–200 Silver)
    let sol_balance = ctx.accounts.sol_profit_wallet.lamports();
    let gold_allocation = (sol_balance as f64 * 0.03) as u64; // 3% for Gold
    let silver_allocation = (sol_balance as f64 * 0.02) as u64; // 2% for Silver
    let btc_price_lamports = 1_000_000_000; // Mock 1 BTC in lamports

    let (amount, players) = if is_gold_player() { // Mock validation
        (gold_allocation / btc_price_lamports / 100, 100)
    } else {
        (silver_allocation / btc_price_lamports / 100, 100)
    };

    // Transfer WBTC to player
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.authority.to_account_info(),
                to: ctx.accounts.player_token_account ler_token_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            }
        ),
        amount
    )?;

    // Deduct SOL from SOL Profit Wallet
    let total_sol = amount * btc_price_lamports * players;
    **ctx.accounts.sol_profit_wallet.lamports.borrow_mut() -= total_sol;

    Ok(())
}

fn is_gold_player() -> bool {
    // Implement rank check (simplified)
    true
}
