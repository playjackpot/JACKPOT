use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer};

use crate::state::{GameState, Player};
use crate::errors::*;

#[derive(Accounts)]
pub struct AirdropSeek<'info> {
    #[account(mut)]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    pub player: Account<'info, Player>,
    #[account(mut)]
    pub seek_rewards_pool: Account<'info, TokenAccount>,
    #[account(mut)]
    pub player_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority: Signer<'info>, // Admin
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<AirdropSeek>, amount: u64) -> Result<()> {
    let game_state = &mut ctx.accounts.game_state;
    let player = &ctx.accounts.player;

    // Verify admin (simplified)
    require!(is_admin(ctx.accounts.authority.key()), ErrorCode::Unauthorized);

    // Check airdrop eligibility (e.g., player has completed at least 1 seek)
    require!(player.seeks > 0, ErrorCode::IneligibleForAirdrop);

    // Check total airdrop limit (15M $SEEK)
    let total_airdropped = game_state.seek_rewards_pool; // Track separately in practice
    require!(
        total_airdropped + amount <= 15_000_000_000_000_000,
        ErrorCode::AirdropLimitReached
    );

    // Transfer $SEEK from rewards pool
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.seek_rewards_pool.to_account_info(),
                to: ctx.accounts.player_token_account.to_account_info(),
                authority: ctx.accounts.game_state.to_account_info(),
            }
        ),
        amount
    )?;

    // Update state
    game_state.seek_rewards_pool = game_state.seek_rewards_pool.saturating_sub(amount);
    emit!(AirdropEvent {
        player: player.pubkey,
        amount,
    });

    Ok(())
}

fn is_admin(_pubkey: Pubkey) -> bool {
    // Implement admin check (e.g., multisig)
    true // Simplified
}

#[event]
pub struct AirdropEvent {
    pub player: Pubkey,
    pub amount: u64,
}
