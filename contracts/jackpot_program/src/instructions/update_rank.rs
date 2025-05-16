use anchor_lang::prelude::*;

use crate::state::{GameState, Player};
use crate::errors::*;

#[derive(Accounts)]
pub struct UpdateRank<'info> {
    #[account(mut)]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    pub player: Account<'info, Player>,
}

pub fn handler(ctx: Context<UpdateRank>) -> Result<()> {
    let game_state = &mut ctx.accounts.game_state;
    let player = &mut ctx.accounts.player;

    // Simplified ranking: based on seeks completed
    // In practice, use a more complex metric (e.g., seeks + hides + rewards earned)
    let seeks = player.seeks;

    // Update rank (mock leaderboard logic)
    // Assume off-chain sorting for top 200; here, we assign a temporary rank
    let previous_rank = player.rank;
    let new_rank = calculate_rank(seeks, game_state.player_count);

    player.rank = new_rank;

    // Update game state (track top players)
    if new_rank <= 200 {
        // Store top 200 players in game_state (simplified)
        // In practice, use a separate leaderboard account or off-chain storage
        emit!(RankUpdateEvent {
            player: player.pubkey,
            old_rank: previous_rank,
            new_rank,
        });
    }

    Ok(())
}

fn calculate_rank(seeks: u32, player_count: u32) -> u32 {
    // Mock ranking: top 1% of seeks for Gold, next 1% for Silver
    let percentile = (seeks as f64 / player_count.max(1) as f64) * 100.0;
    if percentile >= 99.0 {
        (seeks % 100) + 1 // Top 100
    } else if percentile >= 98.0 {
        (seeks % 100) + 101 // 101–200
    } else {
        201 // Below top 200
    }
}

#[event]
pub struct RankUpdateEvent {
    pub player: Pubkey,
    pub old_rank: u32,
    pub new_rank: u32,
}
