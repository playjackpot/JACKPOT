use anchor_lang::prelude::*;

use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct ResetAnnualHide<'info> {
    #[account(mut)]
    pub wallet_state: Account<'info, BTCWalletState>,
    #[account(mut)]
    pub authority: Signer<'info>, // Admin key
}

pub fn handler(ctx: Context<ResetAnnualHide>) -> Result<()> {
    let wallet_state = &mut ctx.accounts.wallet_state;

    // Verify admin (simplified)
    require!(is_admin(ctx.accounts.authority.key()), ErrorCode::Unauthorized);

    // Reset annual hide state
    wallet_state.annual_hide_created = false;
    wallet_state.annual_hide = None;
    wallet_state.year += 1; // Increment year

    Ok(())
}

fn is_admin(_pubkey: Pubkey) -> bool {
    // Implement admin check (e.g., hardcoded admin key)
    true // Simplified
}
