use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Burn};

use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct BurnSeek<'info> {
    #[account(mut)]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    pub player_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<BurnSeek>, amount: u64) -> Result<()> {
    let game_state = &ctx.accounts.game_state;
    let burn_percentage = match game_state.year {
        1..=2 => 0.06,
        3..=5 => 0.04,
        6..=11 => 0.01,
        12..=17 => 0.005,
        18..=23 => 0.0025,
        24..=29 => 0.00125,
        30..=35 => 0.000625,
        _ => 0.0003125,
    };
    let burn_amount = (amount as f64 * burn_percentage) as u64;

    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.player_token_account.mint.to_account_info(),
                to: ctx.accounts.player_token_account.to_account_info(),
                authority: ctx.accounts.player_token_account.to_account_info(),
            }
        ),
        burn_amount
    )?;

    Ok(())
}
