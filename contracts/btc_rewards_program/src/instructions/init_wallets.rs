use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct InitWallets<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 32 + 8 + 4
    )]
    pub wallet_state: Account<'info, BTCWalletState>,
    #[account(mut)]
    pub btc_rewards_wallet: Account<'info, TokenAccount>,
    #[account(mut)]
    pub btc_drop_wallet: Account<'info, TokenAccount>,
    #[account(mut)]
    pub sol_profit_wallet: SystemAccount<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitWallets>) -> Result<()> {
    let wallet_state = &mut ctx.accounts.wallet_state;
    wallet_state.btc_rewards_wallet = ctx.accounts.btc_rewards_wallet.key();
    wallet_state.btc_drop_wallet = ctx.accounts.btc_drop_wallet.key();
    wallet_state.sol_profit_wallet = ctx.accounts.sol_profit_wallet.key();
    wallet_state.fees_collected = 0;
    wallet_state.year = 1;
    Ok(())
}
