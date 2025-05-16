use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer};

use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct AirdropBTC<'info> {
    #[account(mut)]
    pub wallet_state: Account<'info, BTCWalletState>,
    #[account(mut)]
    pub btc_drop_wallet: Account<'info, TokenAccount>,
    #[account(mut)]
    pub player_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<AirdropBTC>) -> Result<()> {
    let wallet_state = &ctx.accounts.wallet_state;
    let btc_drop_wallet = &mut ctx.accounts.btc_drop_wallet;

    // Assume player is in top 100 (validate off-chain or via jackpot_program)
    let total_balance = btc_drop_wallet.amount;
    let per_player = total_balance / 100; // Equal split for top 100

    // Transfer WBTC to player
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: btc_drop_wallet.to_account_info(),
                to: ctx.accounts.player_token_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            }
        ),
        per_player
    )?;

    // Reset BTC Drop Wallet annually
    btc_drop_wallet.amount = btc_drop_wallet.amount.saturating_sub(per_player);

    Ok(())
}
