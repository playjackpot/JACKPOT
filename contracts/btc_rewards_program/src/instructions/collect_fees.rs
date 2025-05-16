use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer};

use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct CollectFees<'info> {
    #[account(mut)]
    pub wallet_state: Account<'info, BTCWalletState>,
    #[account(mut)]
    pub btc_rewards_wallet: Account<'info, TokenAccount>,
    #[account(mut)]
    pub btc_drop_wallet: Account<'info, TokenAccount>,
    #[account(mut)]
    pub sol_profit_wallet: SystemAccount<'info>,
    #[account(mut)]
    pub fee_payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CollectFees>, amount: u64) -> Result<()> {
    let wallet_state = &mut ctx.accounts.wallet_state;

    // Collect 50% of fees
    let fee_amount = amount / 2;
    wallet_state.fees_collected += fee_amount;

    // Collect SOL Profit Wallet contribution (1% Year 3+, increasing)
    let sol_contribution_percent = match wallet_state.year {
        1..=2 => 0.0,
        3..=8 => 0.01,
        9..=14 => 0.02,
        15..=20 => 0.03,
        _ => 0.035, // +0.5% after Year 16
    };
    let sol_balance = ctx.accounts.sol_profit_wallet.lamports();
    let sol_contribution = (sol_balance as f64 * sol_contribution_percent) as u64;

    // Convert to WBTC (simplified, assume oracle for BTC price)
    let btc_price_lamports = 1_000_000_000; // Mock 1 BTC in lamports
    let btc_amount = (fee_amount + sol_contribution) / btc_price_lamports;

    // Cap BTC Rewards Wallet at 1 BTC
    let btc_cap = 1_000_000_000; // 1 BTC in lamports
    let rewards_balance = ctx.accounts.btc_rewards_wallet.amount;
    let new_rewards_amount = rewards_balance.saturating_add(btc_amount);
    let excess = if new_rewards_amount > btc_cap {
        new_rewards_amount - btc_cap
    } else {
        0
    };
    let rewards_to_add = btc_amount - excess;

    // Transfer to BTC Rewards Wallet
    if rewards_to_add > 0 {
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.fee_payer.to_account_info(),
                    to: ctx.accounts.btc_rewards_wallet.to_account_info(),
                    authority: ctx.accounts.fee_payer.to_account_info(),
                }
            ),
            rewards_to_add
        )?;
    }

    // Transfer excess to BTC Drop Wallet
    if excess > 0 {
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.fee_payer.to_account_info(),
                    to: ctx.accounts.btc_drop_wallet.to_account_info(),
                    authority: ctx.accounts.fee_payer.to_account_info(),
                }
            ),
            excess
        )?;
    }

    // Deduct SOL contribution
    **ctx.accounts.sol_profit_wallet.lamports.borrow_mut() -= sol_contribution;

    Ok(())
}
