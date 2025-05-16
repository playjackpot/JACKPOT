use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct CreateHide<'info> {
    #[account(mut)]
    pub game_state: Account<'info, GameState>,
    #[account(
        init,
        payer = player,
        space = 8 + 32 + 16 + 33 + 2 + 8
    )]
    pub hide: Account<'info, Hide>,
    #[account(mut)]
    pub player: Account<'info, Player>,
    #[account(mut)]
    pub nft_mint: Account<'info, Mint>, // PrizePals NFT (Years 1–5)
    #[account(mut)]
    pub sol_profit_wallet: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<CreateHide>, coordinates: (f64, f64)) -> Result<()> {
    let game_state = &mut ctx.accounts.game_state;
    let hide = &mut ctx.accounts.hide;
    let player = &mut ctx.accounts.player;

    // Check if player can hide (3 hides to unlock first seek, then 1:1)
    require!(
        player.hides < 3 || player.seeks >= player.hides - 2,
        ErrorCode::InvalidHideSequence
    );

    // Set hide data
    hide.creator = player.pubkey;
    hide.coordinates = coordinates;
    hide.nft = if game_state.year <= 5 {
        Some(ctx.accounts.nft_mint.key())
    } else {
        None
    };
    hide.treasure_box = if game_state.year > 5 {
        Some(game_state.btc_rewards_wallet >= 1_000_000_000) // 1 BTC in lamports
    } else {
        None
    };
    hide.reward = calculate_sol_reward(game_state.sol_profit_wallet); // 0.001–0.01 SOL

    // Update state
    player.hides += 1;
    if game_state.year <= 5 {
        game_state.nft_count += 1;
    }
    game_state.player_count = game_state.player_count.saturating_add(1);

    // Transfer SOL reward to hider
    let sol_reward = hide.reward;
    **ctx.accounts.sol_profit_wallet.lamports.borrow_mut() -= sol_reward;
    **ctx.accounts.player.to_account_info().lamports.borrow_mut() += sol_reward;

    Ok(())
}

fn calculate_sol_reward(sol_profit_wallet: Pubkey) -> u64 {
    // Dynamic reward based on SOL Profit Wallet balance
    let balance = sol_profit_wallet.lamports();
    if balance > 20_000_000_000_000 {
        10_000_000 // 0.01 SOL
    } else if balance > 10_000_000_000_000 {
        5_000_000  // 0.005 SOL
    } else if balance > 5_000_000_000_000 {
        2_500_000  // 0.0025 SOL
    } else {
        1_000_000  // 0.001 SOL
    }
}
