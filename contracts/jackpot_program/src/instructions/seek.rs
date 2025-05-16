use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer};

use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct Seek<'info> {
    #[account(mut)]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    pub hide: Account<'info, Hide>,
    #[account(mut)]
    pub player: Account<'info, Player>,
    #[account(mut)]
    pub seek_rewards_pool: Account<'info, TokenAccount>,
    #[account(mut)]
    pub player_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub sol_profit_wallet: SystemAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Seek>, coordinates: (f64, f64)) -> Result<()> {
    let game_state = &mut ctx.accounts.game_state;
    let hide = &ctx.accounts.hide;
    let player = &mut ctx.accounts.player;

    // Validate geolocation (off-chain, assume passed from backend)
    require!(
        is_within_radius(coordinates, hide.coordinates, 100.0),
        ErrorCode::InvalidLocation
    );

    // Charge $SEEK fees
    let start_fee = 2_500_000_000; // 2,500 $SEEK
    let find_fee = if game_state.year >= 8 { 2_000_000 } else { 1_000_000 }; // $1 or $2
    let cpi_accounts = Transfer {
        from: ctx.accounts.player_token_account.to_account_info(),
        to: ctx.accounts.seek_rewards_pool.to_account_info(),
        authority: ctx.accounts.player.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    token::transfer(
        CpiContext::new(cpi_program, cpi_accounts),
        start_fee + find_fee
    )?;

    // Burn 50% of find fee
    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Burn {
                mint: ctx.accounts.player_token_account.mint.to_account_info(),
                to: ctx.accounts.player_token_account.to_account_info(),
                authority: ctx.accounts.player.to_account_info(),
            }
        ),
        find_fee / 2
    )?;

    // Distribute rewards
    let seek_reward = calculate_seek_reward(game_state.seek_rewards_pool);
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.seek_rewards_pool.to_account_info(),
                to: ctx.accounts.player_token_account.to_account_info(),
                authority: ctx.accounts.game_state.to_account_info(),
            }
        ),
        seek_reward
    )?;

    let sol_reward = calculate_sol_reward(ctx.accounts.sol_profit_wallet.key());
    **ctx.accounts.sol_profit_wallet.lamports.borrow_mut() -= sol_reward;
    **ctx.accounts.player.to_account_info().lamports.borrow_mut() += sol_reward;

    // Update state
    player.seeks += 1;
    update_rank(player, game_state)?;

    Ok(())
}

fn calculate_seek_reward(pool_balance: u64) -> u64 {
    if pool_balance >= 200_000_000_000_000 {
        25_000_000_000 // 25,000 $SEEK
    } else if pool_balance >= 150_000_000_000_000 {
        15_000_000_000 // 15,000 $SEEK
    } else if pool_balance >= 50_000_000_000_000 {
        5_000_000_000  // 5,000 $SEEK
    } else {
        2_500_000_000  // 2,500 $SEEK
    }
}

fn is_within_radius(player: (f64, f64), hide: (f64, f64), radius_m: f64) -> bool {
    // Simplified; actual haversine formula should be in backend
    true // Assume validated by backend
}

fn calculate_sol_reward(sol_profit_wallet: Pubkey) -> u64 {
    let balance = sol_profit_wallet.lamports();
    if balance > 20_000_000_000_000 {
        10_000_000
    } else if balance > 10_000_000_000_000 {
        5_000_000
    } else if balance > 5_000_000_000_000 {
        2_500_000
    } else {
        1_000_000
    }
}

fn update_rank(_player: &mut Player, _game_state: &mut GameState) -> Result<()> {
    // Implement leaderboard logic (simplified)
    Ok(())
}
